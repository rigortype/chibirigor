# frozen_string_literal: true

# 依存ゼロの素朴なテスト（Prism 以外は何も要らない）。
# ソースは「文字列」で渡すのでフォーマッタの影響を受けない。
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

# check
assert.call('valid addition has no diagnostics', Chibirigor.check('1 + 2'), [])
assert.call('type mismatch is reported', Chibirigor.check('1 + true').size, 1)
assert.call('unknown call stays silent', Chibirigor.check('foo.bar'), [])

# annotate
types = Chibirigor.annotate("42\n1 + 2\nfoo.bar\n").map { |a| a[:type].to_s }
assert.call('annotate infers literal type', types[0], '42')
assert.call('annotate folds literal addition', types[1], '3')
assert.call('annotate marks unknown as untyped', types[2], 'untyped')

# 定数畳み込み（畳めれば畳む・無理なら丸める）
folds = Chibirigor.annotate("1 + 2 + 3\n\"a\" * 3\n").map { |a| a[:type].to_s }
assert.call('chained literal arithmetic folds', folds[0], '6')
assert.call('string repetition folds', folds[1], '"aaa"')
# 予算超過・非リテラルは畳まず丸める
rounds = Chibirigor.annotate("100000 * 100\n\"ab\".length + 1\n").map { |a| a[:type].to_s }
assert.call('over-budget result rounds to Integer', rounds[0], 'Integer')
assert.call('non-literal operand rounds to Integer', rounds[1], 'Integer')
# 変数が既知の Const を運んでいれば、それも畳む
through_var = Chibirigor.annotate("x = 1\n1 + x\n").map { |a| a[:type].to_s }
assert.call('known-Const variable folds too', through_var[1], '2')

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
