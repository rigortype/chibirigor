# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Type-check the source and return the array of diagnostics found.
  # Run return-type checking (⇐) only when rbs: is passed.
  def check(source, baseline = [], rbs: nil)
    program = Prism.parse(source).value
    diagnostics = []
    scope = Scope.new
    program.statements.body.each do |stmt|
      _type, scope = eval_statement(stmt, scope, diagnostics)
    end

    if rbs
      user_sigs = Rbs.load(rbs)
      program.statements.body.each do |node|
        next unless node.is_a?(Prism::DefNode)

        sig = user_sigs.find { |(_klass, meth), _| meth == node.name }&.last
        next unless sig

        body_type = method_return_type(node, scope, [])
        check_against(node, sig[:returns], body_type, diagnostics)
      end
    end

    seen = baseline.map { |d| d.slice(:line, :message) }
    diagnostics.reject { |d| seen.include?(d.slice(:line, :message)) }
  end

end
