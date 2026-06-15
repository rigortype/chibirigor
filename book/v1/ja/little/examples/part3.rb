# frozen_string_literal: true

# Part 3 到達段階の挙動確認。

$LOAD_PATH.unshift File.expand_path('dist/part3/lib', __dir__)
require 'chibirigor'

# region scope_demo
Chibirigor.annotate("x = 1\nx\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
puts Chibirigor.check("x = \"a\"\nx + 1").map { |d| d[:message] }.first
# endregion
