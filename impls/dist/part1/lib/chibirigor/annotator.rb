# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 各文の推論型を { line:, type: } の配列で返す（診断は捨てる）。
  def annotate(source)
    program = Prism.parse(source).value
    program.statements.body.map do |stmt|
      type = type_of(stmt, []) # 文句は今は捨てる
      { line: stmt.location.start_line, type: type }
    end
  end
end
