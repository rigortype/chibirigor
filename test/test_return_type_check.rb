# frozen_string_literal: true

# Tests for return-type matching (phase 2 #4; zero-dependency).
# In check(source, rbs:) mode, match the declared return type against the body's synthesized type.
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

RBS_INT = "class Greeter\n  def greet: () -> Integer\nend"

# ── Basic: a mismatch is diagnosed ──────────────────────────────────────────────
assert.call(
  'returning String against a declared Integer is diagnosed',
  Chibirigor.check('def greet; "hi"; end', rbs: RBS_INT).size, 1
)

assert.call(
  'the diagnostic message includes the declared type',
  Chibirigor.check('def greet; "hi"; end', rbs: RBS_INT).first[:message].include?('Integer'), true
)

assert.call(
  'the diagnostic message includes the actual type',
  Chibirigor.check('def greet; "hi"; end', rbs: RBS_INT).first[:message].include?('hi'), true
)

# ── Match: no diagnostic ──────────────────────────────────────────────────────
assert.call(
  'returning Integer against a declared Integer yields no diagnostic',
  Chibirigor.check('def greet; 1; end', rbs: RBS_INT).size, 0
)

assert.call(
  'a Const (integer literal) passes against a declared Integer',
  Chibirigor.check('def greet; 42; end', rbs: RBS_INT).size, 0
)

# ── opt-in: no declaration = no matching ──────────────────────────────────────
assert.call(
  'with no rbs:, no declaration means no diagnostic (zero FP)',
  Chibirigor.check('def greet; "hi"; end').size, 0
)

assert.call(
  'a method without a declaration is not matched',
  Chibirigor.check('def other; "hi"; end', rbs: RBS_INT).size, 0
)

# ── gradual: an untyped declaration is not matched ────────────────────────────
RBS_UNTYPED = "class C\n  def greet: () -> untyped\nend"
assert.call(
  'an untyped declaration stays silent even on a type mismatch',
  Chibirigor.check('def greet; "hi"; end', rbs: RBS_UNTYPED).size, 0
)

# ── gradual: if the body is untyped (uninferrable), don't match ───────────────
assert.call(
  'an untyped body (foo.bar) yields no diagnostic (FP-safe)',
  Chibirigor.check('def greet; foo.bar; end', rbs: RBS_INT).size, 0
)

# ── Coexistence with baseline ─────────────────────────────────────────────────
base = Chibirigor.check('def greet; "hi"; end', rbs: RBS_INT)
assert.call(
  'a baseline can absorb a known return-type diagnostic',
  Chibirigor.check('def greet; "hi"; end', base, rbs: RBS_INT).size, 0
)

# ── Declaring multiple methods at once ────────────────────────────────────────
RBS_MULTI = <<~RBS
  class App
    def greet: () -> String
    def count: () -> Integer
  end
RBS

source_ok   = "def greet; \"hello\"; end\ndef count; 1; end"
source_bad  = "def greet; \"hello\"; end\ndef count; \"oops\"; end"

assert.call('multiple declarations: all matching yields no diagnostic', Chibirigor.check(source_ok,  rbs: RBS_MULTI).size, 0)
assert.call('multiple declarations: one mismatch yields 1 diagnostic', Chibirigor.check(source_bad, rbs: RBS_MULTI).size, 1)

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
