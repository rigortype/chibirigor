# frozen_string_literal: true

# Part 8 milestone behavior check.
# RBS-derived dispatch + synthesized def signatures

$LOAD_PATH.unshift File.expand_path('dist/part8/lib', __dir__)
require 'chibirigor'

# region greet_sig
Chibirigor.annotate("def greet\n  \"hi\".upcase\nend\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
# endregion

# region untyped_sigs
Chibirigor.annotate("def double(n)\n  n * 2\nend\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
Chibirigor.annotate("def mystery(x)\n  x\nend\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
# endregion
