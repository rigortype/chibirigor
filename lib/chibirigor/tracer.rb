# frozen_string_literal: true

module Chibirigor
  # trace ― 型推論の手順をイベント列として記録し、端末でコマ送り再生する。
  #
  # 仕組み: コア（type_of / eval_statement / Type.union / Dispatch.dispatch）には
  # 手を入れず、Module#prepend でフックを差し込む。Tracer.current（レコーダ）が
  # nil のときフックは即 super するので、check / annotate 等の挙動は変わらない。
  #
  # イベントの種類（= アニメーションの 1 コマ候補）:
  #   :stmt     トップレベルの文の評価開始
  #   :enter    type_of がノードに入った（既定の再生では出さない・--verbose で出す）
  #   :result   ノードの型が確定した
  #   :bind     代入で型環境（scope）に束縛が増えた
  #   :union    Type.union が複数の型を 1 つにまとめた
  #   :dispatch メソッド送信を表引きした（畳み込み / fail-soft もここで見える）
  module Tracer
    class << self
      attr_accessor :current
    end

    # トレース中だけ Tracer.current にレコーダを置く
    def self.with(recorder)
      self.current = recorder
      yield
    ensure
      self.current = nil
    end

    # イベントを貯めるレコーダ。型・スコープは表示用文字列に落として記録する
    # （あとから JSON にも端末にも出せる、自己完結なデータにするため）。
    class Recorder
      attr_reader :events

      def initialize
        @events = []
        @stack = [] # 評価中ノードのラベル（パンくず）
        @nodes = [] # 評価中ノードそのもの（union の位置決めに使う）
        @scope = {} # 直近のスコープのスナップショット
      end

      def stmt(node, index, scope)
        @scope = snapshot(scope)
        emit(:stmt, node, index: index)
      end

      def enter(node, scope)
        @stack.push(label(node))
        @nodes.push(node)
        @scope = snapshot(scope)
        emit(:enter, node, label: label(node))
      end

      def leave(node, type)
        emit(:result, node, label: label(node), node: short(node), type: type.to_s)
        @stack.pop
        @nodes.pop
      end

      def bind(node, type, new_scope)
        @scope = snapshot(new_scope)
        emit(:bind, node, name: node.name.to_s, type: type.to_s)
      end

      def union(inputs, result)
        emit(:union, @nodes.last, inputs: inputs.map(&:to_s), result: result.to_s)
      end

      def dispatch(node, receiver, name, args, result, folded:, fail_soft:, distributed:)
        emit(:dispatch, node,
             receiver: receiver.to_s, method: name.to_s, args: args.map(&:to_s),
             result: result.to_s, folded: folded, fail_soft: fail_soft, distributed: distributed)
      end

      private

      def emit(kind, node, **details)
        location = node&.location
        @events << {
          kind: kind,
          line: location&.start_line, column: location&.start_column,
          end_line: location&.end_line, end_column: location&.end_column,
          depth: @stack.size, stack: @stack.dup,
          scope: @scope,
          **details
        }
      end

      def snapshot(scope)
        scope.instance_variable_get(:@locals).to_h { |name, type| [name.to_s, type.to_s] }
      end

      def label(node)
        case node
        when Prism::CallNode then "#{node.name} の呼び出し"
        when Prism::IfNode then 'if（三項含む）'
        when Prism::LocalVariableReadNode then "変数 #{node.name}"
        when Prism::LocalVariableWriteNode then "代入 #{node.name}"
        else short(node).sub(/Node\z/, '')
        end
      end

      def short(node)
        node.class.name.split('::').last
      end
    end

    # ── フック（prepend で割り込む。レコーダが無ければ即 super） ──────────────

    module TypeOfHook
      def type_of(node, scope, diagnostics)
        return super unless (recorder = Tracer.current)

        recorder.enter(node, scope)
        type = super
        recorder.leave(node, type)
        type
      end

      def eval_statement(node, scope, diagnostics)
        return super unless (recorder = Tracer.current)

        type, new_scope = super
        recorder.bind(node, type, new_scope) if node.is_a?(Prism::LocalVariableWriteNode)
        [type, new_scope]
      end
    end

    module UnionHook
      def union(types)
        result = super
        # 1 個をそのまま返すだけの呼び出しは「まとめ」ではないので記録しない
        Tracer.current&.union(types, result) if types.size >= 2
        result
      end
    end

    module DispatchHook
      def dispatch(receiver_type, name, arg_types, node, diagnostics)
        return super unless (recorder = Tracer.current)

        distributed = receiver_type.is_a?(Type::Union) # Union レシーバはメンバへ分配される
        key = [Dispatch.class_of(receiver_type), name]
        signature = Plugin.registry[key] || Dispatch::METHODS[key]
        result = super
        recorder.dispatch(node, receiver_type, name, arg_types, result,
                          folded: Dispatch::FOLD.key?(key) && const_only?(result),
                          fail_soft: !distributed && signature.nil?,
                          distributed: distributed)
        result
      end

      # Const か「Const だけの Union」（メンバごとに畳めた結果）か
      def const_only?(type)
        type.is_a?(Type::Const) ||
          (type.is_a?(Type::Union) && type.members.all?(Type::Const))
      end
    end

    Chibirigor.singleton_class.prepend(TypeOfHook)
    Type.singleton_class.prepend(UnionHook)
    Dispatch.singleton_class.prepend(DispatchHook)

    # ── 再生（端末アニメーション） ────────────────────────────────────────────

    # 既定の再生では出さないイベント: enter（出入りの「入り」）と、
    # 自明なリテラルの result（1 が Const[1] なのは見ても情報がない）。
    LITERAL_NODES = %w[IntegerNode FloatNode StringNode SymbolNode TrueNode FalseNode NilNode].freeze

    module_function

    def skip_by_default?(event)
      event[:kind] == :enter ||
        (event[:kind] == :result && LITERAL_NODES.include?(event[:node]))
    end

    # イベント列をコマ送り再生する。
    #   delay: 秒数を与えると自動再生。nil なら Enter キーでステップ実行。
    #   verbose: true で enter / リテラルの result も全部出す。
    # out が端末でなければ（パイプ等）、画面クリアせず全コマを順に出力する。
    def play(source, events, delay: nil, verbose: false, out: $stdout)
      frames = verbose ? events : events.reject { |e| skip_by_default?(e) }
      lines = source.lines.map(&:chomp)
      animate = out.respond_to?(:tty?) && out.tty?

      frames.each_with_index do |event, index|
        out.print "\e[H\e[2J" if animate
        draw(out, lines, event, index, frames.size)
        if delay
          sleep delay
        elsif animate && $stdin.tty?
          out.print "\n[Enter] 次へ / [q] 終了 > "
          input = $stdin.gets
          break if input.nil? || input.strip == 'q'
        end
      end
      out.puts "\n── 再生おわり（全 #{frames.size} コマ）──"
    end

    def draw(out, lines, event, index, total)
      bar = '─' * 64
      out.puts "chibirigor trace ─ step #{index + 1}/#{total}"
      out.puts bar
      lines.each_with_index do |text, i|
        out.puts format('%3d  %s', i + 1, highlight(text, i + 1, event))
      end
      out.puts bar
      scope = event[:scope]
      out.puts "型環境  : #{scope.empty? ? '（空）' : scope.map { |k, v| "#{k}: #{v}" }.join('   ')}"
      out.puts "評価中  : #{event[:stack].empty? ? '（トップレベル）' : event[:stack].join(' › ')}"
      out.puts "► #{message(event)}"
    end

    # イベントの範囲（start..end の行・列）を反転表示でハイライトする
    def highlight(text, lineno, event)
      return text unless event[:line] && lineno.between?(event[:line], event[:end_line])

      from = lineno == event[:line] ? event[:column] : 0
      to = lineno == event[:end_line] ? event[:end_column] : text.length
      from = from.clamp(0, text.length)
      to = to.clamp(from, text.length)
      "#{text[0...from]}\e[7m#{text[from...to]}\e[0m#{text[to..]}"
    end

    def message(event)
      case event[:kind]
      when :stmt then "文 #{event[:index]} の評価を始めます"
      when :enter then "#{event[:label]} に入ります"
      when :result then "#{event[:label]} ⇒ #{event[:type]}"
      when :bind then "束縛: #{event[:name]} ← #{event[:type]}（型環境に追加）"
      when :union then "union: #{event[:inputs].join(' , ')} → #{event[:result]}"
      when :dispatch
        note = if event[:distributed] then '（Union をメンバへ分配）'
               elsif event[:folded] then '（定数畳み込み）'
               elsif event[:fail_soft] then '（表に無い → fail-soft で untyped）'
               else '（表の戻り型に丸め）'
               end
        "dispatch: #{event[:receiver]}.#{event[:method]}(#{event[:args].join(', ')}) → #{event[:result]} #{note}"
      else event[:kind].to_s
      end
    end
  end

  module_function

  # ソースを推論しながらイベント列を記録して返す（trace コマンドの本体）。
  # 戻り値は自己完結な Hash の配列（そのまま JSON 化できる）。
  def trace(source)
    program = Prism.parse(source).value
    recorder = Tracer::Recorder.new
    Tracer.with(recorder) do
      scope = Scope.new
      diagnostics = []
      program.statements.body.each_with_index do |stmt, index|
        recorder.stmt(stmt, index + 1, scope)
        _type, scope = eval_statement(stmt, scope, diagnostics)
      end
    end
    recorder.events
  end
end
