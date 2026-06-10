# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # ソースを型チェックし、見つかった診断の配列を返す。
  # 文ごとにスコープを縫って渡す（代入で型環境が育つ）。
  # 例外で止めず、最後まで読み進める（止まらない・脅かさない）。
  # baseline に「既に呑んだ診断」を渡すと、それらは差し引いて*新規だけ*返す。
  # 照合は「行＋メッセージ」で行う（列・長さは含めない ― 同じ行を編集して桁がズレても
  # baseline が外れないように）。
  # explain: true なら、推論が untyped に倒した地点（fail-soft）を :info 診断として併せて返す
  #   （実 Rigor の `rigor check --explain` の極小版）。
  # unreachable: true なら、証明可能に到達不能な枝を :info 診断として併せて返す（ADR-47 縮小版）。
  # どちらも既定（false）は挙動不変＝本物の型エラーだけを返す。
  def check(source, baseline = [], rbs: nil, explain: false, unreachable: false)
    program = Prism.parse(source).value
    diagnostics = []
    scope = Scope.new
    program.statements.body.each do |stmt|
      _type, scope = eval_statement(stmt, scope, diagnostics)
    end

    # 戻り型照合（opt-in）: rbs: が渡されたときだけ ⇐ を実行する。
    # 宣言がない def・untyped 宣言は黙って通す（gradual 保証）。
    if rbs
      user_sigs = Rbs.load(rbs)
      program.statements.body.each do |node|
        next unless node.is_a?(Prism::DefNode)

        sig = user_sigs.find { |(_klass, meth), _| meth == node.name }&.last
        next unless sig

        # 本体の型を求めるだけ（診断は捨てる ― 本文エラーは既に上で収集済み）
        body_type = method_return_type(node, scope, [])
        check_against(node, sig[:returns], body_type, diagnostics)
      end
    end

    # 付帯イベント（:kind 付き）は本物の型エラーと分けて持つ。
    special, errors = diagnostics.partition { |d| d[:kind] }

    seen = baseline.map { |d| d.slice(:line, :message) }
    result = errors.reject { |d| seen.include?(d.slice(:line, :message)) }

    # 各フラグが立っているときだけ、その種類を併載する（完全に同一のイベントのみ重複排除）。
    result += special.select { |d| d[:kind] == :fail_soft }.uniq if explain
    result += special.select { |d| d[:kind] == :unreachable }.uniq if unreachable
    result
  end
end
