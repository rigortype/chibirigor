# frozen_string_literal: true

# Part 4 到達段階の挙動確認。
# impls/dist/part4/lib を使う。

$LOAD_PATH.unshift File.expand_path('dist/part4/lib', __dir__)
require 'chibirigor'

# region union_demo
x_int_str = Chibirigor.annotate("x = c ? 1 : \"a\"\nx\n").last[:type]
x_int_nil = Chibirigor.annotate("x = c ? 1 : nil\nx\n").last[:type]
puts "c ? 1 : \"a\"  ->  #{x_int_str}"
puts "c ? 1 : nil   ->  #{x_int_nil}"
# endregion
