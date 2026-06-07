# 【試し書き】The Little chibirigor Part 4 ― Union と絞り込み

> Part 6 の試し書きで「Union を `accepts` に渡すときの三値の優先順位は初学者に少し効く →
> Part 4 で先に地ならししておくと安全」という課題が出た。本章はその地ならしを兼ねた
> ナローイングの試し書き。コードは実 Ruby で動作確認済み。

この章のゴール：**型が一本に決まらないときの型 `Union` を導入し、`if` で型を「絞り込む」
仕組み（ナローイング）を作る。** Ruby のコードが当たり前にやっている「nil チェックしたから、
この先は nil じゃない」を、型でも追えるようにします。

> 本書『型システムのしくみ』には、この章にちょうど対応する章は*ありません*。本書は一般の
> 合併型（union）を「型システムへの影響が大きすぎる」として*あえて避けました*（5 章の演習で
> タグ付き union に少し触れる程度）。でも Ruby を相手にする私たちには Union は必須です。

---

## 4-1. 型が一本に決まらない ― Union

こんな Ruby を考えます：

```ruby
x = rand < 0.5 ? 1 : "a"
```

`x` の型は `Integer`？ `String`？ ── **どちらにもなり得る**。こういうとき、型を一本に
決めず「`Integer` か `String` のどちらか」という型にします。これが **Union**：

```ruby
Union = Data.define(:members) do
  def to_s = members.map(&:to_s).join(" | ")   # 例: "Integer | String"
end

# 型をまとめる小さな道具。入れ子をならし、重複を消す
def union(types)
  flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
  flat.size == 1 ? flat.first : Union[flat]
end
```

`if`（三項演算子も Prism では同じ `IfNode`）の型は、**then 節と else 節の型をまとめた
もの**にします：

```ruby
when Prism::IfNode
  then_type = type_of(node.statements.body.last, scope, diagnostics)
  else_type = type_of(node.subsequent.statements.body.last, scope, diagnostics)
  union([then_type, else_type])
```

```ruby
type_of(parse("rand < 0.5 ? 1 : \"a\""))   # => Integer | String
```

- **① 型理論**：値が複数の型になり得るとき＝合併型（本書はあえて避けた領域）。
- **② Ruby だと**：分岐で別々の型を返すのは日常。`x = cond ? 1 : "a"` は普通に書く。
- **③ Rigor だと**：一本に決めず Union で持つ。決めつけない＝後で困らない。

---

## 4-2. 場合分けで型を絞る ― ナローイング

Union ができると、すぐ次の欲が出ます。こういう Ruby を見てください：

```ruby
x = find_user   # 型は User | nil（見つからなければ nil）
if x.nil?
  puts "いません"
else
  puts x.name   # ここでは x は絶対 nil じゃない → User
end
```

人間は当たり前に「`else` の中では `x` は `nil` じゃない」と読めます。これを型でも追うのが
**ナローイング（絞り込み）**。条件分岐の枝ごとに、変数の型を*狭める*のです。

- `if x.nil?` の **then 節**では、`x` は `nil`。
- **else 節**では、`x` は `nil` を除いた残り（`User | nil` → `User`）。

---

## 4-3. Ruby の「偽」は 2 つだけ ― 絞り込みを実装する

実装の前に、Ruby の大事な事実を一つ。**Ruby で「偽」とみなされるのは `false` と `nil` の
2 つだけ**。`0` も `""` も真です。だから `if x` は「`x` が `false` でも `nil` でもない」を
意味します。

絞り込みは「条件を見て、枝ごとに変数の型を差し替えた**新しいスコープ**を作る」だけです。
スコープは Part 3 で作った「変数名 → 型」の対応（ここでは素朴に Hash）：

```ruby
def remove_nil(t)
  return t unless t.is_a?(Union)
  union(t.members.reject { |m| m == Nominal[:NilClass] })
end

def narrow(scope, cond, truthy:)
  # まずは `x.nil?` の形だけ扱う（他の条件は後で同じ要領で増やせる）
  if cond.is_a?(Prism::CallNode) && cond.name == :nil? &&
     cond.receiver.is_a?(Prism::LocalVariableReadNode)
    name = cond.receiver.name
    narrowed = truthy ? Nominal[:NilClass] : remove_nil(scope[name])
    return scope.merge(name => narrowed)
  end
  scope   # ★ 絞れない条件は、スコープをそのまま返す（何も主張しない）
end
```

`if` の型付けは、then 節を「真に絞ったスコープ」で、else 節を「偽に絞ったスコープ」で
それぞれ求め、最後にまとめます：

```ruby
when Prism::IfNode
  then_scope = narrow(scope, node.predicate, truthy: true)
  else_scope = narrow(scope, node.predicate, truthy: false)
  then_type = type_of(node.statements.body.last, then_scope, diagnostics)
  else_type = type_of(node.subsequent.statements.body.last, else_scope, diagnostics)
  union([then_type, else_type])
```

動かすと、ちゃんと絞れます：

```ruby
# x : Integer | nil のとき
# then 節 → x は NilClass
# else 節 → x は Integer
```

`is_a?` でも同じ要領です（`if x.is_a?(String)` の then 節は `x` を `String` に絞る）。
形が増えても `narrow` に分岐を足すだけ。

ただし `is_a?` には**落とし穴**が一つ。`x` がもともと `Integer` のとき
`if x.is_a?(String)` の中身を「`x` は `String`」と絞ると、その枝は*起き得ない*（Integer は
String にならない）のに `x + 1` を String の足し算とみなして**誤検知**します。これは
「動くコードを脅かさない」に反します。だから **「そのクラスがあり得るときだけ絞る」** ―
`x` が `Integer | String` のように String を含むときは絞る、`Integer` 単体なら絞らない
（その枝は dead branch なので触らない）。`Dynamic` も絞りません（Rigor も post-guard の
`Dynamic → C` narrowing は誤検知が多いとして採らない）。

