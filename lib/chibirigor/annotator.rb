# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 各トップレベル文の推論結果を { line:, type: } の配列で返す。
  # メソッド定義は RBS 風シグネチャ、それ以外は推論した型。診断は捨てる。
  def annotate(source)
    program = Prism.parse(source).value
    scope = Scope.new
    ignored = []
    program.statements.body.map do |stmt|
      if stmt.is_a?(Prism::DefNode)
        { line: stmt.location.start_line, type: method_signature(stmt, scope, ignored) }
      else
        type, scope = eval_statement(stmt, scope, ignored)
        { line: stmt.location.start_line, type: type }
      end
    end
  end

  # 推論したシグネチャを RBS 風に。引数は untyped、戻りは本体から合成。
  # 戻りが untyped なら「推論が型を見失った場所」が見える（sig-gen の芽）。
  def method_signature(node, scope, diagnostics)
    params = method_param_names(node).map { 'untyped' }.join(', ')
    "def #{node.name}: (#{params}) -> #{method_return_type(node, scope, diagnostics)}"
  end
end
