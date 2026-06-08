# frozen_string_literal: true

# Part 2 到達段階のスモークテスト。
# ディスパッチ表で引数の型・数を見る。加算は表経由でも丸めて Integer のまま。

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

check.call('引数の型エラーを診断', Chibirigor.check('"a" + 1'),
           [{ line: 1, message: 'String が必要ですが 1 が渡されました' }])
check.call('引数の数エラーを診断', Chibirigor.check('"ab".length(1)'),
           [{ line: 1, message: 'length の引数の数が違います（0 個必要、1 個渡された）' }])
check.call('未知のレシーバ・メソッドは黙る', Chibirigor.check('foo.bar(1, 2)'), [])

ann = Chibirigor.annotate('1 + 2').map { |h| "#{h[:line]}: #{h[:type]}" }
check.call('加算は dispatch 経由でも丸めて Integer', ann, ['1: Integer'])

if failures.empty?
  puts 'All Part 2 stage checks passed.'
else
  warn "Part 2 FAILURES: #{failures.size}"
  exit 1
end
