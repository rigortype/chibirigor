# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  def type_of(node, scope, diagnostics)
    case node
    when Prism::IntegerNode then Type::Const[node.value]
    when Prism::FloatNode   then Type::Const[node.value]
    when Prism::StringNode  then Type::Const[node.unescaped]
    when Prism::TrueNode    then Type::Const[true]
    when Prism::FalseNode   then Type::Const[false]
    when Prism::NilNode     then Type::Const[nil]
    when Prism::LocalVariableReadNode then scope.local(node.name) || Type::Dynamic.new
    when Prism::CallNode              then type_of_call(node, scope, diagnostics)
    when Prism::IfNode                then type_of_if(node, scope, diagnostics)
    when Prism::ParenthesesNode       then type_of_body(node.body, scope, diagnostics)
    else Type::Dynamic.new
    end
  end

  # if / 三項演算子。両枝の型を Union にまとめる（ナローイングはまだない）。
  def type_of_if(node, scope, diagnostics)
    type_of(node.predicate, scope, diagnostics)

    then_type = type_of_body(node.statements, scope, diagnostics)
    else_type =
      if node.subsequent
        type_of_body(node.subsequent.statements, scope, diagnostics)
      else
        Type::Const[nil]
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

  def type_of_call(node, scope, diagnostics)
    receiver = node.receiver ? type_of(node.receiver, scope, diagnostics) : Type::Dynamic.new
    arg_types = (node.arguments&.arguments || []).map { |arg| type_of(arg, scope, diagnostics) }
    Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
  end

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
