# frozen_string_literal: true

# Zero-dependency naive tests (nothing needed beyond Prism).
# Sources are passed as strings, so they're unaffected by formatters.
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

# check
assert.call('valid addition has no diagnostics', Chibirigor.check('1 + 2'), [])
assert.call('type mismatch is reported', Chibirigor.check('1 + true').size, 1)
assert.call('unknown call stays silent', Chibirigor.check('foo.bar'), [])

# Diagnostics carry a position (line, column, length) — used for the CLI caret display.
diag = Chibirigor.check('1 + true').first
assert.call('diagnostic carries line and column', [diag[:line], diag[:column]], [1, 0])
assert.call('diagnostic carries a span length', diag[:length].positive?, true)

# annotate
types = Chibirigor.annotate("42\n1 + 2\nfoo.bar\n").map { |a| a[:type].to_s }
assert.call('annotate infers literal type', types[0], '42')
assert.call('annotate folds literal addition', types[1], '3')
assert.call('annotate marks unknown as untyped', types[2], 'untyped')

# Constant folding (fold when foldable, round otherwise).
folds = Chibirigor.annotate("1 + 2 + 3\n\"a\" * 3\n").map { |a| a[:type].to_s }
assert.call('chained literal arithmetic folds', folds[0], '6')
assert.call('string repetition folds', folds[1], '"aaa"')
# Over-budget or non-literal: round instead of fold.
rounds = Chibirigor.annotate("100000 * 100\n\"ab\".length + 1\n").map { |a| a[:type].to_s }
assert.call('over-budget result rounds to Integer', rounds[0], 'Integer')
assert.call('non-literal operand rounds to Integer', rounds[1], 'Integer')
# If a variable carries a known Const, fold through it too.
through_var = Chibirigor.annotate("x = 1\n1 + x\n").map { |a| a[:type].to_s }
assert.call('known-Const variable folds too', through_var[1], '2')

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
