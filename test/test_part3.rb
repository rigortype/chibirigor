# frozen_string_literal: true

# Part 3 — local variables and immutable Scope (zero-dependency, string sources)
$LOAD_PATH.unshift File.expand_path('../lib', __dir__)
require 'chibirigor'

failures = []
assert = lambda do |desc, actual, expected|
  if actual == expected
    puts "PASS: #{desc}"
  else
    failures << desc
    puts "FAIL: #{desc} (expected #{expected.inspect}, got #{actual.inspect})"
  end
end

# An assigned variable can be used later.
assert.call('local is usable after assignment', Chibirigor.check("x = 1\nx + 2"), [])

# A variable carries its type through to error detection.
assert.call('local carries its type', Chibirigor.check("x = \"a\"\nx + 1").size, 1)

# Reassignment changes the type.
assert.call('reassignment changes the type', Chibirigor.check("x = 1\nx = \"a\"\nx + 1").size, 1)

# annotate shows a variable's type and how it changes.
types = Chibirigor.annotate("x = 1\nx\nx = \"a\"\nx\n").map { |a| a[:type].to_s }
assert.call('annotate shows local type', types[1], '1')
assert.call('annotate shows reassigned type', types[3], '"a"')

# An unassigned bare name is treated as a method call → degrades silently.
assert.call('unbound bare name degrades silently', Chibirigor.check('y + 1'), [])

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
