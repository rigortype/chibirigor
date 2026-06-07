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
    when Prism::CallNode then type_of_call(node, scope, diagnostics)
    when Prism::IfNode then type_of_if(node, scope, diagnostics)
    else Type::Dynamic.new
    end
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
    arg_types = (node.arguments&.arguments || []).map { |arg| type_of(arg, scope, diagnostics) }
    Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
  end

  def diagnostic(node, message)
    { line: node.location.start_line, message: message }
  end
end
