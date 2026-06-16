# frozen_string_literal: true

# The Seasoned chibirigor Part 2 - a runnable design sketch of subtyping `<:` and variance.
# Zero dependencies, runs standalone. `ruby subtype.rb` self-checks green.
#
# Types: base = Symbol (:Num etc.), Obj = record, Arrow = function, :Top = the maximal type.

Obj   = Struct.new(:fields)       # record: Hash{Symbol => type}
Arrow = Struct.new(:params, :ret) # function: [param types] -> ret
TOP = :Top

def obj(fields) = Obj.new(fields)

# region subtype
# s <: t ?  = "is it safe to pass an s where a t is expected?"
def subtype(s, t)
  return true if t == TOP

  case [s, t]
  in [Symbol, Symbol] then s == t # base types: reflexive only (no nontrivial subtyping)
  in [Obj, Obj] # width + depth: every key of t is in s, and values are covariant
    t.fields.all? { |k, tv| s.fields.key?(k) && subtype(s.fields[k], tv) }
  in [Arrow, Arrow] # params contravariant, return covariant
    s.params.size == t.params.size &&
      s.params.zip(t.params).all? { |sp, tp| subtype(tp, sp) } && # * tp/sp swapped = contravariant
      subtype(s.ret, t.ret)
  else false
  end
end
# endregion

# --- Self-check --------------------------------------------------------------
if __FILE__ == $PROGRAM_NAME
  name     = obj({ name: :Str })
  name_age = obj({ name: :Str, age: :Num })

  checks = {
    'width: {name,age} <: {name}' => subtype(name_age, name) == true,
    'width: {name} </: {name,age}' => subtype(name, name_age) == false,
    'depth: {p:{a,b}} <: {p:{a}}' =>
      subtype(obj({ p: obj({ a: :Num, b: :Num }) }), obj({ p: obj({ a: :Num }) })) == true,
    'arg contravariant: ({name})->Num <: ({name,age})->Num' =>
      subtype(Arrow.new([name], :Num), Arrow.new([name_age], :Num)) == true,
    'arg contravariant reverse is false' =>
      subtype(Arrow.new([name_age], :Num), Arrow.new([name], :Num)) == false,
    'ret covariant: ()->{name,age} <: ()->{name}' =>
      subtype(Arrow.new([], name_age), Arrow.new([], name)) == true,
    'Num <: Top' => subtype(:Num, TOP) == true,
    'Num </: Bool' => subtype(:Num, :Bool) == false
  }
  checks.each { |name, ok| puts "#{ok ? 'PASS' : 'FAIL'}: #{name}" }
  exit(checks.values.all? ? 0 : 1)
end
