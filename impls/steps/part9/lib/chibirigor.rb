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

# chibirigor — a minimal Ruby type inferencer (teaching material).
#
# Provided features:
#   Chibirigor.check(source)    => array of diagnostics
#   Chibirigor.annotate(source) => array of each statement's inferred type
module Chibirigor
end
