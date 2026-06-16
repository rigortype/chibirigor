# frozen_string_literal: true

# Tests for check --explain (the fail-soft map, slice 2; zero-dependency).
# With explain: true, the points where inference fell to untyped are also returned as :info diagnostics.
# The default (no explain) is behavior-unchanged = no fail-soft emitted.
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

# ── Default is behavior-unchanged (no fail-soft emitted) ─────────────────────
assert.call('default check stays silent on unknown methods (zero FP, behavior-unchanged)',
            Chibirigor.check('foo.bar').size, 0)

assert.call('an expression with a type error still reports 1 by default (no fail-soft mixed in)',
            Chibirigor.check('1 + true').size, 1)

# ── explain surfaces fail-soft points as :info ───────────────────────────────
soft = Chibirigor.check('mystery', explain: true) # unknown method send with no receiver = 1 point
assert.call('explain surfaces an unknown dispatch as :info', soft.size, 1)
assert.call('fail-soft has severity :info', soft.first[:severity], :info)
assert.call('fail-soft has kind :fail_soft', soft.first[:kind], :fail_soft)
assert.call('the message includes the method name', soft.first[:message].include?('mystery'), true)

# A nested call has multiple fail-soft points (foo, and the whole foo.bar). Both method names appear.
nested = Chibirigor.check('foo.bar', explain: true)
assert.call('a nested unknown call yields 2 fail-soft points', nested.size, 2)
assert.call('nested: both foo and bar appear',
            nested.map { |d| d[:message] }.join.then { |m| m.include?('foo') && m.include?('bar') }, true)

# ── Known calls emit no fail-soft (no noise) ─────────────────────────────────
assert.call('known arithmetic 1 + 2 yields zero fail-soft even with explain',
            Chibirigor.check('1 + 2', explain: true).size, 0)

# ── Narrowing predicates (is_a?/nil?) aren't put on the fail-soft map (no spurious silence points) ──
assert.call('is_a? is not on the fail-soft map (it is a predicate, not a type loss)',
            Chibirigor.check("x = 1\nif x.is_a?(String)\n 0\nend\n", explain: true).size, 0)
assert.call('nil? is not on the fail-soft map either',
            Chibirigor.check("x = 1\nif x.nil?\n 0\nend\n", explain: true).size, 0)

# ── A real type error and a fail-soft (:info) can coexist ────────────────────
mixed = Chibirigor.check("1 + true\nmystery", explain: true)
errors = mixed.reject { |d| d[:severity] == :info }
infos  = mixed.select { |d| d[:severity] == :info }
assert.call('explain: the real error count is 1', errors.size, 1)
assert.call('explain: the fail-soft count is 1', infos.size, 1)
assert.call('explain: errors pollute exit but fail-soft is :info (distinguishable by severity)',
            infos.all? { |d| d[:severity] == :info }, true)

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
