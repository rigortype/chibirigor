# frozen_string_literal: true

# Part 2 — method sends and dispatch (zero-dependency, string sources)
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

# Known methods in the table.
assert.call('integer subtraction ok', Chibirigor.check('1 - 2'), [])
assert.call('string concat ok', Chibirigor.check('"a" + "b"'), [])
assert.call('chained addition ok', Chibirigor.check('1 + 2 + 3'), [])

# Wrong argument type.
assert.call('string + integer is reported', Chibirigor.check('"a" + 1').size, 1)

# Wrong number of arguments.
assert.call('wrong arity is reported', Chibirigor.check('"ab".length(1)').size, 1)

# Unknown methods don't raise alarms.
assert.call('unknown method stays silent', Chibirigor.check('foo.bar(1, 2)'), [])

# Return types (checked via annotate; use non-folding methods to see the return type itself).
types = Chibirigor.annotate("1.to_s\n\"ab\".length\n\"a\".upcase\n").map { |a| a[:type].to_s }
assert.call('Integer#to_s returns String', types[0], 'String')
assert.call('String#length returns Integer', types[1], 'Integer')
assert.call('String#upcase returns String', types[2], 'String')

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
