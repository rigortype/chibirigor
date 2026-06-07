# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 各トップレベル文の推論した型を { line:, type: } の配列で返す。
  # 文ごとにスコープを縫う（代入後はその変数の型が見える）。診断は捨てる。
  def annotate(source)
    program = Prism.parse(source).value
    scope = Scope.new
    ignored = []
    program.statements.body.map do |stmt|
      type, scope = eval_statement(stmt, scope, ignored)
      { line: stmt.location.start_line, type: type }
    end
  end
end
