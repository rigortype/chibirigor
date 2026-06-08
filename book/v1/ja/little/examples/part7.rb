# frozen_string_literal: true

# Part 7 到達段階の挙動確認。
# impls/dist/part7/lib を使う（段の挙動）。

$LOAD_PATH.unshift File.expand_path('dist/part7/lib', __dir__)
require 'chibirigor'

# region accepts_demo
no_errors = Chibirigor.check("x = c ? 1 : 2\n1 + x")
puts "Integer | Integer: #{no_errors.empty? ? "OK（エラーなし）" : "NG"}"
puts Chibirigor.check("x = c ? 1 : \"a\"\n1 + x").map { |d| d[:message] }.first
# endregion
