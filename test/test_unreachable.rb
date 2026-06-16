# frozen_string_literal: true

# Tests for the unreachable-arm diagnostic (a reduced ADR-47, slice 3; zero-dependency).
# With check(source, unreachable: true), provably unreachable branches are returned as :info diagnostics.
# The default (no unreachable) is behavior-unchanged = real type errors only. Zero false positives is strict.
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

unreachables = lambda do |src|
  Chibirigor.check(src, unreachable: true).select { |d| d[:kind] == :unreachable }
end

# ── Default is behavior-unchanged (Part 4's promise: stay silent on dead branches, zero FP) ──
assert.call('default: an impossible is_a? branch yields zero diagnostics (keeps Part 4 promise)',
            Chibirigor.check("x = 1\nif x.is_a?(String)\n x + 1\nend\n"), [])

# ── The then branch is always false (is_a? with disjoint leaves) ─────────────
assert.call('is_a?(String) on x:Integer makes the then branch unreachable',
            unreachables.call("x = 1\nif x.is_a?(String)\n 0\nend\n").size, 1)

assert.call('unreachable has severity :info (does not pollute exit)',
            unreachables.call("x = 1\nif x.is_a?(String)\n 0\nend\n").first[:severity], :info)

# ── The then branch is always false (nil?: can't be nil) ─────────────────────
assert.call('nil? on x:Integer makes the then branch unreachable',
            unreachables.call("x = 1\nif x.nil?\n 0\nend\n").size, 1)

# ── The else is always true (a tautological guard) ────────────────────────────
assert.call('is_a?(Integer) on x:Integer makes the else branch unreachable',
            unreachables.call("x = 1\nif x.is_a?(Integer)\n 0\nelse\n 1\nend\n").size, 1)

# ── Soundness (zero FP): don't assert ancestor relations ──────────────────────
assert.call("is_a?(Numeric) isn't asserted unreachable (it's an ancestor; avoid FP)",
            unreachables.call("x = 1\nif x.is_a?(Numeric)\n 0\nend\n").size, 0)

assert.call("is_a?(Object) isn't asserted either (avoid FP)",
            unreachables.call("x = 1\nif x.is_a?(Object)\n 0\nend\n").size, 0)

# ── Soundness: stay silent on reachable branches ──────────────────────────────
assert.call('if a Union contains String, is_a?(String) is reachable',
            unreachables.call("x = c ? 1 : \"a\"\nif x.is_a?(String)\n 0\nend\n").size, 0)

assert.call('if it can be nil, nil? is reachable',
            unreachables.call("x = c ? 1 : nil\nif x.nil?\n 0\nend\n").size, 0)

# ── Soundness: don't assert about untyped (gradual) ───────────────────────────
assert.call("is_a? on an untyped variable isn't asserted unreachable (gradual)",
            unreachables.call("if y.is_a?(String)\n 0\nend\n").size, 0)

# ── opt-in: by default no :unreachable is emitted ─────────────────────────────
assert.call('without the unreachable flag, no unreachable :info appears',
            Chibirigor.check("x = 1\nif x.is_a?(String)\n 0\nend\n").any? { |d| d[:kind] == :unreachable }, false)

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
