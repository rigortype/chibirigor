# frozen_string_literal: true

# Smoke test for the Part 3 snapshot.
# Local variables and an immutable Scope. Assignment grows the type; reassignment changes it.

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

check.call('an assigned variable is usable later', Chibirigor.check("x = 1\nx + 2"), [])
check.call('the variable type carries an error', Chibirigor.check("x = \"a\"\nx + 1").size, 1)
check.call('reassignment changes the type', Chibirigor.check("x = 1\nx = \"a\"\nx + 1").size, 1)

ann = Chibirigor.annotate("x = 1\nx\nx = \"a\"\nx\n").map { |h| h[:type].to_s }
check.call('the assignment type is the value itself', ann[0], '1')
check.call('a variable read is the assigned type', ann[1], '1')
check.call('the type after reassignment', ann[3], '"a"')

check.call('an unbound variable stays quiet', Chibirigor.check('y + 1'), [])

if failures.empty?
  puts 'All Part 3 stage checks passed.'
else
  warn "Part 3 FAILURES: #{failures.size}"
  exit 1
end
