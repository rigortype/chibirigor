# frozen_string_literal: true

# type-of 位置クエリ（スライス 1）のテスト（依存ゼロ）。
# Chibirigor.type_at(source, line, col) は位置を含む最小の式の型を返す。
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

ts = ->(src, line, col) { Chibirigor.type_at(src, line, col)&.to_s }

# ── 最小の式を選ぶ ────────────────────────────────────────────────────────────
# "1 + 2": 列1 は整数リテラル 1、列3 は呼び出し全体（畳んで 3）。
assert.call('リテラルを指すとその値型', ts.call('1 + 2', 1, 1), '1')
assert.call('演算子を指すと式全体（定数畳み込みで 3）', ts.call('1 + 2', 1, 3), '3')

# ── 文をまたいで scope が育つ ─────────────────────────────────────────────────
assert.call('後続行の局所変数参照は手前の束縛で解決', ts.call("x = 5\ny = x", 2, 5), '5')

# ── 未知は untyped（脅かさない） ──────────────────────────────────────────────
assert.call('未知メソッド送信は untyped', ts.call('foo.bar', 1, 5), 'untyped')

# ── 文字列リテラル ────────────────────────────────────────────────────────────
assert.call('文字列リテラルはその値型', ts.call('"hi"', 1, 1), '"hi"')

# ── 位置に式が無ければ nil ────────────────────────────────────────────────────
assert.call('式の無い位置は nil', Chibirigor.type_at('1 + 2', 5, 1), nil)

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
