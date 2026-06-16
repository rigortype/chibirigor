# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Return each statement's inferred type as an array of { line:, type: } (diagnostics discarded).
  def annotate(source)
    program = Prism.parse(source).value
    program.statements.body.map do |stmt|
      type = type_of(stmt, []) # discard diagnostics for now
      { line: stmt.location.start_line, type: type }
    end
  end
end
