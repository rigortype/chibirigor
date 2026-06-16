# frozen_string_literal: true

# Part 4 — Union and narrowing (zero-dependency, string sources)
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

# Both branches of an if form a Union.
types = Chibirigor.annotate("x = c ? 1 : \"a\"\nx\n").map { |a| a[:type].to_s }
assert.call('if branches form a union', types[0], '1 | "a"')

# A nil check removes nil from the else branch → x + 1 passes.
assert.call(
  'nil-check narrows the else branch',
  Chibirigor.check("x = c ? 1 : nil\nif x.nil?\n 0\nelse\n x + 1\nend\n"), []
)

# is_a? narrows to a reachable branch → the real error there surfaces.
assert.call(
  'is_a? narrows a reachable branch',
  Chibirigor.check("x = c ? 1 : \"a\"\nif x.is_a?(String)\n x + 1\nend\n").size, 1
)

# is_a? doesn't narrow a disjoint type → no false positive on an impossible branch.
assert.call(
  'is_a? leaves the dead branch alone (no false positive)',
  Chibirigor.check("x = 1\nif x.is_a?(String)\n x + 1\nend\n"), []
)

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
