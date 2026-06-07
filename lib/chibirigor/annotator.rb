# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 各トップレベル文の推論した型を { line:, type: } の配列で返す。
  # type_of が型を作っているので、それを見せるだけ（合成の副産物）。
  def annotate(source)
    program = Prism.parse(source).value
    program.statements.body.map do |stmt|
      { line: stmt.location.start_line, type: type_of(stmt, []) }
    end
  end
end
