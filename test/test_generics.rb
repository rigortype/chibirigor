# frozen_string_literal: true

# Generics 5a — reading element types (slice 4; zero-dependency).
# Read the element type Elem from a known array/hash. A raw Dynamic falls to untyped (zero FP).
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

ann = ->(src) { Chibirigor.annotate(src).map { |a| a[:type].to_s } }

# ── Reading an array's element type (Const rounds to its class) ──────────────
assert.call('[1,2,3].first is Integer (element type)', ann.call("[1,2,3].first\n").last, 'Integer')
assert.call('[1,2,3].last is Integer too', ann.call("[1,2,3].last\n").last, 'Integer')
assert.call('a mixed array element type is a Union', ann.call("[1,\"a\"].first\n").last, 'Integer | String')

# ── Readable through a variable too ───────────────────────────────────────────
assert.call('first on an array variable is the element type', ann.call("a = [10, 20]\na.first\n").last, 'Integer')

# ── A non-literal index reads the element type (position unknown) ────────────
assert.call('non-literal index a[i] is the element type', ann.call("a = [1, 2]\ni = 0\na[i]\n").last, 'Integer')

# ── A literal index keeps positional precision (preserving Part 5) ───────────
assert.call('literal index a[0] keeps positional precision (Const)', ann.call("[1, \"a\"][0]\n").last, '1')

# ── An empty array has unknown elements → untyped (FP-safe) ──────────────────
assert.call('[].first is untyped (the element is unknown)', ann.call("[].first\n").last, 'untyped')

# ── A raw Dynamic receiver falls to untyped (no FP) ──────────────────────────
assert.call("first on an unknown receiver is untyped (don't assume it's an array)",
            ann.call("foo.first\n").last, 'untyped')

# ── Hash value type and key type ──────────────────────────────────────────────
assert.call('h.values is the collected value types', ann.call("{ a: 1, b: 2 }.values\n").last, 'Integer')
assert.call('h.keys is Symbol', ann.call("{ a: 1 }.keys\n").last, 'Symbol')
assert.call('h.values with mixed values is a Union', ann.call("{ a: 1, b: \"x\" }.values\n").last, 'Integer | String')

# ── Element types flow into a check (keep zero false positives) ──────────────
assert.call('the read element type flows into check (Integer + true is 1)',
            Chibirigor.check("a = [1, 2]\na.first + true\n").size, 1)
assert.call('an empty array element is untyped, so check stays silent (FP-safe)',
            Chibirigor.check("[].first + true\n").size, 0)

# ── 5b: push the element type down into the block parameter (the crux of generics) ──
assert.call("map's block parameter x is the element type (map → Array[Elem])",
            ann.call("[1, 2].map { |x| x + 1 }\n").last, 'Array[Integer]')
assert.call("map's return is the body's type (5c return polymorphism)",
            ann.call("[1, 2].map { |x| x.to_s }\n").last, 'Array[String]')
assert.call('select preserves the element type', ann.call("[1, 2, 3].select { |x| x }\n").last, 'Array[Integer]')
assert.call('each returns the receiver (self)', ann.call("[1, 2].each { |x| x + 1 }\n").last, '[1, 2]')
assert.call("map's result is an array too, so first reads (chaining)",
            ann.call("[1, 2].map { |x| x.to_s }.first\n").last, 'String')

# ── 5b: the block body is type-checked against the element type (proof of push-down) ──
assert.call('detect a type error in the block body (Integer + true is 1)',
            Chibirigor.check("[1, 2].map { |x| x + true }\n").size, 1)
assert.call("each's block body is type-checked too",
            Chibirigor.check("[1, 2].each { |x| x + \"a\" }\n").size, 1)
assert.call('a correct block yields zero diagnostics', Chibirigor.check("[1, 2].map { |x| x + 1 }\n").size, 0)

# ── 5b: zero FP (empty array, unknown receiver, unknown iterator) ────────────
assert.call('an empty-array block has untyped element → untyped body → silent (FP-safe)',
            Chibirigor.check("[].map { |x| x + true }\n").size, 0)
assert.call("map on an unknown receiver is untyped (don't assume an array; don't check the body)",
            ann.call("foo.map { |x| x + 1 }\n").last, 'untyped')

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
