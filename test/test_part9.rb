# frozen_string_literal: true

# Part 9 — the philosophy of gradual typing (zero-dependency, string sources)
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

# untyped dominates a union (if any part is unknown, the whole is untyped).
dom = Chibirigor.annotate("x = c ? 1 : foo.bar\nx").map { |a| a[:type].to_s }
assert.call('untyped dominates a union', dom[1], 'untyped')

# A union with no unknowns is preserved as is.
pure = Chibirigor.annotate("x = c ? 1 : \"a\"\nx").map { |a| a[:type].to_s }
assert.call('a pure union is preserved', pure[1], '1 | "a"')

# baseline: subtract diagnostics already swallowed.
base = Chibirigor.check('1 + "x"')
assert.call('baseline absorbs the known diagnostic', Chibirigor.check('1 + "x"', base), [])
assert.call('a new diagnostic survives the baseline', Chibirigor.check("1 + \"x\"\n2 + true", base).size, 1)

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
