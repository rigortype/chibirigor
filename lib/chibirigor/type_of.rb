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
    receiver = node.receiver ? type_of(node.receiver, scope, diagnostics) : Type::Dynamic.new
    arg_nodes = node.arguments&.arguments || []

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
  # Tuple/HashShape という*既知の形*からだけ要素型を取り出す。生 Dynamic の受信は
  # ここで拾わず untyped に倒す（埋まらねば untyped＝誤検知ゼロ）。
  def element_read(receiver, name, arg_nodes)
    case receiver
    when Type::Tuple
      case name
      when :first, :last then array_element_type(receiver) if arg_nodes.empty?
      when :[] then array_element_type(receiver) if arg_nodes.size == 1
      end
    when Type::HashShape
      case name
      when :values then hash_value_type(receiver) if arg_nodes.empty?
      when :keys then hash_key_type(receiver) if arg_nodes.empty?
      end
    end
  end

  # 配列の要素型 Elem ＝ 全要素の型を寄せ集めて 1 つに（リテラル精度は class に丸める）。
  # 空配列は要素が分からないので untyped（FP 安全）。
  def array_element_type(tuple)
    Type.union(tuple.elements.map { |t| widen_element(t) })
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
