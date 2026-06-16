# frozen_string_literal: true

# Part 5 — hash and array types (zero-dependency, string sources)
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

# Building and reading a HashShape.
htypes = Chibirigor.annotate("h = { foo: 1, bar: \"a\" }\nh[:foo]\nh[:bar]\nh[:zzz]\n").map { |a| a[:type].to_s }
assert.call('hash shape is inferred', htypes[0], '{foo: 1, bar: "a"}')
assert.call('known key reads its type', htypes[1], '1')
assert.call('missing key reads nil', htypes[3], 'nil')

# Building and reading a Tuple.
atypes = Chibirigor.annotate("a = [1, \"x\"]\na[0]\na[9]\n").map { |a| a[:type].to_s }
assert.call('tuple is inferred', atypes[0], '[1, "x"]')
assert.call('index reads the element type', atypes[1], '1')
assert.call('out-of-range index reads nil', atypes[2], 'nil')

# The read type flows into a check.
assert.call('hash value type flows into a check', Chibirigor.check("h = { foo: \"a\" }\nh[:foo] + 1").size, 1)

# Reading a missing key never errors.
assert.call('missing key never errors', Chibirigor.check("h = { foo: 1 }\nh[:zzz]"), [])

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
