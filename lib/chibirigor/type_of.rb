# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 式（Prism のノード）から型を求める。型チェッカーの心臓。
  # scope は型環境（変数名→型）。わからなければ Dynamic を返す（脅かさない）。
  def type_of(node, scope, diagnostics)
    case node
    when Prism::IntegerNode then Type::Const[node.value]
    when Prism::FloatNode   then Type::Const[node.value]
    when Prism::StringNode  then Type::Const[node.unescaped]
    when Prism::TrueNode    then Type::Const[true]
    when Prism::FalseNode   then Type::Const[false]
    when Prism::NilNode     then Type::Const[nil]
    when Prism::LocalVariableReadNode then scope.local(node.name) || Type::Dynamic.new
    when Prism::HashNode then type_of_hash(node, scope, diagnostics)
    when Prism::ArrayNode then Type::Tuple[node.elements.map { |el| type_of(el, scope, diagnostics) }.freeze]
    when Prism::CallNode then type_of_call(node, scope, diagnostics)
    when Prism::IfNode then type_of_if(node, scope, diagnostics)
    when Prism::ParenthesesNode then type_of_body(node.body, scope, diagnostics)
    when Prism::DefNode then type_of_def(node, scope, diagnostics)
    else Type::Dynamic.new
    end
  end

  # メソッド定義。本体を型チェックし、def 式の値（メソッド名シンボル）を返す。
  def type_of_def(node, scope, diagnostics)
    method_return_type(node, scope, diagnostics) # 本体を型チェック（診断収集）
    Type::Const[node.name]
  end

  # メソッドの戻り型を本体から合成する。仮引数は untyped（本編は引数推論しない）。
  def method_return_type(node, scope, diagnostics)
    body_scope = method_param_names(node).reduce(scope) { |s, name| s.with_local(name, Type::Dynamic.new) }
    type_of_body(node.body, body_scope, diagnostics)
  end

  def method_param_names(node)
    node.parameters&.requireds&.map(&:name) || []
  end

  # ハッシュリテラル → HashShape（symbol キーのみ覚える）。
  def type_of_hash(node, scope, diagnostics)
    fields = {}
    node.elements.each do |assoc|
      next unless assoc.is_a?(Prism::AssocNode) && assoc.key.is_a?(Prism::SymbolNode)

      fields[assoc.key.unescaped.to_sym] = type_of(assoc.value, scope, diagnostics)
    end
    Type::HashShape[fields.freeze]
  end

  # if / 三項演算子。両枝の型をまとめ、枝ごとに型を絞る（ナローイング）。
  def type_of_if(node, scope, diagnostics)
    type_of(node.predicate, scope, diagnostics) # 条件も型チェック（入れ子のエラー検出）

    # 証明可能に到達不能な枝を記録（check(unreachable: true) のときだけ表に出る・既定は無害）。
    if Narrowing.unreachable_branch?(scope, node.predicate, true)
      diagnostics << unreachable_diagnostic(node.statements || node, true)
    end
    then_type = type_of_body(node.statements, Narrowing.narrow(scope, node.predicate, true), diagnostics)

    else_type =
      if node.subsequent
        if Narrowing.unreachable_branch?(scope, node.predicate, false)
          diagnostics << unreachable_diagnostic(node.subsequent.statements || node.subsequent, false)
        end
        type_of_body(node.subsequent.statements, Narrowing.narrow(scope, node.predicate, false), diagnostics)
      else
        Type::Const[nil] # else が無ければ偽のとき nil
      end

    Type.union([then_type, else_type])
  end

  # 到達不能アーム診断（ADR-47 の縮小版）。:info・kind :unreachable で持つ。
  # truthy=true の枝は「条件が必ず偽」、false の枝は「条件が必ず真（else が死ぬ）」。
  def unreachable_diagnostic(node, truthy)
    reason = truthy ? "条件が必ず偽になります" : "条件が必ず真になります"
    diagnostic(node, "この枝には到達しません（#{reason}）").merge(kind: :unreachable, severity: :info)
  end

  # dump_type(式) の型印字（:info・kind :dump_type）。値素通しなので型エラーではない。
  # check がフラグなしで常に併載する基本機能（実 Rigor の Rigor::Testing.dump_type に相当）。
  def dump_type_diagnostic(node, type)
    diagnostic(node, "dump_type: #{type}").merge(kind: :dump_type, severity: :info)
  end

  # 枝（文の並び）を評価し、最後の文の型を返す。枝の中でもスコープを縫う。
  def type_of_body(statements_node, scope, diagnostics)
    return Type::Const[nil] if statements_node.nil?

    last = Type::Const[nil]
    statements_node.body.each { |stmt| last, scope = eval_statement(stmt, scope, diagnostics) }
    last
  end

  # メソッド送信。レシーバと各引数の型を求め、ディスパッチ表に委ねる。
  # （Part 1 の `+` 場当たり特別扱いを、Part 2 で手書きの表に一般化した。）
  def type_of_call(node, scope, diagnostics)
    # dump_type(式) ― その位置の推論型を :info で印字する基本機能（実 Rigor の
    # Rigor::Testing.dump_type 相当）。実行時は値をそのまま返すので、型も引数の型を返す。
    if node.receiver.nil? && node.name == :dump_type && (node.arguments&.arguments || []).size == 1
      t = type_of(node.arguments.arguments.first, scope, diagnostics)
      diagnostics << dump_type_diagnostic(node, t)
      return t
    end

    receiver = node.receiver ? type_of(node.receiver, scope, diagnostics) : Type::Dynamic.new
    arg_nodes = node.arguments&.arguments || []

    # ブロック付き反復（generics 5b/5c）：既知の配列なら要素型をブロック仮引数へ押し下げる。
    if node.block.is_a?(Prism::BlockNode)
      blocked = type_of_block(receiver, node.name, node.block, scope, diagnostics)
      return blocked if blocked
    end

    # 構造的な型の添字読み（h[:k] / a[0]）はリテラルのキー/添字だけ特別扱い。
    if node.name == :[] && arg_nodes.size == 1
      indexed = read_index(receiver, arg_nodes.first)
      return indexed if indexed
    end

    # 要素型の読み（generics 5a）：既知の配列／ハッシュから要素型 Elem を読む。
    # arr.first / arr.last / 非リテラル添字 arr[i] → 要素型、h.values / h.keys → 値・キー型。
    element = element_read(receiver, node.name, arg_nodes)
    return element if element

    arg_types = arg_nodes.map { |arg| type_of(arg, scope, diagnostics) }
    Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
  end

  # 要素型の読み（generics 5a）。読めなければ nil（通常ディスパッチに回す）。
  # Tuple/Array[Elem]/HashShape という*既知の形*からだけ要素型を取り出す。生 Dynamic の
  # 受信はここで拾わず untyped に倒す（埋まらねば untyped＝誤検知ゼロ）。
  def element_read(receiver, name, arg_nodes)
    if (elem = element_type_of(receiver))
      case name
      when :first, :last then elem if arg_nodes.empty?
      when :[] then elem if arg_nodes.size == 1
      end
    elsif receiver.is_a?(Type::HashShape)
      case name
      when :values then hash_value_type(receiver) if arg_nodes.empty?
      when :keys then hash_key_type(receiver) if arg_nodes.empty?
      end
    end
  end

  # 配列の要素型 Elem。Tuple は全要素を寄せ集め（リテラル精度は class に丸める）、
  # Array[Elem] はその型引数。空配列は要素が分からないので untyped（FP 安全）。
  # 配列でなければ nil（＝「要素を読める形」ではない）。
  def element_type_of(receiver)
    case receiver
    when Type::Tuple then Type.union(receiver.elements.map { |t| widen_element(t) })
    when Type::Generic then receiver.args.first if receiver.name == :Array
    end
  end

  def hash_value_type(shape)
    Type.union(shape.fields.values.map { |t| widen_element(t) })
  end

  # symbol キーのみ覚える（Part 5）ので、キー型は Symbol。空なら untyped。
  def hash_key_type(shape)
    shape.fields.empty? ? Type::Dynamic.new : Type::Nominal[:Symbol]
  end

  # 要素型では「この値そのもの（Const）」をクラスに広げる（`[1,2].first` は `1` でなく Integer）。
  def widen_element(type)
    type.is_a?(Type::Const) ? Type::Nominal[Dispatch.class_of(type)] : type
  end

  # 第1仮引数＝要素という関係が確実な反復子だけを扱う（FP 安全）。
  # each_with_index（|x, i|）や reduce（|acc, x|）は仮引数の意味が違うので含めない。
  ELEMENT_ITERATORS = %i[map collect each select filter reject find_all].freeze

  # ブロック付き反復の型付け（generics 5b/5c）。既知の配列の要素型 Elem をブロック仮引数へ
  # 押し下げ、本体を型チェックする。配列でない／未知の反復子なら nil（通常ディスパッチへ）。
  #   map/collect → Array[本体の型]（5c の戻り多相）／each → レシーバ（self を返す）／
  #   select/filter/reject/find_all → Array[Elem]（要素型は不変）。
  def type_of_block(receiver, name, block, scope, diagnostics)
    return nil unless ELEMENT_ITERATORS.include?(name)

    elem = element_type_of(receiver)
    return nil if elem.nil? # 既知の配列でなければ手を出さない（untyped に倒す＝FP なし）

    body_type = type_of_body(block.body, bind_block_params(block, elem, scope), diagnostics)

    case name
    when :map, :collect then Type::Generic[:Array, [widen_element(body_type)].freeze]
    when :each then receiver
    else Type::Generic[:Array, [elem].freeze]
    end
  end

  # ブロック仮引数を束縛したスコープを返す。第1仮引数＝要素型 Elem、以降は安全側で untyped。
  def bind_block_params(block, elem, scope)
    block_param_names(block).each_with_index.reduce(scope) do |s, (name, index)|
      s.with_local(name, index.zero? ? elem : Type::Dynamic.new)
    end
  end

  def block_param_names(block)
    block.parameters&.parameters&.requireds&.map(&:name) || []
  end

  # 構造的な型からの読み出し。読めなければ nil（通常ディスパッチに回す）。
  def read_index(receiver, arg_node)
    if receiver.is_a?(Type::HashShape) && arg_node.is_a?(Prism::SymbolNode)
      # 未知キーは nil（実 Ruby が nil を返すから。エラーにしない）
      return receiver.fields.fetch(arg_node.unescaped.to_sym, Type::Const[nil])
    end
    if receiver.is_a?(Type::Tuple) && arg_node.is_a?(Prism::IntegerNode)
      return receiver.elements.fetch(arg_node.value, Type::Const[nil])
    end

    nil
  end

  # ⇐ subsumption（照合モード）: expected 型に actual を照合する。
  # :no のときだけ診断を出す。untyped が絡んだら黙る（gradual の約束）。
  # `⇐` が診断を出す最初の口。呼び出し引数照合（dispatch）も同じ原則で動いている。
  def check_against(node, expected, actual, diagnostics)
    return if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)
    return unless Accepts.call(expected, actual) == :no

    diagnostics << diagnostic(node, "戻り型 #{expected} が宣言されていますが #{actual} を返します")
  end

  # 診断は「どこの・何が問題か」。位置（行・列・長さ）はキャレット表示に使う。
  def diagnostic(node, message)
    location = node.location
    {
      line: location.start_line,
      column: location.start_column,
      length: location.length,
      message: message
    }
  end
end
