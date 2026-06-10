# frozen_string_literal: true

# check --explain（fail-soft 地図・スライス 2）のテスト（依存ゼロ）。
# explain: true のとき、推論が untyped に倒した地点を :info 診断として併せて返す。
# 既定（explain なし）は挙動不変＝fail-soft は出さない。
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

# ── 既定では挙動不変（fail-soft は出さない） ─────────────────────────────────
assert.call('未知メソッドでも既定の check は診断ゼロ（FP ゼロ・挙動不変）',
            Chibirigor.check('foo.bar').size, 0)

assert.call('型エラーのある式は既定でも 1 件（fail-soft は混ざらない）',
            Chibirigor.check('1 + true').size, 1)

# ── explain で fail-soft 地点が :info として現れる ────────────────────────────
soft = Chibirigor.check('mystery', explain: true) # receiver なしの未知メソッド送信＝1 地点
assert.call('explain で未知ディスパッチが :info で出る', soft.size, 1)
assert.call('fail-soft は severity :info', soft.first[:severity], :info)
assert.call('fail-soft は kind :fail_soft', soft.first[:kind], :fail_soft)
assert.call('メッセージにメソッド名が入る', soft.first[:message].include?('mystery'), true)

# 入れ子呼び出しは fail-soft の地点が複数（foo と foo.bar 全体）。両方のメソッド名が出る。
nested = Chibirigor.check('foo.bar', explain: true)
assert.call('入れ子の未知呼び出しは fail-soft 2 件', nested.size, 2)
assert.call('入れ子: foo と bar の両方が現れる',
            nested.map { |d| d[:message] }.join.then { |m| m.include?('foo') && m.include?('bar') }, true)

# ── 既知の呼び出しは fail-soft を出さない（ノイズなし） ───────────────────────
assert.call('既知の算術 1 + 2 は explain でも fail-soft ゼロ',
            Chibirigor.check('1 + 2', explain: true).size, 0)

# ── ナローイング述語（is_a?/nil?）は fail-soft 地図に載せない（誤った沈黙地点を出さない） ──
assert.call('is_a? は fail-soft に載らない（述語であって型喪失でない）',
            Chibirigor.check("x = 1\nif x.is_a?(String)\n 0\nend\n", explain: true).size, 0)
assert.call('nil? も fail-soft に載らない',
            Chibirigor.check("x = 1\nif x.nil?\n 0\nend\n", explain: true).size, 0)

# ── 型エラー（本物）と fail-soft（:info）が併存できる ─────────────────────────
mixed = Chibirigor.check("1 + true\nmystery", explain: true)
errors = mixed.reject { |d| d[:severity] == :info }
infos  = mixed.select { |d| d[:severity] == :info }
assert.call('explain: 本物のエラーは 1 件', errors.size, 1)
assert.call('explain: fail-soft は 1 件', infos.size, 1)
assert.call('explain: エラーは exit を汚すが fail-soft は :info（severity で区別できる）',
            infos.all? { |d| d[:severity] == :info }, true)

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
