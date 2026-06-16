# frozen_string_literal: true

# Smoke test for the Part 6 snapshot.
# HashShape and Tuple: structural types let read types flow through.

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

htypes = Chibirigor.annotate("h = { foo: 1, bar: \"a\" }\nh[:foo]\nh[:bar]\nh[:zzz]\n").map { |h| h[:type].to_s }
check.call('hash shape is inferred', htypes[0], '{foo: 1, bar: "a"}')
check.call('known key reads its type', htypes[1], '1')
check.call('missing key reads nil', htypes[3], 'nil')

atypes = Chibirigor.annotate("a = [1, \"x\"]\na[0]\na[9]\n").map { |h| h[:type].to_s }
check.call('tuple is inferred', atypes[0], '[1, "x"]')
check.call('index reads the element type', atypes[1], '1')
check.call('out-of-range index reads nil', atypes[2], 'nil')

check.call('hash value type flows into a check', Chibirigor.check("h = { foo: \"a\" }\nh[:foo] + 1").size, 1)
check.call('missing key never errors', Chibirigor.check("h = { foo: 1 }\nh[:zzz]"), [])

if failures.empty?
  puts 'All Part 6 stage checks passed.'
else
  warn "Part 6 FAILURES: #{failures.size}"
  exit 1
end
