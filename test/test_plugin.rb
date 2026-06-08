# frozen_string_literal: true

# 極小プラグインフックのテスト（依存ゼロ）。
# Chibirigor.register_method で登録したメソッドが dispatch に効くことを確認する。
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

# 各テストの前に登録済みエントリを掃除する
Chibirigor::Plugin.reset!

# ── テスト 1: 登録したメソッドの戻り型が annotate に反映される ──────────────
Chibirigor.register_method(
  :String, :shout,
  params:  [],
  returns: Chibirigor::Type::Nominal[:String]
)

result = Chibirigor.annotate('"hello".shout')
assert.call('registered method returns declared type', result.last[:type].to_s, 'String')

# ── テスト 2: 登録したメソッドへの型エラーが検出される ─────────────────────
Chibirigor::Plugin.reset!

Chibirigor.register_method(
  :Integer, :repeat,
  params:  [Chibirigor::Type::Nominal[:Integer]],
  returns: Chibirigor::Type::Nominal[:String]
)

diags = Chibirigor.check('1.repeat("bad")')
assert.call('registered method rejects wrong argument type', diags.size, 1)

# ── テスト 3: 正しい引数は診断なし ────────────────────────────────────────────
diags_ok = Chibirigor.check('1.repeat(3)')
assert.call('registered method accepts correct argument type', diags_ok.size, 0)

# ── テスト 4: FP ゼロ ― 未登録メソッドは Dynamic（untyped）に倒れる ─────────
Chibirigor::Plugin.reset!
result_unknown = Chibirigor.annotate('"hello".unknown_method')
assert.call('unknown method degrades to untyped', result_unknown.last[:type].to_s, 'untyped')

# ── テスト 5: コア METHODS は書き換えない ─────────────────────────────────────
Chibirigor.register_method(
  :Integer, :to_s,
  params:  [],
  returns: Chibirigor::Type::Nominal[:Symbol]   # わざと間違った上書き
)
result_core = Chibirigor.annotate('1.to_s')
assert.call('plugin overrides core without mutating METHODS constant', result_core.last[:type].to_s, 'Symbol')
assert.call('METHODS constant is intact', Chibirigor::Dispatch::METHODS[[:Integer, :to_s]][:returns].to_s, 'String')

Chibirigor::Plugin.reset!

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
