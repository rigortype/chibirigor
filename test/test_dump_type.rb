# frozen_string_literal: true

# dump_type(式) ― その位置の推論型を :info で印字する基本機能（依存ゼロ・文字列ソース）。
# 実 Rigor の Rigor::Testing.dump_type 相当。check が常に併載し、実行時は値を素通しする。
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

# check が dump_type を :info で印字する（フラグ不要・基本機能）
ds = Chibirigor.check("x = c ? 1 : \"a\"\ndump_type(x)\n")
dump = ds.find { |d| d[:kind] == :dump_type }
assert.call('dump_type emits a diagnostic', !dump.nil?, true)
assert.call('dump_type is :info severity', dump && dump[:severity], :info)
assert.call('dump_type prints the inferred type', dump && dump[:message], 'dump_type: 1 | "a"')
assert.call('dump_type points at its own line', dump && dump[:line], 2)

# :info なので「本物の型エラー」ではない（型エラーゼロのソースは error 段が空）
errors = Chibirigor.check("dump_type(\"a\".upcase)\n").reject { |d| d[:severity] == :info }
assert.call('dump_type alone raises no real error', errors, [])

# 実行時は値を素通し → その式の型は引数の型（annotate で確認）
types = Chibirigor.annotate("dump_type(\"a\".upcase)\n").map { |a| a[:type].to_s }
assert.call('dump_type passes the value type through', types[0], 'String')

# 本物の型エラーと共存できる（dump_type は exit に影響しないが診断は両方出る）
mixed = Chibirigor.check("dump_type(1)\n1 + \"x\"\n")
assert.call('real error still reported alongside dump_type', mixed.count { |d| d[:severity] != :info }, 1)
assert.call('dump_type info also present', mixed.count { |d| d[:kind] == :dump_type }, 1)

# 引数が 1 つでない dump_type は特別扱いしない（未知メソッド＝黙って untyped）
assert.call('dump_type with no args is not special', Chibirigor.check("dump_type()\n").any? { |d| d[:kind] == :dump_type }, false)

if failures.empty?
  puts "\nAll checks passed."
else
  puts "\n#{failures.size} failed."
  exit 1
end
