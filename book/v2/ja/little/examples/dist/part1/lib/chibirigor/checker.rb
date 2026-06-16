# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Type-check the source and return the array of diagnostics found (don't halt, don't frighten).
  def check(source)
    program = Prism.parse(source).value
    diagnostics = []
    program.statements.body.each { |stmt| type_of(stmt, diagnostics) }
    diagnostics
  end
end
