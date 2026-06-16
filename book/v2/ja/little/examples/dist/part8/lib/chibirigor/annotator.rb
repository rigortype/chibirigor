# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Return each top-level statement's inferred result as an array of { line:, type: }.
  # Method definitions get an RBS-style signature, everything else the inferred type.
  def annotate(source)
    program = Prism.parse(source).value
    scope = Scope.new
    ignored = []
    program.statements.body.map do |stmt|
      if stmt.is_a?(Prism::DefNode)
        { line: stmt.location.start_line, type: method_signature(stmt, scope, ignored) }
      else
        type, scope = eval_statement(stmt, scope, ignored)
        { line: stmt.location.start_line, type: type }
      end
    end
  end

  # Render the inferred signature RBS-style.
  def method_signature(node, scope, diagnostics)
    params = method_param_names(node).map { 'untyped' }.join(', ')
    "def #{node.name}: (#{params}) -> #{method_return_type(node, scope, diagnostics)}"
  end
end
