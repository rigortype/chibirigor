# frozen_string_literal: true

# ジェネリクス 5a ― 要素型の読み（スライス 4）のテスト（依存ゼロ）。
# 既知の配列／ハッシュから要素型 Elem を読む。生 Dynamic は untyped に倒す（FP ゼロ）。
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

ann = ->(src) { Chibirigor.annotate(src).map { |a| a[:type].to_s } }

# ── 配列の要素型の読み（Const は class に丸める） ────────────────────────────
assert.call('[1,2,3].first は Integer（要素型）', ann.call("[1,2,3].first\n").last, 'Integer')
assert.call('[1,2,3].last も Integer', ann.call("[1,2,3].last\n").last, 'Integer')
assert.call('混在配列の要素型は Union', ann.call("[1,\"a\"].first\n").last, 'Integer | String')

# ── 変数経由でも読める ────────────────────────────────────────────────────────
assert.call('変数の配列でも first は要素型', ann.call("a = [10, 20]\na.first\n").last, 'Integer')

# ── 非リテラル添字は要素型（位置は不明） ─────────────────────────────────────
assert.call('非リテラル添字 a[i] は要素型', ann.call("a = [1, 2]\ni = 0\na[i]\n").last, 'Integer')

# ── リテラル添字は位置どおり（Part 5 の精度を保つ） ──────────────────────────
assert.call('リテラル添字 a[0] は位置どおりの精度（Const）を保つ', ann.call("[1, \"a\"][0]\n").last, '1')

# ── 空配列は要素不明 → untyped（FP 安全） ────────────────────────────────────
assert.call('[].first は untyped（要素が分からない）', ann.call("[].first\n").last, 'untyped')

# ── 生 Dynamic 受信は untyped に倒れる（FP なし） ────────────────────────────
assert.call('未知レシーバの first は untyped（配列と決めつけない）',
            ann.call("foo.first\n").last, 'untyped')

# ── ハッシュの値型・キー型 ────────────────────────────────────────────────────
assert.call('h.values は値型の寄せ集め', ann.call("{ a: 1, b: 2 }.values\n").last, 'Integer')
assert.call('h.keys は Symbol', ann.call("{ a: 1 }.keys\n").last, 'Symbol')
assert.call('混在値の h.values は Union', ann.call("{ a: 1, b: \"x\" }.values\n").last, 'Integer | String')

# ── 要素型がチェックに流れる（誤検知ゼロは保つ） ─────────────────────────────
assert.call('読んだ要素型が check に流れる（Integer + true は 1 件）',
            Chibirigor.check("a = [1, 2]\na.first + true\n").size, 1)
assert.call('空配列の要素は untyped なので check は黙る（FP 安全）',
            Chibirigor.check("[].first + true\n").size, 0)

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
