# frozen_string_literal: true

# The Seasoned chibirigor Part 3 ― 型代入 subst（シャドーイング＋変数捕獲回避）の動く設計スケッチ。
# 依存ゼロ・単体で走る。`ruby subst.rb` で自己チェックが緑になる。
#
# 型：基底＝Symbol、Var＝型変数、Arrow＝関数、TypeAbs＝型抽象 <params...> body。

TypeAbs = Struct.new(:params, :body)
Arrow   = Struct.new(:params, :ret)
Var     = Struct.new(:name)

# 「プログラマが書けない」@ を使った新名。テストの決定性のため明示リセットできる形に。
$fresh = 0
def fresh_name(base)
  $fresh += 1
  :"#{base}@#{$fresh}"
end

# region subst
# 型 ty の中の型変数 x を repl で置換する。
def subst(ty, x, repl)
  case ty
  in Symbol then ty
  in Var then ty.name == x ? repl : ty
  in Arrow then Arrow.new(ty.params.map { subst(it, x, repl) }, subst(ty.ret, x, repl))
  in TypeAbs
    return ty if ty.params.include?(x) # シャドーイング → その抽象の中は置換しない

    body = ty.body
    new_params = ty.params.map do |p| # 束縛変数を fresh に α 変換してから…
      np = fresh_name(p)
      body = subst(body, p, Var.new(np))
      np
    end
    TypeAbs.new(new_params, subst(body, x, repl)) # …外側の置換（捕獲が起きない）
  end
end
# endregion

def show(ty)
  case ty
  in Symbol then ty.to_s
  in Var then ty.name.to_s
  in Arrow then "(#{ty.params.map { show(it) }.join(', ')}) -> #{show(ty.ret)}"
  in TypeAbs then "<#{ty.params.join(', ')}>#{show(ty.body)}"
  end
end

# --- 自己チェック -----------------------------------------------------------
if __FILE__ == $PROGRAM_NAME
  results = {}

  # 1) シャドーイング：内側 <T> は別物 → 置換しない
  $fresh = 0
  shadow = TypeAbs.new([:T], Arrow.new([Var.new(:T)], :Bool))
  results['shadowing leaves the inner T untouched'] =
    show(subst(shadow, :T, :Num)) == '<T>(T) -> Bool'

  # 2) 内側が別名 <U> なら外側 T は置換され、U は fresh 化
  $fresh = 0
  nonshadow = TypeAbs.new([:U], Arrow.new([Var.new(:T), Var.new(:U)], :Bool))
  results['non-shadowing substitutes T and freshens U'] =
    show(subst(nonshadow, :T, :Num)) == '<U@1>(Num, U@1) -> Bool'

  # 3) 変数捕獲：repl=U が内側 <U> と衝突 → fresh で別物に保つ
  $fresh = 0
  foo_body = Arrow.new(
    [Var.new(:T), TypeAbs.new([:U], Arrow.new([Var.new(:T), Var.new(:U)], :Bool))], :Bool
  )
  results['capture is avoided (inner U becomes U@1, distinct from the substituted U)'] =
    show(subst(foo_body, :T, Var.new(:U))) == '(U, <U@1>(U, U@1) -> Bool) -> Bool'

  results.each { |name, ok| puts "#{ok ? 'PASS' : 'FAIL'}: #{name}" }
  exit(results.values.all? ? 0 : 1)
end
