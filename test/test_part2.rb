# frozen_string_literal: true

# Part 2 ― メソッド送信とディスパッチ（依存ゼロ・文字列ソース）
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

# 表にある既知メソッド
assert.call('integer subtraction ok', Chibirigor.check('1 - 2'), [])
assert.call('string concat ok', Chibirigor.check('"a" + "b"'), [])
assert.call('chained addition ok', Chibirigor.check('1 + 2 + 3'), [])

# 引数の型が違う
assert.call('string + integer is reported', Chibirigor.check('"a" + 1').size, 1)

# 引数の数が違う
assert.call('wrong arity is reported', Chibirigor.check('"ab".length(1)').size, 1)

# 知らないメソッドは脅かさない
assert.call('unknown method stays silent', Chibirigor.check('foo.bar(1, 2)'), [])

# 戻り型（annotate で確認。畳まれないメソッドで戻り型そのものを見る）
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
