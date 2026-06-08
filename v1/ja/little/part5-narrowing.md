---
title: The Little chibirigor Part 5 ― ナローイング：場合分けで絞る
description: "Union で増えた型を、`if`/`case` の枝ごとに絞り込む（ナローイング）。`narrow` の実装と、絞り込みの 2 つの掟を作る。"
sidebar:
  order: 6
---

# The Little chibirigor Part 5 ― ナローイング：場合分けで絞る

この章のゴール：**前章で型が「一本に決まらない」Union を手に入れた。今度は `if`/`case` の
枝ごとに、変数の型を*絞る*仕組み（ナローイング）を作る。** Ruby のコードが当たり前に
やっている「nil チェックしたから、この先は nil じゃない」を、型でも追えるようにします。

前章（Part 4）で Union を導入したことで、ひとつの変数が `User | nil` のように複数の型を
持つようになりました。型が増えたなら、次は**減らす**番です（前章で書いた `IfNode` の型付けに、
枝ごとの絞り込みを足していきます）。場合分け（`if`/`case`）の
枝の中では、人間は無意識に「この枝なら型はこれ」と読んでいます。それを型でも追うのが、
この章の主題です。

---

## 5-1. 場合分けで型を絞る ― ナローイング

こういう Ruby を見てください：

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

```text
              x : User | nil
                    │
            if x.nil?
          ┌─────────┴─────────┐
       then 節               else 節
     x : nil          x : User （nil を除く）
          └─────────┬─────────┘
              両枝の型を union
```

> ▼ 図 5-1　`if x.nil?` のナローイング（`[図: 後で清書]`）

枝ごとに `x` の型を差し替えた**別の Scope**で本体を型付けし、最後に両枝の結果を union します。

> **コラム：これは「ぬるぽ」を型で捕まえる話です**
>
> Java を書く人には、上の `User | nil` は見覚えがあるはずです。`find_user` が「見つかれば
> `User`、なければ `nil`」を返す ― これは Java の「`User` か `null`」と同じ構図で、Ruby の
> `nil` は Java の `null` に当たります。そして `x.name` を `x` が `nil` のまま呼べば、Ruby なら
> `NoMethodError`、Java なら `NullPointerException`（**ぬるぽ**）― 同じ事故です。
>
> ナローイングは、その事故を**型のレベルで先回りして捕まえる**仕組みです。`if x.nil?` の
> else 節で「ここの `x` はもう `nil` じゃない」と型を狭める ― これは Java で `if (x != null) { … }`
> と書いてからフィールドに触る、あの習慣を、**型チェッカーが自動で追ってくれる**のと同じこと。
> 「`nil` を含む Union」を持ち歩き、ガードを通った所で `nil` を**剥がす**。剥がし切れていない
> （`nil` がまだ型に残っている）場所で `.name` を呼べば、そこが「ぬるぽが出る場所」です。
>
> Java の `NullPointerException` を「実行時にたまたま落ちるもの」と捉えていた人にとって、
> ここは見方が変わる所です ― **null は型で表現でき、型で防げるバグ**だった。これが
> 「null 安全（null safety）」と呼ばれる考え方の芯で、Kotlin の `User?`、TypeScript の
> `User | null` も同じ発想です。Rigor／chibirigor では `nil` をただの Union のメンバとして
> 持ち、ナローイングで剥がす ― 特別扱いの構文を足さずに null 安全の入口に立てるわけです。

---

## 5-2. Ruby の「偽」は 2 つだけ ― 絞り込みを実装する

実装の前に、Ruby の大事な事実を一つ。**Ruby で「偽」とみなされるのは `false` と `nil` の
2 つだけ**。`0` も `""` も真です。だから `if x` は「`x` が `false` でも `nil` でもない」を
意味します。

絞り込みは「条件を見て、枝ごとに変数の型を差し替えた**新しいスコープ**を作る」だけです。
スコープは Part 3 で作った不変 `Scope`（`scope.local(名前)` で型を引き、`scope.with_local(名前, 型)`
で束縛を 1 つ足した新しい `Scope` を返す）をそのまま使います：

```ruby
def remove_nil(t)
  return t unless t.is_a?(Type::Union)
  Type.union(t.members.reject { |m| m == Type::Nominal[:NilClass] })
end

def narrow(scope, cond, truthy:)
  # まずは `x.nil?` の形だけ扱う（他の条件は後で同じ要領で増やせる）
  if cond.is_a?(Prism::CallNode) && cond.name == :nil? &&
     cond.receiver.is_a?(Prism::LocalVariableReadNode)
    name = cond.receiver.name
    narrowed = truthy ? Type::Nominal[:NilClass] : remove_nil(scope.local(name))
    return scope.with_local(name, narrowed)   # 不変 Scope に束縛を足して返す
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
  else_type =
    if node.subsequent   # else 節がある（三項演算子も同じ IfNode）
      type_of(node.subsequent.statements.body.last, else_scope, diagnostics)
    else
      Type::Const[nil]   # else が無ければ、偽のとき nil
    end
  Type.union([then_type, else_type])
```

