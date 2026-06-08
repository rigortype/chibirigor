# frozen_string_literal: true

# The Seasoned chibirigor Part 2 ― 部分型 `<:` と変性の、動く設計スケッチ。
# 依存ゼロ・単体で走る。`ruby subtype.rb` で自己チェックが緑になる。
#
# 型：基底＝Symbol（:Num など）、Obj＝レコード、Arrow＝関数、:Top＝最大型。

Obj   = Struct.new(:fields)       # record: Hash{Symbol => type}
Arrow = Struct.new(:params, :ret) # function: [param types] -> ret
TOP = :Top

def obj(fields) = Obj.new(fields)

# region subtype
# s <: t ?  ＝「t が要る所に s を渡しても安全か」
def subtype(s, t)
  return true if t == TOP

  case [s, t]
  in [Symbol, Symbol] then s == t # 基底は反射のみ（非自明な部分型なし）
  in [Obj, Obj] # 幅＋深さ：t の各キーが s にあり、値は共変
    t.fields.all? { |k, tv| s.fields.key?(k) && subtype(s.fields[k], tv) }
  in [Arrow, Arrow] # 引数は反変・戻りは共変
    s.params.size == t.params.size &&
      s.params.zip(t.params).all? { |sp, tp| subtype(tp, sp) } && # ★ tp/sp 入れ替え＝反変
      subtype(s.ret, t.ret)
  else false
  end
end
# endregion

# --- 自己チェック -----------------------------------------------------------
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
