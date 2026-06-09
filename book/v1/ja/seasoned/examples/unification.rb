# frozen_string_literal: true

# The Seasoned chibirigor Part 5 ― 制約ベース推論（単一化）の最小スケッチ。
# 依存ゼロ・単体で走る。`ruby unification.rb` で自己チェックが緑になる。
#
# 型変数 TVar（未知）と、基底型 TCon（:Integer など）だけの世界で、
# 「制約を集めて単一化で解く」＝ HM/型再構築の骨子を体感する。

TVar = Struct.new(:name) # 未知の型変数
TCon = Struct.new(:name) # 基底型（:Integer など）

# region unify
# 代入 subst で type をたどり、これ以上たどれない形まで解決する
def resolve(type, subst)
  type.is_a?(TVar) && subst.key?(type.name) ? resolve(subst[type.name], subst) : type
end

class UnifyError < StandardError; end

# a と b を等しくする代入を返す（できなければ UnifyError）
# 注：本スケッチは occurs-check を省略している（`unify(X, X->X)` のような自己参照が通ってしまう）。
# TVar/TCon のみの世界では関数型を作らないため自己チェックは緑のまま動くが、
# 本物の HM では occurs-check が停止性・健全性に必須。
def unify(a, b, subst)
  a = resolve(a, subst)
  b = resolve(b, subst)
  return subst if a == b
  return subst.merge(a.name => b) if a.is_a?(TVar) # 変数 a を b に束縛
  return subst.merge(b.name => a) if b.is_a?(TVar) # 変数 b を a に束縛

  raise UnifyError, "#{a.name} と #{b.name} は一致しない"
end

# 制約（= 等しくしたい型のペア）を順に単一化していく
def solve(constraints)
  constraints.reduce({}) { |subst, (a, b)| unify(a, b, subst) }
end
# endregion

def show(type, subst)
  resolve(type, subst).name.to_s
end

# --- 自己チェック -----------------------------------------------------------
if __FILE__ == $PROGRAM_NAME
  x = TVar.new(:X)
  n = TVar.new(:N)
  integer = TCon.new(:Integer)
  string = TCon.new(:String)

  results = {}

  # id(x) = x : 制約が無い → X は自由のまま（＝ジェネリック (X) -> X）
  id_subst = solve([])
  results['id has no constraint (X stays generic)'] = show(x, id_subst) == 'X'

  # inc(n) = n + 1 : 本体の n + 1 から「N は Integer」という制約 → N = Integer
  inc_subst = solve([[n, integer]])
  results['inc resolves N to Integer from n + 1'] = show(n, inc_subst) == 'Integer'

  # 矛盾：X = Integer かつ X = String は単一化できない（型エラー）
  conflicts = begin
    solve([[x, integer], [x, string]])
    false
  rescue UnifyError
    true
  end
  results['conflicting constraints raise UnifyError'] = conflicts

  results.each { |name, ok| puts "#{ok ? 'PASS' : 'FAIL'}: #{name}" }
  exit(results.values.all? ? 0 : 1)
end
