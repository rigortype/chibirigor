# frozen_string_literal: true

# Smoke test for the Part 7 snapshot.
# The three-valued acceptance check: :yes/:no/:maybe handle Union correctly.

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

int   = Chibirigor::Type::Nominal[:Integer]
const = Chibirigor::Type::Const
dyn   = Chibirigor::Type::Dynamic
acc   = Chibirigor::Accepts

check.call('concrete match is yes',   acc.call(int, const[1]),   :yes)
check.call('concrete mismatch is no', acc.call(int, const['x']), :no)
check.call('dynamic is maybe',        acc.call(int, dyn.new),    :maybe)

# Every member of the Union is Integer → :yes (removes the false positive)
check.call('union of integers is no longer a false positive',
           Chibirigor.check("x = c ? 1 : 2\n1 + x"), [])

# A String mixed into the Union → :no → an error is reported
check.call('union with a bad member is reported',
           Chibirigor.check("x = c ? 1 : \"a\"\n1 + x").size, 1)

# untyped stays quiet
check.call('dynamic arg stays silent', Chibirigor.check('1 + foo.bar'), [])

if failures.empty?
  puts 'All Part 7 stage checks passed.'
else
  warn "Part 7 FAILURES: #{failures.size}"
  exit 1
end
