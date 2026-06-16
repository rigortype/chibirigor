# frozen_string_literal: true

# Part 9 milestone behavior check.
# Dynamic dominance, baseline, constant folding

$LOAD_PATH.unshift File.expand_path('dist/part9/lib', __dir__)
require 'chibirigor'

# region untyped_dominance
# Show whether untyped dominates a union (check via the type of x)
x_with_unknown = Chibirigor.annotate("x = c ? 1 : foo.bar\nx\n").last[:type]
x_pure         = Chibirigor.annotate("x = c ? 1 : \"a\"\nx\n").last[:type]
puts "c ? 1 : foo.bar  ->  #{x_with_unknown}"
puts "c ? 1 : \"a\"      ->  #{x_pure}"
# endregion
