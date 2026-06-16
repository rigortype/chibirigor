# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # Evaluate one statement and return [the statement's type, the updated scope].
  # Only assignments grow the scope; everything else leaves it unchanged.
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