```ruby
def possible?(current, klass)
  return false if current.is_a?(Type::Dynamic)
  members = current.is_a?(Type::Union) ? current.members : [current]
  members.any? { |m| Dispatch.class_of(m) == klass }
end
# narrow_type の is_a? 節：klass && truthy && possible?(current, klass) のときだけ絞る
```

```ruby
check("x = 1\nif x.is_a?(String)\n x + 1\nend\n")              # OK（dead branch、誤検知しない）
check("x = c ? 1 : \"a\"\nif x.is_a?(String)\n x + 1\nend\n")  # String の足し算エラー（正しい）
```

---

## 4-4. 絞り込みの 2 つの掟（ここが Rigor らしさ）

ナローイングには、Rigor が守っている掟が 2 つあります。どちらも「脅かさない」ためです。

**掟その 1：絞れない条件は、黙ってそのまま通す。** `narrow` の最後の行 ―
`scope`（そのまま返す）― がそれです。`if complicated_check(x)` のような、私たちに読めない
条件のときは、**何も主張しません**。「絞れないから怪しい」とは言わない。

**掟その 2：絞り込みは「事実を足す」だけ。間違えたら緩める側に倒す。** 型を*狭める*操作なので、
やりすぎると「本当はあり得る値」を消してしまい、誤検知の元になります。だから迷ったら絞らない。

> **Part 6 への地ならし：Union は「全メンバで考える」**
> Union から何かを読むとき（例：`(Integer | String).to_s`）は、**メンバを 1 つずつ考えて
> まとめる**のが基本です。`to_s` は Integer にも String にもあるので OK。もし片方にしか無い
> メソッドなら、その分だけ怪しくなる。── この「**全メンバを回して一番弱い結論を採る**」考え方は、
> 次々章 Part 6 の `accepts`（`:yes`/`:no`/`:maybe`）でそのまま再登場します。ここで体に
> 入れておいてください。

- **① 型理論**：場合分けで型情報が増える（本書は扱わない独自地形）。
- **② Ruby だと**：`false`/`nil` だけが偽、`x.nil?`/`is_a?` でガードするのが定石。さらに ―
  `x` が*局所変数*かどうかは「先に代入があるか」で決まる（無ければ `self.x` の呼び出し扱い）。[^bare]
- **③ Rigor だと**：絞り込みは*事実を足すだけ*。読めない条件は黙る。間違えるなら緩める側に。

[^bare]: この「裸の `x` が局所変数かメソッド呼び出しか」は Prism が文脈で決めます。ナローイングは
局所変数にしか効かないので、実 Rigor もここを丁寧に見分けています。本編では深追いしません。

---

## 4-5. この章のまとめ

足したもの：型カリア `Union` ひとつ、道具 `union`／`remove_nil`／`narrow`、そして `IfNode` の
型付け。`narrow` は実質 7 行。スコープは Part 3 の Hash をそのまま `merge` で増やすだけで、
**実 Rigor の凝った FactStore はまだ出していません**。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論 | 値が複数の型になり得る＝合併（本書が*あえて避けた*領域） |
| ② Ruby/RBS | 偽は `false`/`nil` だけ、`x.nil?`/`is_a?` でガードが定石 |
| ③ Rigor 実装の問題 | 絞り込みは*事実を足すだけ・読めなければ黙る・迷えば緩める*＝誤検知を出さない |

**続編に送ったもの**：

- 本物の **FactStore**（6 種類の「事実の置き場」、いつ事実が無効になるか、再代入やブロックの
  クロージャ捕獲で事実を捨てる機微）。本編は素朴な Hash 止まり。
- `case`/`when`・`case`/`in`（パターンマッチ）の絞り込みと、到達しない枝の検出（実 Rigor の
  ADR-47）。本編は `if` の `nil?`/`is_a?` まで。

**次章予告（Part 5）**：ハッシュや配列のリテラルに型をつけます（`HashShape`/`Tuple`）。
「symbol キーのオプションハッシュ」だらけの Ruby で、型を*完全一致で要求すると誤検知の嵐に
なる*話に踏み込みます。

---

> **検証メモ（この試し書きの自己評価）**
> - 地ならし目的は達成：4-4 の囲みで「Union は全メンバで考える」を*先に*入れられた。Part 6 の
>   `accepts`-over-Union が「ここで見たやつ」になる。○
> - トーン：「合併型」「ナローイング」は普通の言葉（型が一本に決まらない／絞り込み）で導入でき、
>   用語は後置。FactStore は名前だけ出して続編送り。○
> - 複雑さ予算：新カリアは `Union` 1 つ。`narrow` は `nil?` の 1 形だけに絞り、「is_a? も同じ
>   要領」で逃がした。スコープは Hash の `merge`。1 step 1 難所を維持。○
> - 三題噺：③「足すだけ・黙る・緩める」が、②（Ruby の偽は 2 つ・ガードが定石）から自然に出た。○
> - 発見：②に「裸の x が局所変数か否か」という Ruby 固有のワナを脚注で 1 つ拾えた。実 Rigor の
>   苦労が透けるので、③の説得力が増す良い小ネタ。本番でも残す価値あり。
> - 結論：Part 6 で見つけた地ならし案は機能する。Part 4 → 6 の順序依存を明示的に設計に残す。
