# frozen_string_literal: true

# Part 1 ― check と annotate の基本動作。
# 本文に示した出力と一致することを確認する。

$LOAD_PATH.unshift File.expand_path('../../../../lib', __dir__)
require 'chibirigor'

failures = []
ok = lambda do |desc, actual, expected|
  if actual == expected
    puts "PASS: #{desc}"
  else
    failures << "#{desc}: expected #{expected.inspect}, got #{actual.inspect}"
    puts "FAIL: #{desc}"
  end
end

# check の基本
ok.call('1 + 2 は診断なし',      Chibirigor.check('1 + 2'),    [])
ok.call('型不一致は診断 1 件',    Chibirigor.check('1 + "x"').size, 1)
ok.call('知らないメソッドは黙る', Chibirigor.check('foo.bar'),  [])

# check のメッセージに行番号が含まれる
diag = Chibirigor.check('1 + "x"').first
ok.call('診断に :line キーがある',    diag.key?(:line),    true)
ok.call('診断に :message キーがある', diag.key?(:message), true)
ok.call('診断の行番号は 1',           diag[:line],         1)

# annotate の基本
results = Chibirigor.annotate(<<~RUBY)
  42
  "hello"
  1 + 2
  foo.bar
RUBY

ok.call('annotate: 4 件返る',          results.size,                4)
ok.call('annotate: 42 → "42"',         results[0][:type].to_s,     '42')
ok.call('annotate: "hello" → "hello"', results[1][:type].to_s,     '"hello"')
ok.call('annotate: 1+2 → 3（定数畳み込み）', results[2][:type].to_s, '3')
ok.call('annotate: foo.bar → untyped', results[3][:type].to_s,     'untyped')

# 定数畳み込み（1-4a 発展ノート）：閾値を超えたら Nominal に丸める（widening）
ok.call('annotate: 大きな数は Integer に丸める（widening）',
        Chibirigor.annotate('1000001 + 1').first[:type].to_s, 'Integer')

# annotate 出力デモ（<!-- run: --> で章本文に埋め込み）
puts "\n-- annotate demo --"
Chibirigor.annotate(<<~RUBY).each { |a| puts "#{a[:line]}: #{a[:type]}" }
  42
  "hello"
  1 + 2
  foo.bar
RUBY

if failures.empty?
  puts "\nAll checks passed."
else
  failures.each { |f| warn "  FAIL: #{f}" }
  exit 1
end
