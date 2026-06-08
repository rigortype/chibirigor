# frozen_string_literal: true

# Part 3 到達段階のスモークテスト。
# ローカル変数と不変 Scope。代入で型が育ち、再代入で型が変わる。

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

check.call('代入した変数は後で使える', Chibirigor.check("x = 1\nx + 2"), [])
check.call('変数の型がエラーを運ぶ', Chibirigor.check("x = \"a\"\nx + 1").size, 1)
check.call('再代入で型が変わる', Chibirigor.check("x = 1\nx = \"a\"\nx + 1").size, 1)

ann = Chibirigor.annotate("x = 1\nx\nx = \"a\"\nx\n").map { |h| h[:type].to_s }
check.call('代入の型は値そのもの', ann[0], '1')
check.call('変数読み取りは代入された型', ann[1], '1')
check.call('再代入後の型', ann[3], '"a"')

check.call('未束縛の変数は黙る', Chibirigor.check('y + 1'), [])

if failures.empty?
  puts 'All Part 3 stage checks passed.'
else
  warn "Part 3 FAILURES: #{failures.size}"
  exit 1
end
