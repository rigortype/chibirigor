# frozen_string_literal: true

# Smoke test for the Part 1 snapshot.
# Runs against the generated impls/dist/part1/lib via `ruby -I .../lib test_stage.rb`.
# Pins this stage's behavior (literal → value, addition rounds to Integer, unknown is untyped, don't halt).

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

check.call('1 + 2 has no diagnostics', Chibirigor.check('1 + 2'), [])
check.call('type mismatch is diagnosed', Chibirigor.check('1 + "x"'),
           [{ line: 1, message: 'can\'t add "x" to an integer' }])
check.call('unknown method stays quiet', Chibirigor.check('foo.bar'), [])

ann = Chibirigor.annotate("42\n1 + 2\nfoo.bar\n").map { |h| "#{h[:line]}: #{h[:type]}" }
check.call('annotate: value itself / addition rounds to Integer / unknown is untyped',
           ann, ['1: 42', '2: Integer', '3: untyped'])

if failures.empty?
  puts 'All Part 1 stage checks passed.'
else
  warn "Part 1 FAILURES: #{failures.size}"
  exit 1
end
