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
    when Prism::HashNode              then type_of_hash(node, scope, diagnostics)
    when Prism::ArrayNode             then Type::Tuple[node.elements.map { |el| type_of(el, scope, diagnostics) }.freeze]
    when Prism::CallNode              then type_of_call(node, scope, diagnostics)
    when Prism::IfNode                then type_of_if(node, scope, diagnostics)
    when Prism::ParenthesesNode       then type_of_body(node.body, scope, diagnostics)
    when Prism::DefNode               then type_of_def(node, scope, diagnostics)
    else Type::Dynamic.new
    end
  end

  # Method definition. Type-check the body and return the def expression's value (the method-name symbol).
  def type_of_def(node, scope, diagnostics)
    method_return_type(node, scope, diagnostics)
    Type::Const[node.name]
  end

  # Synthesize a method's return type from its body. Params are untyped.
  def method_return_type(node, scope, diagnostics)
    body_scope = method_param_names(node).reduce(scope) { |s, name| s.with_local(name, Type::Dynamic.new) }
    type_of_body(node.body, body_scope, diagnostics)
  end

  def method_param_names(node)
    node.parameters&.requireds&.map(&:name) || []
  end

  def type_of_hash(node, scope, diagnostics)
    fields = {}
    node.elements.each do |assoc|
      next unless assoc.is_a?(Prism::AssocNode) && assoc.key.is_a?(Prism::SymbolNode)

      fields[assoc.key.unescaped.to_sym] = type_of(assoc.value, scope, diagnostics)
    end
    Type::HashShape[fields.freeze]
  end

  def type_of_if(node, scope, diagnostics)
    type_of(node.predicate, scope, diagnostics)

    then_type = type_of_body(node.statements, Narrowing.narrow(scope, node.predicate, true), diagnostics)
    else_type =
      if node.subsequent
        type_of_body(node.subsequent.statements, Narrowing.narrow(scope, node.predicate, false), diagnostics)
      else
        Type::Const[nil]
      end

    Type.union([then_type, else_type])
  end

  def type_of_body(statements_node, scope, diagnostics)
    return Type::Const[nil] if statements_node.nil?

    last = Type::Const[nil]
    statements_node.body.each { |stmt| last, scope = eval_statement(stmt, scope, diagnostics) }
    last
  end

  def type_of_call(node, scope, diagnostics)
    receiver = node.receiver ? type_of(node.receiver, scope, diagnostics) : Type::Dynamic.new
    arg_nodes = node.arguments&.arguments || []

    if node.name == :[] && arg_nodes.size == 1
      indexed = read_index(receiver, arg_nodes.first)
      return indexed if indexed
    end

    arg_types = arg_nodes.map { |arg| type_of(arg, scope, diagnostics) }
    Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
  end

  def read_index(receiver, arg_node)
    if receiver.is_a?(Type::HashShape) && arg_node.is_a?(Prism::SymbolNode)
      return receiver.fields.fetch(arg_node.unescaped.to_sym, Type::Const[nil])
    end
    if receiver.is_a?(Type::Tuple) && arg_node.is_a?(Prism::IntegerNode)
      return receiver.elements.fetch(arg_node.value, Type::Const[nil])
    end

    nil
  end

  # ⇐ subsumption (checking mode): check actual against the expected type.
  def check_against(node, expected, actual, diagnostics)
    return if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)
    return unless Accepts.call(expected, actual) == :no

    diagnostics << diagnostic(node, "return type #{expected} is declared but #{actual} is returned")
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
