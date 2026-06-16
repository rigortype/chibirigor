# frozen_string_literal: true

# Smoke test for the Part 4 snapshot.
# Union types: an if's two branches fold together into a Union.

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

# A ternary's two branches become a Union
types = Chibirigor.annotate("x = c ? 1 : \"a\"\nx\n").map { |h| h[:type].to_s }
check.call('the ternary branches become a Union', types[0], '1 | "a"')
check.call('a variable read also returns the Union', types[1], '1 | "a"')

# Union with nil
types2 = Chibirigor.annotate("x = c ? 1 : nil\nx\n").map { |h| h[:type].to_s }
check.call('Union with nil', types2[0], '1 | nil')

# A Union of the same type folds down to a single one
types3 = Chibirigor.annotate("x = c ? 1 : 2\nx\n").map { |h| h[:type].to_s }
# Const[1] and Const[2] are distinct Consts → Union[[1, 2]]
check.call('a Union of similar types stays expanded', types3[0].include?('|'), true)

if failures.empty?
  puts 'All Part 4 stage checks passed.'
else
  warn "Part 4 FAILURES: #{failures.size}"
  exit 1
end
