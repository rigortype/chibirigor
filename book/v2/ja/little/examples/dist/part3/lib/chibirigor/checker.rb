# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Type-check the source and return the array of diagnostics found.
  # Thread the scope through statement by statement (assignments grow the type environment).
  # Don't stop on exceptions; read all the way through (don't halt, don't frighten).
  def check(source)
    program = Prism.parse(source).value
    diagnostics = []
    scope = Scope.new
    program.statements.body.each do |stmt|
      _type, scope = eval_statement(stmt, scope, diagnostics)
    end
    diagnostics
  end
end
