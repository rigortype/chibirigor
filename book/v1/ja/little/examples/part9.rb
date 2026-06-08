# frozen_string_literal: true

# Part 9 到達段階の挙動確認。
# impls/dist/part9/lib を使う（Dynamic 支配・baseline・定数畳み込み）。

$LOAD_PATH.unshift File.expand_path('../../../../../impls/dist/part9/lib', __dir__)
require 'chibirigor'

# region untyped_dominance
# untyped が union を支配するかを示す（変数 x の型で確認）
x_with_unknown = Chibirigor.annotate("x = c ? 1 : foo.bar\nx\n").last[:type]
x_pure         = Chibirigor.annotate("x = c ? 1 : \"a\"\nx\n").last[:type]
puts "c ? 1 : foo.bar  ->  #{x_with_unknown}"
puts "c ? 1 : \"a\"      ->  #{x_pure}"
# endregion
