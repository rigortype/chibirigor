# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Find a type from an expression (a Prism node). The heart of the type checker.
  # scope is the type environment (variable name → type). If unknown, return Dynamic (don't frighten).
  def type_of(node, scope, diagnostics)
    case node
    when Prism::IntegerNode then Type::Const[node.value]
    when Prism::FloatNode   then Type::Const[node.value]
    when Prism::StringNode  then Type::Const[node.unescaped]
    when Prism::TrueNode    then Type::Const[true]
    when Prism::FalseNode   then Type::Const[false]
    when Prism::NilNode     then Type::Const[nil]
    when Prism::LocalVariableReadNode then scope.local(node.name) || Type::Dynamic.new
    when Prism::CallNode    then type_of_call(node, scope, diagnostics)
    else Type::Dynamic.new
    end
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
