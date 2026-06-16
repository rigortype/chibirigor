# frozen_string_literal: true

# Part 2 milestone behavior check.

$LOAD_PATH.unshift File.expand_path('dist/part2/lib', __dir__)
require 'chibirigor'

# region dispatch_demo
Chibirigor.annotate("1.to_s\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
Chibirigor.annotate('"ab".length' + "\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
puts Chibirigor.check('1 + "x"').map { |d| d[:message] }.first
# endregion
