# frozen_string_literal: true

# Part 4 ― Union と絞り込み（依存ゼロ・文字列ソース）
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

# if の両枝が Union になる
types = Chibirigor.annotate("x = c ? 1 : \"a\"\nx\n").map { |a| a[:type].to_s }
assert.call('if branches form a union', types[0], '1 | "a"')

# nil チェックで else 枝が nil を除く → x + 1 が通る
assert.call(
  'nil-check narrows the else branch',
  Chibirigor.check("x = c ? 1 : nil\nif x.nil?\n 0\nelse\n x + 1\nend\n"), []
)

# is_a? は「あり得る枝」を絞る → そこの本当のエラーが出る
assert.call(
  'is_a? narrows a reachable branch',
  Chibirigor.check("x = c ? 1 : \"a\"\nif x.is_a?(String)\n x + 1\nend\n").size, 1
)

# is_a? は disjoint な型を絞らない → 起き得ない枝で誤検知しない
assert.call(
  'is_a? leaves the dead branch alone (no false positive)',
  Chibirigor.check("x = 1\nif x.is_a?(String)\n x + 1\nend\n"), []
)

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
