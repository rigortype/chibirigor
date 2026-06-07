# frozen_string_literal: true

# The Seasoned chibirigor Part 4 ― 再帰型の等価判定（μ＋余帰納）の動く設計スケッチ。
# 依存ゼロ・単体で走る。`ruby mu_typeeq.rb` で自己チェックが緑になる。
#
# 型の表現：基底型は Symbol（:Num など）、Obj はレコード、Rec は μ 型、Var は型変数。

Obj = Struct.new(:fields) # fields: Hash{Symbol => type}
Rec = Struct.new(:var, :body) # μ var. body
Var = Struct.new(:name)

def obj(fields) = Obj.new(fields)

# 型変数 var を repl で置換（shadowing は内側の Rec で止める）
def subst(type, var, repl)
  case type
  in Symbol then type
  in Obj then obj(type.fields.transform_values { subst(it, var, repl) })
  in Var then type.name == var ? repl : type
  in Rec then type.var == var ? type : Rec.new(type.var, subst(type.body, var, repl))
  end
end

# μ 型を一段ほどく：本体の自分自身（var）を、自分（rec）で置換する
def unfold(rec) = subst(rec.body, rec.var, rec)

# 束縛変数名の違いだけを吸収する α 同値（展開しない）。map は s の変数→t の変数。
def naive_eq(s, t, map = {})
  case [s, t]
  in [Symbol, Symbol] then s == t
  in [Obj, Obj]
    s.fields.size == t.fields.size &&
      s.fields.all? { |k, v| t.fields.key?(k) && naive_eq(v, t.fields[k], map) }
  in [Rec, Rec] then naive_eq(s.body, t.body, map.merge(s.var => t.var))
  in [Var, Var] then map.fetch(s.name, s.name) == t.name
  else false
  end
end

# 畳んだ形と展開した形を等しいと見なす等価判定。
# seen は「いま比較中のペア」。同じペアを再び問われたら true と仮定して止める（余帰納）。
def type_eq(s, t, seen = [])
  return true if seen.any? { |s2, t2| naive_eq(s2, s) && naive_eq(t2, t) }
  return type_eq(unfold(s), t, seen + [[s, t]]) if s.is_a?(Rec)
  return type_eq(s, unfold(t), seen + [[s, t]]) if t.is_a?(Rec)

  case [s, t]
  in [Symbol, Symbol] then s == t
  in [Obj, Obj]
    s.fields.size == t.fields.size &&
      s.fields.all? { |k, v| t.fields.key?(k) && type_eq(v, t.fields[k], seen) }
  else false
  end
end

# --- 自己チェック -----------------------------------------------------------
if __FILE__ == $PROGRAM_NAME
  mu_x = Rec.new(:X, obj({ foo: Var.new(:X) }))
  mu_y = Rec.new(:Y, obj({ foo: Var.new(:Y) })) # α 違いだけ
  unfolded = obj({ foo: Rec.new(:X, obj({ foo: Var.new(:X) })) }) # mu_x の一段展開
  mu_bar = Rec.new(:Y, obj({ bar: Var.new(:Y) })) # フィールド名違い
  stream1 = Rec.new(:X, obj({ n: :Num, rest: Var.new(:X) }))
  stream2 = obj({ n: :Num, rest: Rec.new(:Y, obj({ n: :Num, rest: Var.new(:Y) })) })

  checks = {
    'muX{foo:X} == muY{foo:Y} (α + cycle)' => type_eq(mu_x, mu_y) == true,
    'muX{foo:X} == {foo: muX{foo:X}} (fold/unfold)' => type_eq(mu_x, unfolded) == true,
    'muX{foo:X} != muY{bar:Y} (field name)' => type_eq(mu_x, mu_bar) == false,
    'stream fold == unfold (α)' => type_eq(stream1, stream2) == true,
    'Num == Num' => type_eq(:Num, :Num) == true,
    'Num != Bool' => type_eq(:Num, :Bool) == false
  }
  checks.each { |name, ok| puts "#{ok ? 'PASS' : 'FAIL'}: #{name}" }
  exit(checks.values.all? ? 0 : 1)
end
