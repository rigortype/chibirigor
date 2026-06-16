# frozen_string_literal: true

# The Seasoned chibirigor Part 5 - a minimal sketch of constraint-based inference (unification).
# Zero dependencies, runs standalone. `ruby unification.rb` self-checks green.
#
# In a world of only type variables TVar (unknown) and base types TCon (:Integer etc.),
# "collect constraints and solve by unification" gives a feel for the core of HM / type reconstruction.

TVar = Struct.new(:name) # unknown type variable
TCon = Struct.new(:name) # base type (:Integer etc.)

# region unify
# Follow type through the substitution subst, resolving until it can't be followed further.
def resolve(type, subst)
  type.is_a?(TVar) && subst.key?(type.name) ? resolve(subst[type.name], subst) : type
end

class UnifyError < StandardError; end

# Return a substitution that makes a and b equal (or raise UnifyError if impossible).
# Note: this sketch omits the occurs-check (a self-reference like `unify(X, X->X)` would slip through).
# Since a TVar/TCon-only world never builds function types, the self-check stays green,
# but in real HM the occurs-check is essential for termination and soundness.
def unify(a, b, subst)
  a = resolve(a, subst)
  b = resolve(b, subst)
  return subst if a == b
  return subst.merge(a.name => b) if a.is_a?(TVar) # bind variable a to b
  return subst.merge(b.name => a) if b.is_a?(TVar) # bind variable b to a

  raise UnifyError, "#{a.name} and #{b.name} don't match"
end

# Unify the constraints (= pairs of types we want to make equal) one by one.
def solve(constraints)
  constraints.reduce({}) { |subst, (a, b)| unify(a, b, subst) }
end
# endregion

def show(type, subst)
  resolve(type, subst).name.to_s
end

# --- Self-check --------------------------------------------------------------
if __FILE__ == $PROGRAM_NAME
  x = TVar.new(:X)
  n = TVar.new(:N)
  integer = TCon.new(:Integer)
  string = TCon.new(:String)

  results = {}

  # id(x) = x : no constraints -> X stays free (= generic (X) -> X)
  id_subst = solve([])
  results['id has no constraint (X stays generic)'] = show(x, id_subst) == 'X'

  # inc(n) = n + 1 : the body's n + 1 yields the constraint "N is Integer" -> N = Integer
  inc_subst = solve([[n, integer]])
  results['inc resolves N to Integer from n + 1'] = show(n, inc_subst) == 'Integer'

  # conflict: X = Integer and X = String can't be unified (type error)
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
