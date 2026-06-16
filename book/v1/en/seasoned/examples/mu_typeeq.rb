# frozen_string_literal: true

# The Seasoned chibirigor Part 4 - a runnable design sketch of recursive-type equality (μ + coinduction).
# Zero dependencies, runs standalone. `ruby mu_typeeq.rb` self-checks green.
#
# Type representation: base types are Symbol (:Num etc.), Obj is a record, Rec is a μ type, Var is a type variable.

Obj = Struct.new(:fields) # fields: Hash{Symbol => type}
Rec = Struct.new(:var, :body) # μ var. body
Var = Struct.new(:name)

def obj(fields) = Obj.new(fields)

# Substitute type variable var with repl (shadowing stops at an inner Rec).
def subst(type, var, repl)
  case type
  in Symbol then type
  in Obj then obj(type.fields.transform_values { subst(it, var, repl) })
  in Var then type.name == var ? repl : type
  in Rec then type.var == var ? type : Rec.new(type.var, subst(type.body, var, repl))
  end
end

# Unfold a μ type one step: substitute its own variable (var) in the body with itself (rec).
def unfold(rec) = subst(rec.body, rec.var, rec)

# Alpha-equivalence absorbing only bound-variable name differences (no unfolding). map is s's var -> t's var.
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

# region type_eq
# Equality that treats a folded form and its unfolded form as equal.
# seen holds the pairs currently under comparison. If the same pair is asked again, assume true and stop (coinduction).
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
# endregion

# --- Self-check --------------------------------------------------------------
if __FILE__ == $PROGRAM_NAME
  mu_x = Rec.new(:X, obj({ foo: Var.new(:X) }))
  mu_y = Rec.new(:Y, obj({ foo: Var.new(:Y) })) # differs only by α
  unfolded = obj({ foo: Rec.new(:X, obj({ foo: Var.new(:X) })) }) # one-step unfolding of mu_x
  mu_bar = Rec.new(:Y, obj({ bar: Var.new(:Y) })) # differs by field name
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
