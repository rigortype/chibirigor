# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Replaces Part 1: carriers move to the `Type::` namespace, calls delegate to Dispatch.
  def type_of(node, diagnostics)
    case node
    when Prism::IntegerNode then Type::Const[node.value]
    when Prism::FloatNode   then Type::Const[node.value]
    when Prism::StringNode  then Type::Const[node.unescaped]
    when Prism::TrueNode    then Type::Const[true]
    when Prism::FalseNode   then Type::Const[false]
    when Prism::CallNode    then type_of_call(node, diagnostics)
    else
      Type::Dynamic.new
    end
  end

  def type_of_call(node, diagnostics)
    receiver = node.receiver ? type_of(node.receiver, diagnostics) : Type::Dynamic.new
    arg_types = (node.arguments&.arguments || []).map { |arg| type_of(arg, diagnostics) }
    Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
  end

  def diagnostic(node, message)
    { line: node.location.start_line, message: message }
  end
end
