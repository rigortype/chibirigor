# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 式（Prism のノード）から型を求める。型検査器の心臓。
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
    else Type::Dynamic.new
    end
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
    type_of(node.predicate, scope, diagnostics) # 条件も型検査（入れ子のエラー検出）

    then_type = type_of_body(node.statements, Narrowing.narrow(scope, node.predicate, true), diagnostics)
    else_type =
      if node.subsequent
        type_of_body(node.subsequent.statements, Narrowing.narrow(scope, node.predicate, false), diagnostics)
      else
        Type::Const[nil] # else が無ければ偽のとき nil
      end

    Type.union([then_type, else_type])
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

    arg_types = arg_nodes.map { |arg| type_of(arg, scope, diagnostics) }
    Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
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

  def diagnostic(node, message)
    { line: node.location.start_line, message: message }
  end
end