（`if cond; ...; end` のように **else が無い** とき `node.subsequent` は `nil` です。その場合は
偽の枝の型を `nil` とします ― 実際の Ruby が、else 無しの `if` が偽のとき `nil` を返すのに
合わせています。）

動かすと、ちゃんと絞れます：

```ruby
# x : Integer | nil のとき
# then 節 → x は NilClass
# else 節 → x は Integer
```

`is_a?` でも同じ要領です（`if x.is_a?(String)` の then 節は `x` を `String` に絞る）。
形が増えても `narrow` に分岐を足すだけ。

ただし `is_a?` には落とし穴が一つ。`x` がもともと `Integer` のとき
`if x.is_a?(String)` の中身を「`x` は `String`」と絞ると、その枝は*起き得ない*（Integer は
String にならない）のに `x + 1` を String の足し算とみなして**誤検知**します。これは
「動くコードを脅かさない」に反します。だから **「そのクラスがあり得るときだけ絞る」** ―
`x` が `Integer | String` のように String を含むときは絞る、`Integer` 単体なら絞らない
（その枝は dead branch なので触らない）。`Dynamic` も絞りません（Rigor も post-guard の
`Dynamic → C` narrowing は誤検知が多いとして採らない）。

```ruby
check("x = 1\nif x.is_a?(String)\n x + 1\nend\n")              # OK（dead branch、誤検知しない）
check("x = c ? 1 : \"a\"\nif x.is_a?(String)\n x + 1\nend\n")  # String の足し算エラー（正しい）
```

> **実装メモ ― `possible?` ガード**　「そのクラスがあり得るか」を `narrow` に判定させるには
> 小さなヘルパが要ります。`Dynamic` は「あり得ないとは言えない」ので false、Union は
> メンバを探索、それ以外はクラスが一致するかで判定します：
>
> ```ruby
> def possible?(current, klass)
>   return false if current.is_a?(Type::Dynamic)
>   members = current.is_a?(Type::Union) ? current.members : [current]
>   members.any? { |m| Dispatch.class_of(m) == klass }
> end
> # narrow_type の is_a? 節：klass && truthy && possible?(current, klass) のときだけ絞る
> ```
>
> このガードを入れないと、`Integer` 単体に `is_a?(String)` を当てたとき dead branch を
> `String` に絞ってしまい、`x + 1` が「String の足し算」として誤検知されます。

### 到達できない枝の報告 ― Java/C# 網羅性との方向の違い

`is_a?` の dead branch を「触らない（絞らない）」のは誤検知を避ける消極的な対応ですが、
Rigor はもう一歩進めて、その枝を**積極的に報告**します。ここで Java・C# との向きの違いが
はっきり出ます。

Java・C# の `switch`/`pattern matching` は**網羅性**を強制します。`case` が全パターンを
網羅していないと、コンパイラが「*missing arm*（腕が足りない）」として止めます。Rigor（と
chibirigor）は逆の方向を向いています。「足りない」は問いません。代わりに「**到達できない枝**
（unreachable arm）」を報告します。`x : Integer` なのに `if x.is_a?(String)` を書いたとき、
その枝は絶対に実行されません ― そこを見つけて「余分な分岐です」と伝えます（ADR-47）。

| | Java / C# | Rigor / chibirigor |
|---|---|---|
| 何を報告するか | missing arm（網羅しない腕） | unreachable arm（絶対に通らない腕） |
| 動くコードへの態度 | 書くまで止める | 動くものには黙る |
| 誰が損をするか | 「そのパターンは来ない」と知っている開発者 | 「来ないと思っているが実は来る」バグ |

これは健全性（全パターンを押さえる）より誤検知の少なさ（動くコードを脅かさない）を
優先する Rigor の価値観の現れです。上の `possible?` ガードで「あり得ないときは絞らない」と
した判断と、同じ軸の上にあります。

> 到達できない枝の「型」（ボトム型 `Bot`／`never`）については、付録 a1「特別な型カタログ」で
> 扱います。

---

## 5-3. 絞り込みの 2 つの掟（ここが Rigor らしさ）

ナローイングには、Rigor が守っている掟が 2 つあります。どちらも「脅かさない」ためです。

