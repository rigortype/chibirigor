# frozen_string_literal: true

# Tests for the minimal plugin hook (zero-dependency).
# Confirm that a method registered via Chibirigor.register_method takes effect in dispatch.
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

# Clean up registered entries before each test.
Chibirigor::Plugin.reset!

# ── Test 1: a registered method's return type is reflected in annotate ───────
Chibirigor.register_method(
  :String, :shout,
  params:  [],
  returns: Chibirigor::Type::Nominal[:String]
)

result = Chibirigor.annotate('"hello".shout')
assert.call('registered method returns declared type', result.last[:type].to_s, 'String')

# ── Test 2: a type error against a registered method is detected ─────────────
Chibirigor::Plugin.reset!

Chibirigor.register_method(
  :Integer, :repeat,
  params:  [Chibirigor::Type::Nominal[:Integer]],
  returns: Chibirigor::Type::Nominal[:String]
)

diags = Chibirigor.check('1.repeat("bad")')
assert.call('registered method rejects wrong argument type', diags.size, 1)

# ── Test 3: a correct argument yields no diagnostic ──────────────────────────
diags_ok = Chibirigor.check('1.repeat(3)')
assert.call('registered method accepts correct argument type', diags_ok.size, 0)

# ── Test 4: zero FP — an unregistered method falls to Dynamic (untyped) ──────
Chibirigor::Plugin.reset!
result_unknown = Chibirigor.annotate('"hello".unknown_method')
assert.call('unknown method degrades to untyped', result_unknown.last[:type].to_s, 'untyped')

# ── Test 5: the core METHODS table is not mutated ────────────────────────────
Chibirigor.register_method(
  :Integer, :to_s,
  params:  [],
  returns: Chibirigor::Type::Nominal[:Symbol]   # deliberately wrong override
)
result_core = Chibirigor.annotate('1.to_s')
assert.call('plugin overrides core without mutating METHODS constant', result_core.last[:type].to_s, 'Symbol')
assert.call('METHODS constant is intact', Chibirigor::Dispatch::METHODS[[:Integer, :to_s]][:returns].to_s, 'String')

Chibirigor::Plugin.reset!

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
