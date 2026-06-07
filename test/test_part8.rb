# frozen_string_literal: true

# Part 8 ― annotate を仕上げる（依存ゼロ・文字列ソース）
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

# 本体から戻り型を合成し、RBS 風シグネチャで見せる
sig = Chibirigor.annotate("def greet\n  \"hi\".upcase\nend\n").first[:type]
assert.call('return type synthesized from body', sig, 'def greet: () -> String')

# 引数は untyped。untyped がどこに出るか＝推論の弱点の可視化
sig2 = Chibirigor.annotate("def mystery(x)\n  x\nend\n").first[:type]
assert.call('untyped param yields untyped return', sig2, 'def mystery: (untyped) -> untyped')

# def の本体も型検査される（check と annotate は同じ推論器を共有）
assert.call('def body is type-checked', Chibirigor.check("def bad\n  1 + \"x\"\nend\n").size, 1)

# untyped 引数は本体で誤検知しない（脅かさない）
assert.call('untyped param is FP-safe in the body', Chibirigor.check("def ok(x)\n  x + 1\nend\n"), [])

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
