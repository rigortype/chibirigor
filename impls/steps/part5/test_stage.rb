# frozen_string_literal: true

# Smoke test for the Part 5 snapshot.
# Narrowing: nil? / is_a? narrow the type per branch.

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

# A nil-check removes nil in the else branch
check.call('nil-check narrows the else branch',
           Chibirigor.check("x = c ? 1 : nil\nif x.nil?\n  0\nelse\n  x + 1\nend\n"), [])

# is_a? narrows to the "possible" branch
check.call('is_a? narrows a reachable branch',
           Chibirigor.check("x = c ? 1 : \"a\"\nif x.is_a?(String)\n  x + 1\nend\n").size, 1)

# is_a? doesn't narrow a disjoint type → no false positive
check.call('is_a? leaves the dead branch alone',
           Chibirigor.check("x = 1\nif x.is_a?(String)\n  x + 1\nend\n"), [])

if failures.empty?
  puts 'All Part 5 stage checks passed.'
else
  warn "Part 5 FAILURES: #{failures.size}"
  exit 1
end
