# frozen_string_literal: true

# Smoke test for the Part 9 snapshot.
# The gradual philosophy: untyped dominance, baseline, plugin extension, constant folding.

require 'chibirigor'

failures = []
check = lambda do |desc, actual, expected|
  if actual == expected
    puts "PASS: #{desc}"
  else
    failures << desc
    puts "FAIL: #{desc} (expected #{expected.inspect}, got #{actual.inspect})"
  end
end

# untyped dominates a union
dom = Chibirigor.annotate("x = c ? 1 : foo.bar\nx").map { |h| h[:type].to_s }
check.call('untyped dominates a union', dom[1], 'untyped')

# A pure union is preserved as is
pure = Chibirigor.annotate("x = c ? 1 : \"a\"\nx").map { |h| h[:type].to_s }
check.call('a pure union is preserved', pure[1], '1 | "a"')

# baseline: already-accepted diagnostics are subtracted out
base = Chibirigor.check('1 + "x"')
check.call('baseline absorbs the known diagnostic', Chibirigor.check('1 + "x"', base), [])
check.call('a new diagnostic survives the baseline', Chibirigor.check("1 + \"x\"\n2 + true", base).size, 1)

# Plugin extension
Chibirigor.register_method(:MyClass, :my_method,
                           params: [Chibirigor::Type::Nominal[:Integer]],
                           returns: Chibirigor::Type::Nominal[:String])
check.call('plugin method is dispatched', Chibirigor.check('foo.my_method(1)'), [])
Chibirigor::Plugin.reset!

# Constant folding
ann = Chibirigor.annotate('1 + 2').map { |h| h[:type].to_s }
check.call('constant folding: 1 + 2 gives 3', ann[0], '3')

if failures.empty?
  puts 'All Part 9 stage checks passed.'
else
  warn "Part 9 FAILURES: #{failures.size}"
  exit 1
end
