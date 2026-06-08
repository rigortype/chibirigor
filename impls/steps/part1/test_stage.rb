# frozen_string_literal: true

# Part 1 到達段階のスモークテスト。
# 生成された impls/dist/part1/lib に対して `ruby -I .../lib test_stage.rb` で走る。
# この段の挙動（リテラル→値・加算は丸めて Integer・不明は untyped・止まらない）を固定する。

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

check.call('1 + 2 は診断なし', Chibirigor.check('1 + 2'), [])
check.call('型不一致を診断', Chibirigor.check('1 + "x"'),
           [{ line: 1, message: '整数に "x" は足せません' }])
check.call('知らないメソッドは黙る', Chibirigor.check('foo.bar'), [])

ann = Chibirigor.annotate("42\n1 + 2\nfoo.bar\n").map { |h| "#{h[:line]}: #{h[:type]}" }
check.call('annotate: 値そのもの／加算は丸めて Integer／不明は untyped',
           ann, ['1: 42', '2: Integer', '3: untyped'])

if failures.empty?
  puts 'All Part 1 stage checks passed.'
else
  warn "Part 1 FAILURES: #{failures.size}"
  exit 1
end
