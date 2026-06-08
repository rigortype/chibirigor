# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # ソースを型チェックし、見つかった診断の配列を返す（止まらない・脅かさない）。
  def check(source)
    program = Prism.parse(source).value
    diagnostics = []
    program.statements.body.each { |stmt| type_of(stmt, diagnostics) }
    diagnostics
  end
end
