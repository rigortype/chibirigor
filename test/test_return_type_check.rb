# frozen_string_literal: true

# 戻り型照合（フェーズ 2 #4）のテスト（依存ゼロ）。
# check(source, rbs:) モードで、宣言された戻り型と本体の合成型を照合する。
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

RBS_INT = "class Greeter\n  def greet: () -> Integer\nend"

# ── 基本: 不一致は診断 ─────────────────────────────────────────────────────────
assert.call(
  '宣言 Integer に対し String を返すと診断',
  Chibirigor.check('def greet; "hi"; end', rbs: RBS_INT).size, 1
)

assert.call(
  '診断メッセージに宣言型が含まれる',
  Chibirigor.check('def greet; "hi"; end', rbs: RBS_INT).first[:message].include?('Integer'), true
)

assert.call(
  '診断メッセージに実際の型が含まれる',
  Chibirigor.check('def greet; "hi"; end', rbs: RBS_INT).first[:message].include?('hi'), true
)

# ── 一致: 診断なし ────────────────────────────────────────────────────────────
assert.call(
  '宣言 Integer に対し Integer を返すと診断なし',
  Chibirigor.check('def greet; 1; end', rbs: RBS_INT).size, 0
)

assert.call(
  '宣言 Integer に対し Const（整数リテラル）は通る',
  Chibirigor.check('def greet; 42; end', rbs: RBS_INT).size, 0
)

# ── opt-in: 宣言なし = 照合なし ───────────────────────────────────────────────
assert.call(
  'rbs: なしなら宣言なしで診断なし（FP ゼロ）',
  Chibirigor.check('def greet; "hi"; end').size, 0
)

assert.call(
  '宣言のないメソッドは照合しない',
  Chibirigor.check('def other; "hi"; end', rbs: RBS_INT).size, 0
)

# ── gradual: untyped 宣言は照合しない ─────────────────────────────────────────
RBS_UNTYPED = "class C\n  def greet: () -> untyped\nend"
assert.call(
  'untyped 宣言は型不一致でも黙る',
  Chibirigor.check('def greet; "hi"; end', rbs: RBS_UNTYPED).size, 0
)

# ── gradual: 本体が untyped（推論不能）なら照合しない ─────────────────────────
assert.call(
  '本体が untyped（foo.bar）なら診断なし（FP 安全）',
  Chibirigor.check('def greet; foo.bar; end', rbs: RBS_INT).size, 0
)

# ── baseline との共存 ─────────────────────────────────────────────────────────
base = Chibirigor.check('def greet; "hi"; end', rbs: RBS_INT)
assert.call(
  'baseline で既知の戻り型診断を吸収できる',
  Chibirigor.check('def greet; "hi"; end', base, rbs: RBS_INT).size, 0
)

# ── 複数メソッドを同時宣言 ────────────────────────────────────────────────────
RBS_MULTI = <<~RBS
  class App
    def greet: () -> String
    def count: () -> Integer
  end
RBS

source_ok   = "def greet; \"hello\"; end\ndef count; 1; end"
source_bad  = "def greet; \"hello\"; end\ndef count; \"oops\"; end"

assert.call('複数宣言: 全一致なら診断なし', Chibirigor.check(source_ok,  rbs: RBS_MULTI).size, 0)
assert.call('複数宣言: 1 つ不一致なら診断 1 件', Chibirigor.check(source_bad, rbs: RBS_MULTI).size, 1)

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
