# frozen_string_literal: true

# 到達不能アーム診断（ADR-47 縮小版・スライス 3）のテスト（依存ゼロ）。
# check(source, unreachable: true) のとき、証明可能に到達不能な枝を :info 診断で返す。
# 既定（unreachable なし）は挙動不変＝本物の型エラーだけ。誤検知ゼロを厳守する。
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

unreachables = lambda do |src|
  Chibirigor.check(src, unreachable: true).select { |d| d[:kind] == :unreachable }
end

# ── 既定では挙動不変（Part 4 の約束：dead 枝に黙る・FP ゼロ） ──────────────────
assert.call('既定: 起き得ない is_a? 枝は診断ゼロ（Part 4 の約束を維持）',
            Chibirigor.check("x = 1\nif x.is_a?(String)\n x + 1\nend\n"), [])

# ── 真の枝が必ず偽（is_a? 互いに素な葉） ──────────────────────────────────────
assert.call('x:Integer の is_a?(String) は then 到達不能',
            unreachables.call("x = 1\nif x.is_a?(String)\n 0\nend\n").size, 1)

assert.call('到達不能は severity :info（exit を汚さない）',
            unreachables.call("x = 1\nif x.is_a?(String)\n 0\nend\n").first[:severity], :info)

# ── 真の枝が必ず偽（nil?：nil になり得ない） ──────────────────────────────────
assert.call('x:Integer の nil? は then 到達不能',
            unreachables.call("x = 1\nif x.nil?\n 0\nend\n").size, 1)

# ── else が必ず真（恒真ガード） ───────────────────────────────────────────────
assert.call('x:Integer の is_a?(Integer) は else 到達不能',
            unreachables.call("x = 1\nif x.is_a?(Integer)\n 0\nelse\n 1\nend\n").size, 1)

# ── 健全性（FP ゼロ）：祖先関係を断言しない ───────────────────────────────────
assert.call('is_a?(Numeric) は祖先ゆえ到達不能と断言しない（FP 回避）',
            unreachables.call("x = 1\nif x.is_a?(Numeric)\n 0\nend\n").size, 0)

assert.call('is_a?(Object) も断言しない（FP 回避）',
            unreachables.call("x = 1\nif x.is_a?(Object)\n 0\nend\n").size, 0)

# ── 健全性：到達可能な枝は黙る ────────────────────────────────────────────────
assert.call('Union に String を含むなら is_a?(String) は到達可能',
            unreachables.call("x = c ? 1 : \"a\"\nif x.is_a?(String)\n 0\nend\n").size, 0)

assert.call('nil になり得るなら nil? は到達可能',
            unreachables.call("x = c ? 1 : nil\nif x.nil?\n 0\nend\n").size, 0)

# ── 健全性：untyped（gradual）は断言しない ───────────────────────────────────
assert.call('untyped な変数の is_a? は到達不能と断言しない（gradual）',
            unreachables.call("if y.is_a?(String)\n 0\nend\n").size, 0)

# ── opt-in：既定では :unreachable を出さない ─────────────────────────────────
assert.call('unreachable フラグ無しなら到達不能 :info は出ない',
            Chibirigor.check("x = 1\nif x.is_a?(String)\n 0\nend\n").any? { |d| d[:kind] == :unreachable }, false)

# ─────────────────────────────────────────────────────────────────────────────
if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failure(s): #{failures.inspect}"
  exit 1
end
