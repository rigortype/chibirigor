# frozen_string_literal: true

# Part 9 ― gradual の哲学（依存ゼロ・文字列ソース）
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

# untyped は union を支配する（一部でも未知なら全体 untyped）
dom = Chibirigor.annotate("x = c ? 1 : foo.bar\nx").map { |a| a[:type].to_s }
assert.call('untyped dominates a union', dom[1], 'untyped')

# 未知の混じらない union はそのまま
pure = Chibirigor.annotate("x = c ? 1 : \"a\"\nx").map { |a| a[:type].to_s }
assert.call('a pure union is preserved', pure[1], '1 | "a"')

# baseline：既に呑んだ診断は差し引く
base = Chibirigor.check('1 + "x"')
assert.call('baseline absorbs the known diagnostic', Chibirigor.check('1 + "x"', base), [])
assert.call('a new diagnostic survives the baseline', Chibirigor.check("1 + \"x\"\n2 + true", base).size, 1)

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
