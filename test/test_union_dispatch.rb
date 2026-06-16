# frozen_string_literal: true

# Tests for Union dispatch (receiver distribution + folding distribution; zero-dependency).
# (1|2).+ dispatches per member and unions the results. A Union argument is
# distributed over the Cartesian product by folding (1 + (1|2) → 2 | 3). An error
# surfaces only when every member fails (a partial failure is :maybe = silent, zero FP).
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

last_type = ->(source) { Chibirigor.annotate(source).last[:type].to_s }

# ── Folding Union distribution (the argument is a Union) ─────────────────────
assert.call('1 + (1|2) folds per member',
            last_type.call("a = 1\nb = a + (rand == 0 ? 1 : 2)"), '2 | 3')

# ── Receiver distribution (the receiver is a Union) ──────────────────────────
assert.call('(1|2) - 1 distributes the receiver and folds',
            last_type.call("b = rand == 0 ? 1 : 2\nc = b - 1"), '0 | 1')

assert.call('chained distribution keeps precision ((2|3) - 1 → 1 | 2)',
            last_type.call("a = 1\nb = a + (rand == 0 ? 1 : 2)\nc = b - a"), '1 | 2')

assert.call('overlapping results collapse into one union ((1|2) * 0 → 0)',
            last_type.call("b = rand == 0 ? 1 : 2\nc = b * 0"), '0')

# ── Heterogeneous Union: fold the foldable members, round the rest to the table's return type ──
assert.call('(1|"a") + 1 is a union of per-member results (only the Integer side folds)',
            last_type.call(%(x = rand == 0 ? 1 : "a"\ny = x + 1)), '2 | String')

# ── Diagnostic policy: complain only when every member fails ─────────────────
assert.call('a partial-member failure stays silent (:maybe, zero FP)',
            Chibirigor.check(%(x = rand == 0 ? 1 : "a"\ny = x + 1)).size, 0)

assert.call('if every member fails, complain exactly once',
            Chibirigor.check(%(x = rand == 0 ? 1 : 2\ny = x + "a")).size, 1)

# ── gradual: if an unknown member is mixed in, fall to untyped (fail-soft stays on the map) ──
assert.call('an unknown member (nil.+) mixed in is untyped',
            last_type.call("x = rand == 0 ? 1 : nil\ny = x + 1"), 'untyped')

assert.call('default check yields zero diagnostics (fail-soft is hidden)',
            Chibirigor.check("x = rand == 0 ? 1 : nil\ny = x + 1").size, 0)

# Line 1 also has fail-soft from rand / ==, so count only line 2 where distribution happened.
soft = Chibirigor.check("x = rand == 0 ? 1 : nil\ny = x + 1", explain: true)
assert.call('explain shows 1 fail-soft point at the distribution site (no per-member duplication)',
            soft.count { |d| d[:kind] == :fail_soft && d[:line] == 2 }, 1)

# ── MEMBER_LIMIT: round to the class once the member-count budget is exceeded ──
five = "b = rand == 0 ? 1 : (rand == 0 ? 2 : (rand == 0 ? 3 : (rand == 0 ? 4 : 5)))\n"
assert.call('an operation on a 5-member Union rounds to Integer (member-count budget)',
            last_type.call("#{five}c = b + 1"), 'Integer')

assert.call('precision is kept up to 4 members',
            last_type.call("b = rand == 0 ? 1 : (rand == 0 ? 2 : (rand == 0 ? 3 : 4))\nc = b + 1"),
            '2 | 3 | 4 | 5')

if failures.empty?
  puts 'All tests passed.'
else
  puts "#{failures.size} failed: #{failures.join(', ')}"
  exit 1
end
