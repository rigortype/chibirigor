# frozen_string_literal: true

# Part 3 ― ローカル変数と不変 Scope（依存ゼロ・文字列ソース）
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

# 代入した変数を後で使える
assert.call('local is usable after assignment', Chibirigor.check("x = 1\nx + 2"), [])

# 変数はその型をエラー検出まで運ぶ
assert.call('local carries its type', Chibirigor.check("x = \"a\"\nx + 1").size, 1)

# 再代入で型が変わる
assert.call('reassignment changes the type', Chibirigor.check("x = 1\nx = \"a\"\nx + 1").size, 1)

# annotate が変数の型とその変化を見せる
types = Chibirigor.annotate("x = 1\nx\nx = \"a\"\nx\n").map { |a| a[:type].to_s }
assert.call('annotate shows local type', types[1], '1')
assert.call('annotate shows reassigned type', types[3], '"a"')

# 代入していない裸の名前はメソッド呼び出し扱い → 黙って degrade
assert.call('unbound bare name degrades silently', Chibirigor.check('y + 1'), [])

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
