# frozen_string_literal: true

# Part 4 到達段階のスモークテスト。
# Union 型：if の両枝がまとまって Union になる。

require 'chibirigor'

failures = []
check = lambda do |desc, actual, expected|
  if actual == expected
    puts "PASS: #{desc}"
  else
    failures << desc
    puts "FAIL: #{desc} (expected #{expected.inspect}, got #{actual.inspect})"
  end
end

# 三項演算子の両枝が Union に
types = Chibirigor.annotate("x = c ? 1 : \"a\"\nx\n").map { |h| h[:type].to_s }
check.call('三項の両枝が Union に', types[0], '1 | "a"')
check.call('変数読み取りも Union を返す', types[1], '1 | "a"')

# nil との Union
types2 = Chibirigor.annotate("x = c ? 1 : nil\nx\n").map { |h| h[:type].to_s }
check.call('nil との Union', types2[0], '1 | nil')

# 同じ型の Union は単一に畳まれる
types3 = Chibirigor.annotate("x = c ? 1 : 2\nx\n").map { |h| h[:type].to_s }
# Const[1] と Const[2] は別 Const → Union[[1, 2]]
check.call('同型の Union は展開される', types3[0].include?('|'), true)

if failures.empty?
  puts 'All Part 4 stage checks passed.'
else
  warn "Part 4 FAILURES: #{failures.size}"
  exit 1
end
