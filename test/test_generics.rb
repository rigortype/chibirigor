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

# ── 5b：ブロック仮引数へ要素型を押し下げる（generics の山場） ─────────────────
assert.call('map のブロック仮引数 x は要素型（map → Array[Elem]）',
            ann.call("[1, 2].map { |x| x + 1 }\n").last, 'Array[Integer]')
assert.call('map の戻りは本体の型（5c 戻り多相）',
            ann.call("[1, 2].map { |x| x.to_s }\n").last, 'Array[String]')
assert.call('select は要素型を保つ', ann.call("[1, 2, 3].select { |x| x }\n").last, 'Array[Integer]')
assert.call('each はレシーバ（self）を返す', ann.call("[1, 2].each { |x| x + 1 }\n").last, '[1, 2]')
assert.call('map の結果も配列なので first が読める（連鎖）',
            ann.call("[1, 2].map { |x| x.to_s }.first\n").last, 'String')

# ── 5b：ブロック本体が要素型で型チェックされる（押し下げの証拠） ──────────────
assert.call('ブロック本体の型エラーを検出（Integer + true は 1 件）',
            Chibirigor.check("[1, 2].map { |x| x + true }\n").size, 1)
assert.call('each のブロック本体も型チェックされる',
            Chibirigor.check("[1, 2].each { |x| x + \"a\" }\n").size, 1)
assert.call('正しいブロックは診断ゼロ', Chibirigor.check("[1, 2].map { |x| x + 1 }\n").size, 0)

# ── 5b：FP ゼロ（空配列・未知レシーバ・未知反復子） ──────────────────────────
assert.call('空配列のブロックは要素 untyped → 本体も untyped → 黙る（FP 安全）',
            Chibirigor.check("[].map { |x| x + true }\n").size, 0)
assert.call('未知レシーバの map は untyped（配列と決めつけず本体も検査しない）',
            ann.call("foo.map { |x| x + 1 }\n").last, 'untyped')

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
