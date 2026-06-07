# frozen_string_literal: true

# Part 5 ― ハッシュと配列の型（依存ゼロ・文字列ソース）
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

# HashShape の構築と読み
htypes = Chibirigor.annotate("h = { foo: 1, bar: \"a\" }\nh[:foo]\nh[:bar]\nh[:zzz]\n").map { |a| a[:type].to_s }
assert.call('hash shape is inferred', htypes[0], '{foo: 1, bar: "a"}')
assert.call('known key reads its type', htypes[1], '1')
assert.call('missing key reads nil', htypes[3], 'nil')

# Tuple の構築と読み
atypes = Chibirigor.annotate("a = [1, \"x\"]\na[0]\na[9]\n").map { |a| a[:type].to_s }
assert.call('tuple is inferred', atypes[0], '[1, "x"]')
assert.call('index reads the element type', atypes[1], '1')
assert.call('out-of-range index reads nil', atypes[2], 'nil')

# 読んだ型がチェックに流れる
assert.call('hash value type flows into a check', Chibirigor.check("h = { foo: \"a\" }\nh[:foo] + 1").size, 1)

# 未知キーの読みは決してエラーにしない
assert.call('missing key never errors', Chibirigor.check("h = { foo: 1 }\nh[:zzz]"), [])

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
