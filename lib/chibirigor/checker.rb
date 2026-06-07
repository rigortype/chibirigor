# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # ソースを型チェックし、見つかった診断の配列を返す。
  # 文ごとにスコープを縫って渡す（代入で型環境が育つ）。
  # 例外で止めず、最後まで読み進める（止まらない・脅かさない）。
  # baseline に「既に呑んだ診断」を渡すと、それらは差し引いて*新規だけ*返す。
  def check(source, baseline = [])
    program = Prism.parse(source).value
    diagnostics = []
    scope = Scope.new
    program.statements.body.each do |stmt|
      _type, scope = eval_statement(stmt, scope, diagnostics)
    end
    diagnostics.reject { |diagnostic| baseline.include?(diagnostic) }
  end
end
