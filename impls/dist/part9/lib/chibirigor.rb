# frozen_string_literal: true

require 'prism'

require_relative 'chibirigor/version'
require_relative 'chibirigor/type'
require_relative 'chibirigor/scope'
require_relative 'chibirigor/rbs'
require_relative 'chibirigor/plugin'
require_relative 'chibirigor/dispatch'
require_relative 'chibirigor/accepts'
require_relative 'chibirigor/narrowing'
require_relative 'chibirigor/type_of'
require_relative 'chibirigor/evaluator'
require_relative 'chibirigor/checker'
require_relative 'chibirigor/annotator'

# chibirigor ― 最小限の Ruby 型推論器（教材）
#
# 提供機能:
#   Chibirigor.check(source)    => 診断の配列
#   Chibirigor.annotate(source) => 各文の推論型の配列
module Chibirigor
end
