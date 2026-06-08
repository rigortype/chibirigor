# frozen_string_literal: true

# Part 3〜6 ― Scope・Union・narrowing・accepts の動作確認。

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

# ── Part 3: Scope と文 ────────────────────────────────────────────────────────
ok.call('x=1 → x+2 は診断なし',
        Chibirigor.check("x = 1\nx + 2"), [])
ok.call('x="a" → x+1 は型エラー',
        Chibirigor.check("x = \"a\"\nx + 1").size, 1)
ok.call('再代入後の型で検査する',
        Chibirigor.check("x = 1\nx = \"a\"\nx + 1").size, 1)

# ── Part 4: Union と narrowing ────────────────────────────────────────────────
ok.call('dead branch は誤検知しない（Integer に is_a?(String) は絞らない）',
        Chibirigor.check("x = 1\nif x.is_a?(String)\n  x + 1\nend"), [])
ok.call('Union に is_a? で正しく絞り込む',
        Chibirigor.check("x = c ? 1 : \"a\"\nif x.is_a?(String)\n  x + 1\nend").size, 1)
ok.call('nil チェック後は nil を除く',
        Chibirigor.check("x = c ? 1 : nil\nif x.nil?\nelse\n  x + 1\nend"), [])

# ── Part 5: HashShape / Tuple ─────────────────────────────────────────────────
ok.call('ハッシュの型が返る',
        Chibirigor.annotate('{ foo: 1, bar: "a" }').first[:type].to_s, '{foo: 1, bar: "a"}')
ok.call('配列（Tuple）の型が返る',
        Chibirigor.annotate('[1, "x"]').first[:type].to_s, '[1, "x"]')
ok.call('存在しないキーは nil（エラーにしない）',
        Chibirigor.check('h = { foo: 1 }; h[:zzz]'), [])

# ── Part 6: accepts / 三値 ────────────────────────────────────────────────────
ok.call('1 + 2 は診断なし（:yes）',
        Chibirigor.check('1 + 2'), [])
ok.call('1 + "x" は診断あり（:no）',
        Chibirigor.check('1 + "x"').size, 1)
ok.call('untyped の引数は黙る（:maybe）',
        Chibirigor.check('1 + foo.bar'), [])
ok.call('Integer メッセージ本文に型名が入る',
        Chibirigor.check('1 + "x"').first[:message].include?('Integer'), true)

if failures.empty?
  puts "\nAll checks passed."
else
  failures.each { |f| warn "  FAIL: #{f}" }
  exit 1
end
