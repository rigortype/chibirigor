# frozen_string_literal: true

# Part 7 milestone behavior check.

$LOAD_PATH.unshift File.expand_path('dist/part7/lib', __dir__)
require 'chibirigor'

# region accepts_demo
no_errors = Chibirigor.check("x = c ? 1 : 2\n1 + x")
puts "Integer | Integer: #{no_errors.empty? ? "OK (no errors)" : "NG"}"
puts Chibirigor.check("x = c ? 1 : \"a\"\n1 + x").map { |d| d[:message] }.first
# endregion
