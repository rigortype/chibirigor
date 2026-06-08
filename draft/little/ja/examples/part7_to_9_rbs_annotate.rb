# frozen_string_literal: true

# Part 7〜9 ― RBS・annotate・baseline の動作確認。

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

# ── Part 7: RBS ──────────────────────────────────────────────────────────────
# Dispatch::METHODS は RBS 由来（手書き表を differ 置換済み）。
# 既存テストと同等の検証：RBS 由来でも同じ診断が出ること。
ok.call('RBS 由来の表：String + Integer はエラー',
        Chibirigor.check('"a" + 1').size, 1)
ok.call('RBS 由来の表：引数の数の違いはエラー',
        Chibirigor.check('"ab".length(1)').size, 1)
ok.call('RBS 由来の表：String#upcase は通る',
        Chibirigor.check('"hello".upcase'), [])
ok.call('RBS 由来の表：String#upcase の戻りは String',
        Chibirigor.annotate('"hello".upcase').first[:type].to_s, 'String')

# ── Part 8: annotate と def のシグネチャ ────────────────────────────────────
sig_results = Chibirigor.annotate(<<~RUBY)
  def greet(name)
    "Hello, " + name
  end
RUBY
ok.call('def の annotate はシグネチャ文字列',
        sig_results.first[:type].include?('def greet'), true)
ok.call('def のシグネチャに -> が含まれる',
        sig_results.first[:type].include?('->'), true)
ok.call('戻り型が String（本体最終式が String）',
        sig_results.first[:type].include?('String'), true)

# ── Part 9: baseline ─────────────────────────────────────────────────────────
source = '1 + "x"'
base   = Chibirigor.check(source)   # 既知の診断を baseline に
ok.call('baseline で吸収した診断は出ない',
        Chibirigor.check(source, base).size, 0)
ok.call('新規エラーは baseline があっても出る',
        Chibirigor.check("1 + \"x\"\n\"a\" + 1", base).size, 1)

# ── Part 9: gradual（untyped の伝播）────────────────────────────────────────
ok.call('untyped + Integer は診断なし（:maybe）',
        Chibirigor.check('foo.bar + 1'), [])
ok.call('untyped の足し算も診断なし',
        Chibirigor.check('x = foo.bar; x + 1'), [])

if failures.empty?
  puts "\nAll checks passed."
else
  failures.each { |f| warn "  FAIL: #{f}" }
  exit 1
end
