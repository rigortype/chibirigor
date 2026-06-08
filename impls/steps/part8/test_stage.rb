# frozen_string_literal: true

# Part 8 到達段階のスモークテスト。
# RBS 派生の dispatch 表＋ def の本体型チェックとシグネチャ合成。

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

# Rbs.load が class / def を表にする
table = Chibirigor::Rbs.load("class Foo\n  def bar: (Integer) -> String\nend\n")
check.call('rbs parses a param type', table[%i[Foo bar]][:params].map(&:to_s), ['Integer'])
check.call('rbs parses the return type', table[%i[Foo bar]][:returns].to_s, 'String')

# コア表は RBS 由来でも挙動は変わらない
check.call('RBS-backed dispatch still reports', Chibirigor.check('1 + "x"').size, 1)
check.call('RBS-backed dispatch still passes', Chibirigor.check('1 + 2'), [])

# def の本体型チェック
check.call('def body is type-checked', Chibirigor.check("def bad\n  1 + \"x\"\nend\n").size, 1)
check.call('untyped param is FP-safe', Chibirigor.check("def ok(x)\n  x + 1\nend\n"), [])

# annotate が RBS 風シグネチャを返す
sig = Chibirigor.annotate("def greet\n  \"hi\".upcase\nend\n").first[:type]
check.call('return type synthesized from body', sig, 'def greet: () -> String')

sig2 = Chibirigor.annotate("def mystery(x)\n  x\nend\n").first[:type]
check.call('untyped param yields untyped return', sig2, 'def mystery: (untyped) -> untyped')

if failures.empty?
  puts 'All Part 8 stage checks passed.'
else
  warn "Part 8 FAILURES: #{failures.size}"
  exit 1
end
