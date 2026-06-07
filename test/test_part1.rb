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
assert.call('annotate rounds addition to Integer', types[1], 'Integer')
assert.call('annotate marks unknown as untyped', types[2], 'untyped')

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
