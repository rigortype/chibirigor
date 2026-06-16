# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Find a type from an expression (an AST node). Unknown nodes fall back to untyped (don't frighten).
  def type_of(node, diagnostics)
    case node
    when Prism::IntegerNode then Const[node.value]
    when Prism::FloatNode   then Const[node.value]
    when Prism::StringNode  then Const[node.unescaped]
    when Prism::TrueNode    then Const[true]
    when Prism::FalseNode   then Const[false]
    when Prism::CallNode    then type_of_call(node, diagnostics)
    else
      Dynamic.new # unknown nodes "don't frighten" — quietly return untyped
    end
  end

  def type_of_call(node, diagnostics)
    recv = type_of(node.receiver, diagnostics)
    args = node.arguments&.arguments || []

    if node.name == :+ && integerish?(recv)
      arg = type_of(args.first, diagnostics)
      unless integerish?(arg)
        diagnostics << diagnostic(node, "can't add #{arg} to an integer")
        return Dynamic.new
      end
      # ★ The key point: don't compute Const[3] — "round" to Integer
      return Nominal[:Integer]
    end

    Dynamic.new # other methods aren't known yet → don't frighten
  end

  def integerish?(t)
    (t.is_a?(Const) && t.value.is_a?(Integer)) || t == Nominal[:Integer]
  end

  # A diagnostic is a small hash holding "which line, what's wrong"
  def diagnostic(node, message)
    { line: node.location.start_line, message: message }
  end
end
