# frozen_string_literal: true

# Part 1 到達段階の挙動確認。
# impls/dist/part1/lib を使う（加算は丸めて Integer）。

$LOAD_PATH.unshift File.expand_path('dist/part1/lib', __dir__)
require 'chibirigor'

# region annotate_demo
Chibirigor.annotate(<<~RUBY).each { |a| puts "#{a[:line]}: #{a[:type]}" }
  42
  "hello"
  1 + 2
  foo.bar
RUBY
# endregion
