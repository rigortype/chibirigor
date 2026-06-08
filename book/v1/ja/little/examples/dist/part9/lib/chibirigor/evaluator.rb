# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 文を 1 つ評価し、[その文の型, 更新後のスコープ] を返す。
  # スコープを増やすのは代入だけ。それ以外はスコープを変えない。
  def eval_statement(node, scope, diagnostics)
    case node
    when Prism::LocalVariableWriteNode
      type = type_of(node.value, scope, diagnostics)
      [type, scope.with_local(node.name, type)]
    else
      [type_of(node, scope, diagnostics), scope]
    end
  end
end
