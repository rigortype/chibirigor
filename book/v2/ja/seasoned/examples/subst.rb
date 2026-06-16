# frozen_string_literal: true

# The Seasoned chibirigor Part 3 - a runnable design sketch of type substitution subst (shadowing + capture avoidance).
# Zero dependencies, runs standalone. `ruby subst.rb` self-checks green.
#
# Types: base = Symbol, Var = type variable, Arrow = function, TypeAbs = type abstraction <params...> body.

TypeAbs = Struct.new(:params, :body)
Arrow   = Struct.new(:params, :ret)
Var     = Struct.new(:name)

# Fresh name using `@`, which a programmer can't write. Resettable so tests stay deterministic.
$fresh = 0
def fresh_name(base)
  $fresh += 1
  :"#{base}@#{$fresh}"
end

# region subst
# Substitute type variable x with repl inside type ty.
def subst(ty, x, repl)
  case ty
  in Symbol then ty
  in Var then ty.name == x ? repl : ty
  in Arrow then Arrow.new(ty.params.map { subst(it, x, repl) }, subst(ty.ret, x, repl))
  in TypeAbs
    return ty if ty.params.include?(x) # shadowing -> don't substitute inside this abstraction

    body = ty.body
    new_params = ty.params.map do |p| # alpha-rename the bound variables to fresh names first...
      np = fresh_name(p)
      body = subst(body, p, Var.new(np))
      np
    end
    TypeAbs.new(new_params, subst(body, x, repl)) # ...then the outer substitution (no capture)
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

# --- Self-check --------------------------------------------------------------
if __FILE__ == $PROGRAM_NAME
  results = {}

  # 1) shadowing: the inner <T> is a different thing -> don't substitute
  $fresh = 0
  shadow = TypeAbs.new([:T], Arrow.new([Var.new(:T)], :Bool))
  results['shadowing leaves the inner T untouched'] =
    show(subst(shadow, :T, :Num)) == '<T>(T) -> Bool'

  # 2) if the inner name <U> differs, the outer T is substituted and U is freshened
  $fresh = 0
  nonshadow = TypeAbs.new([:U], Arrow.new([Var.new(:T), Var.new(:U)], :Bool))
  results['non-shadowing substitutes T and freshens U'] =
    show(subst(nonshadow, :T, :Num)) == '<U@1>(Num, U@1) -> Bool'

  # 3) capture: repl=U collides with the inner <U> -> keep them distinct via a fresh name
  $fresh = 0
  foo_body = Arrow.new(
    [Var.new(:T), TypeAbs.new([:U], Arrow.new([Var.new(:T), Var.new(:U)], :Bool))], :Bool
  )
  results['capture is avoided (inner U becomes U@1, distinct from the substituted U)'] =
    show(subst(foo_body, :T, Var.new(:U))) == '(U, <U@1>(U, U@1) -> Bool) -> Bool'

  results.each { |name, ok| puts "#{ok ? 'PASS' : 'FAIL'}: #{name}" }
  exit(results.values.all? ? 0 : 1)
end
