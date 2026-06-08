# frozen_string_literal: true

# Part 7 到達段階のスモークテスト。
# 受理判定（三値）：:yes/:no/:maybe で Union を正しく捌く。

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

int   = Chibirigor::Type::Nominal[:Integer]
const = Chibirigor::Type::Const
dyn   = Chibirigor::Type::Dynamic
acc   = Chibirigor::Accepts

check.call('concrete match is yes',   acc.call(int, const[1]),   :yes)
check.call('concrete mismatch is no', acc.call(int, const['x']), :no)
check.call('dynamic is maybe',        acc.call(int, dyn.new),    :maybe)

# Union のうち全員が Integer → :yes（誤検知を消す）
check.call('union of integers is no longer a false positive',
           Chibirigor.check("x = c ? 1 : 2\n1 + x"), [])

# Union に String が混じる → :no → エラーが出る
check.call('union with a bad member is reported',
           Chibirigor.check("x = c ? 1 : \"a\"\n1 + x").size, 1)

# untyped は黙る
check.call('dynamic arg stays silent', Chibirigor.check('1 + foo.bar'), [])

if failures.empty?
  puts 'All Part 7 stage checks passed.'
else
  warn "Part 7 FAILURES: #{failures.size}"
  exit 1
end
