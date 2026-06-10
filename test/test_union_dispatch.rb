# frozen_string_literal: true

# Union ディスパッチ（レシーバ分配＋畳み込み分配）のテスト（依存ゼロ）。
# (1|2).+ はメンバごとにディスパッチして結果を union する。引数の Union は
# 畳み込みで直積に分配する（1 + (1|2) → 2 | 3）。エラーは全メンバで失敗した
# ときだけ表に出す（一部の失敗は :maybe ＝黙る、誤検知ゼロ）。
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

last_type = ->(source) { Chibirigor.annotate(source).last[:type].to_s }

# ── 畳み込みの Union 分配（引数が Union） ────────────────────────────────────
assert.call('1 + (1|2) はメンバごとに畳む',
            last_type.call("a = 1\nb = a + (rand == 0 ? 1 : 2)"), '2 | 3')

# ── レシーバ分配（レシーバが Union） ────────────────────────────────────────
assert.call('(1|2) - 1 はレシーバを分配して畳む',
            last_type.call("b = rand == 0 ? 1 : 2\nc = b - 1"), '0 | 1')

assert.call('分配の連鎖でも精度を保つ（(2|3) - 1 → 1 | 2）',
            last_type.call("a = 1\nb = a + (rand == 0 ? 1 : 2)\nc = b - a"), '1 | 2')

assert.call('結果が重なれば union が 1 つにまとめる（(1|2) * 0 → 0）',
            last_type.call("b = rand == 0 ? 1 : 2\nc = b * 0"), '0')

# ── 異種 Union：畳めるメンバは畳み、畳めないメンバは表の戻り型へ丸める ──────
assert.call('(1|"a") + 1 はメンバごとの結果の union（Integer 側だけ畳む）',
            last_type.call(%(x = rand == 0 ? 1 : "a"\ny = x + 1)), '2 | String')

# ── 診断方針：全メンバで失敗したときだけ怒る ────────────────────────────────
assert.call('一部メンバの失敗は黙る（:maybe・誤検知ゼロ）',
            Chibirigor.check(%(x = rand == 0 ? 1 : "a"\ny = x + 1)).size, 0)

assert.call('全メンバで失敗すれば 1 件だけ怒る',
            Chibirigor.check(%(x = rand == 0 ? 1 : 2\ny = x + "a")).size, 1)

# ── gradual：未知メンバが混じれば untyped に倒す（fail-soft は地図に残る） ──
assert.call('未知メンバ（nil.+）が混じれば untyped',
            last_type.call("x = rand == 0 ? 1 : nil\ny = x + 1"), 'untyped')

assert.call('既定の check では診断ゼロ（fail-soft は隠れる）',
            Chibirigor.check("x = rand == 0 ? 1 : nil\ny = x + 1").size, 0)

# 1 行目には rand / == の fail-soft もあるので、分配が起きた 2 行目だけ数える
soft = Chibirigor.check("x = rand == 0 ? 1 : nil\ny = x + 1", explain: true)
assert.call('explain では分配先の fail-soft 地点が 1 件（メンバ分の重複はしない）',
            soft.count { |d| d[:kind] == :fail_soft && d[:line] == 2 }, 1)

# ── MEMBER_LIMIT：メンバ数予算を超えたらクラスに丸める ──────────────────────
five = "b = rand == 0 ? 1 : (rand == 0 ? 2 : (rand == 0 ? 3 : (rand == 0 ? 4 : 5)))\n"
assert.call('5 メンバの Union への演算は Integer に丸める（メンバ数予算）',
            last_type.call("#{five}c = b + 1"), 'Integer')

assert.call('4 メンバまでは精度を保つ',
            last_type.call("b = rand == 0 ? 1 : (rand == 0 ? 2 : (rand == 0 ? 3 : 4))\nc = b + 1"),
            '2 | 3 | 4 | 5')

if failures.empty?
  puts 'すべてのテストが通りました'
else
  puts "#{failures.size} 件失敗: #{failures.join(', ')}"
  exit 1
end
