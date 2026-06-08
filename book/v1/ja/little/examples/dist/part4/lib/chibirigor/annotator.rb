# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 各トップレベル文の推論結果を { line:, type: } の配列で返す。
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
