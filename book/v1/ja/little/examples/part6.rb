# frozen_string_literal: true

# Part 6 到達段階の挙動確認。
# impls/dist/part6/lib を使う。

$LOAD_PATH.unshift File.expand_path('dist/part6/lib', __dir__)
require 'chibirigor'

# region hash_demo
Chibirigor.annotate("h = {foo: 1, bar: \"a\"}\nh[:foo]\nh[:bar]\nh[:zzz]\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
# endregion