**掟その 1：絞れない条件は、黙ってそのまま通す。** `narrow` の最後の行 ―
`scope`（そのまま返す）― がそれです。`if complicated_check(x)` のような、私たちに読めない
条件のときは、**何も主張しません**。「絞れないから怪しい」とは言わない。

**掟その 2：絞り込みは「事実を足す」だけ。間違えたら緩める側に倒す。** 型を*狭める*操作なので、
やりすぎると「本当はあり得る値」を消してしまい、誤検知の元になります。だから迷ったら絞らない。
なお、変数への**再代入**はそれ以前の全 facts をリセットします ― 事実は「変数名」ではなく
「そのスコープ位置で確定した事実」に結びついているからです。`x = something_else` を書いた
瞬間、`x` に関する narrowing の記憶は全て消えます。

> **Part 7 への地ならし：Union は「全メンバで考える」**
> Union から何かを読むとき（例：`(Integer | String).to_s`）は、メンバを 1 つずつ考えて
> まとめるのが基本です。`to_s` は Integer にも String にもあるので OK。もし片方にしか無い
> メソッドなら、その分だけ怪しくなる。── この「**全メンバを回して一番弱い結論を採る**」考え方は、
> Part 7 の `accepts`（`:yes`/`:no`/`:maybe`）でそのまま再登場します。ここで
> 身につけておいてください。

- **① 型理論**：場合分けで型情報が増える（『しくみ』は扱わない独自地形）。
- **② Ruby だと**：`false`/`nil` だけが偽、`x.nil?`/`is_a?` でガードするのが定石。さらに ―
  `x` が*局所変数*かどうかは「先に代入があるか」で決まる（無ければ `self.x` の呼び出し扱い）。[^bare]
- **③ Rigor だと**：絞り込みは*事実を足すだけ*。読めない条件は黙る。間違えるなら緩める側に。

[^bare]: この「裸の `x` が局所変数かメソッド呼び出しか」は Prism が文脈で決めます。ナローイングは
局所変数にしか効かないので、実 Rigor もここを丁寧に見分けています。本編では深追いしません。

---

## 5-4. この章のまとめ

足したもの：道具 `remove_nil`／`narrow`、そして `IfNode` の絞り込み付き型付け。`narrow` は
実質 7 行。スコープは Part 3 の不変 `Scope` に `with_local` で束縛を足すだけで、
**実 Rigor の凝った FactStore はまだ出していません**。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論 | 場合分けで型情報が増える（『しくみ』が扱わない独自地形。dead branch ＝ボトム型は付録 a1） |
| ② Ruby/RBS | 偽は `false`/`nil` だけ、`x.nil?`/`is_a?` でガードが定石 |
| ③ Rigor 実装の問題 | 絞り込みは*事実を足すだけ・読めなければ黙る・迷えば緩める*＝誤検知を出さない |

**続編に送ったもの**：

- 本物の **FactStore**（6 種類の「事実の置き場」、いつ事実が無効になるか、再代入やブロックの
  クロージャ捕獲で事実を捨てる機微）。本編は素朴な `Scope` 止まり。この章で触れた「再代入で
  facts が消える」話は、後編 Part 6（完全な FactStore）で一般化します。
- `case`/`when`・`case`/`in`（パターンマッチ）の絞り込みと、到達しない枝の検出（実 Rigor の
  ADR-47）。本編は `if` の `nil?`/`is_a?` まで。
- **Union のサイズ予算**：前章の `union` ヘルパは重複を消すだけですが、実 Rigor では Union の
  メンバ数が設定上限を超えると、各メンバの名前的クラス（`Integer`・`String` など）の Union に
  **強制 wide（広げ）**します。これは定数畳み込みの「大きすぎたら丸める」と同じ発想 ―
  「型も*予算を持つ*」という設計原則の別の現れです。

## 演習

1. `x : String | nil` のとき `if x` の then 節で `x` が `String` に絞れることを確かめよ
   （Ruby の偽は `false`/`nil` の 2 つだけ、を使う）。else 節では `x` は何型か。
2. `x : Integer | String` のとき `if x.is_a?(Integer)` の then 節で `x` が `Integer` に絞れる
   ことを確かめよ。
3. `unless` も絞り込めるようにするには、`if` の型付けをどう変えればよいか方針を述べよ
   （ヒント：真の枝と偽の枝を入れ替える）。

---

**次章予告（Part 6）**：ハッシュや配列のリテラルに型をつけます（`HashShape`/`Tuple`）。
「symbol キーのオプションハッシュ」だらけの Ruby で、型を*完全一致で要求すると誤検知の嵐に
なる*話に踏み込みます。
