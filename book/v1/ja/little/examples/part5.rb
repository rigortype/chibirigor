# frozen_string_literal: true

# Part 5 milestone behavior check.

$LOAD_PATH.unshift File.expand_path('dist/part5/lib', __dir__)
require 'chibirigor'

# region narrowing_demo
no_errors = Chibirigor.check("x = c ? 1 : nil\nif x.nil?\n  0\nelse\n  x + 1\nend\n")
puts "nil? narrowing: #{no_errors.empty? ? "OK (no errors)" : "NG"}"
puts Chibirigor.check("x = c ? 1 : \"a\"\nif x.is_a?(String)\n  x + 1\nend\n").map { |d| d[:message] }.first
# endregion
