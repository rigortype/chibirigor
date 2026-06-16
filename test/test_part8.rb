# frozen_string_literal: true

# Part 8 — finishing annotate (zero-dependency, string sources)
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

# Synthesize the return type from the body and show it as an RBS-style signature.
sig = Chibirigor.annotate("def greet\n  \"hi\".upcase\nend\n").first[:type]
assert.call('return type synthesized from body', sig, 'def greet: () -> String')

# Params are untyped. Where untyped appears = visualizing the inference's weak spots.
sig2 = Chibirigor.annotate("def mystery(x)\n  x\nend\n").first[:type]
assert.call('untyped param yields untyped return', sig2, 'def mystery: (untyped) -> untyped')

# A def body is type-checked too (check and annotate share the same inference engine).
assert.call('def body is type-checked', Chibirigor.check("def bad\n  1 + \"x\"\nend\n").size, 1)

# An untyped param produces no false positive in the body (no alarms).
assert.call('untyped param is FP-safe in the body', Chibirigor.check("def ok(x)\n  x + 1\nend\n"), [])

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
