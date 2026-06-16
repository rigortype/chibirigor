# frozen_string_literal: true

# Smoke test for the Part 2 snapshot.
# The dispatch table checks argument types and counts. Addition still rounds to Integer through the table.

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

check.call('argument type error is diagnosed', Chibirigor.check('"a" + 1'),
           [{ line: 1, message: 'expected String but got 1' }])
check.call('argument count error is diagnosed', Chibirigor.check('"ab".length(1)'),
           [{ line: 1, message: 'wrong number of arguments for length (0 expected, 1 given)' }])
check.call('unknown receiver / method stays quiet', Chibirigor.check('foo.bar(1, 2)'), [])

ann = Chibirigor.annotate('1 + 2').map { |h| "#{h[:line]}: #{h[:type]}" }
check.call('addition rounds to Integer even through dispatch', ann, ['1: Integer'])

if failures.empty?
  puts 'All Part 2 stage checks passed.'
else
  warn "Part 2 FAILURES: #{failures.size}"
  exit 1
end
