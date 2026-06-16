# frozen_string_literal: true

# dump_type(expr) — a basic feature that prints the inferred type at that position as :info (zero-dependency, string sources).
# Corresponds to real Rigor's Rigor::Testing.dump_type. check always co-emits it, and at run time the value passes through.
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

# check prints dump_type as :info (no flag needed, a basic feature).
ds = Chibirigor.check("x = c ? 1 : \"a\"\ndump_type(x)\n")
dump = ds.find { |d| d[:kind] == :dump_type }
assert.call('dump_type emits a diagnostic', !dump.nil?, true)
assert.call('dump_type is :info severity', dump && dump[:severity], :info)
assert.call('dump_type prints the inferred type', dump && dump[:message], 'dump_type: 1 | "a"')
assert.call('dump_type points at its own line', dump && dump[:line], 2)

# It's :info, so it's not a "real type error" (an error-free source has an empty error tier).
errors = Chibirigor.check("dump_type(\"a\".upcase)\n").reject { |d| d[:severity] == :info }
assert.call('dump_type alone raises no real error', errors, [])

# At run time the value passes through → the expression's type is the argument's type (checked via annotate).
types = Chibirigor.annotate("dump_type(\"a\".upcase)\n").map { |a| a[:type].to_s }
assert.call('dump_type passes the value type through', types[0], 'String')

# Can coexist with a real type error (dump_type doesn't affect exit, but both diagnostics appear).
mixed = Chibirigor.check("dump_type(1)\n1 + \"x\"\n")
assert.call('real error still reported alongside dump_type', mixed.count { |d| d[:severity] != :info }, 1)
assert.call('dump_type info also present', mixed.count { |d| d[:kind] == :dump_type }, 1)

# dump_type without exactly one argument isn't special-cased (unknown method = silently untyped).
assert.call('dump_type with no args is not special', Chibirigor.check("dump_type()\n").any? { |d| d[:kind] == :dump_type }, false)

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
