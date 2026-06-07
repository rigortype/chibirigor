# frozen_string_literal: true

# Part 6 ― 受理判定・三値（依存ゼロ・文字列ソース）
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

int = Chibirigor::Type::Nominal[:Integer]
const = Chibirigor::Type::Const
union = Chibirigor::Type::Union
dyn = Chibirigor::Type::Dynamic
accepts = Chibirigor::Accepts

# 三値そのもの
assert.call('concrete match is yes', accepts.call(int, const[1]), :yes)
assert.call('concrete mismatch is no', accepts.call(int, const['x']), :no)
assert.call('dynamic is maybe', accepts.call(int, dyn.new), :maybe)
assert.call('union all-ok is yes', accepts.call(int, union[[const[1], int]]), :yes)
assert.call('union with dynamic is maybe', accepts.call(int, union[[const[1], dyn.new]]), :maybe)
assert.call('union with mismatch is no', accepts.call(int, union[[const[1], const['x']]]), :no)

# 挙動：:maybe は罰しない／union-of-ok はもう誤検知しない／本当の不一致は出る
assert.call('dynamic arg stays silent', Chibirigor.check('1 + foo.bar'), [])
assert.call('union of integers is no longer a false positive', Chibirigor.check("x = c ? 1 : 2\n1 + x"), [])
assert.call('union with a bad member is reported', Chibirigor.check("x = c ? 1 : \"a\"\n1 + x").size, 1)

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
