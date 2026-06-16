# frozen_string_literal: true

# Part 7 — a dash of RBS (zero-dependency, string sources)
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

# Rbs.load turns class / def into a table.
table = Chibirigor::Rbs.load("class Foo\n  def bar: (Integer) -> String\n  def baz: () -> Integer\nend\n")
assert.call('rbs parses a param type', table[%i[Foo bar]][:params].map(&:to_s), ['Integer'])
assert.call('rbs parses the return type', table[%i[Foo bar]][:returns].to_s, 'String')
assert.call('rbs parses a no-arg method', table[%i[Foo baz]][:params], [])

# The core table is RBS-derived. Swapping it out keeps the behavior the same.
assert.call('core table is RBS-derived', Chibirigor::Dispatch::METHODS.key?(%i[Integer +]), true)
assert.call('RBS-backed dispatch still reports', Chibirigor.check('1 + "x"').size, 1)
assert.call('RBS-backed dispatch still passes', Chibirigor.check('1 + 2'), [])

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
