---
title: Part 5　ナローイング：場合分けで絞る
description: "Union で増えた型を、`if`/`case` の枝ごとに絞り込む（ナローイング）。`narrow` の実装と、絞り込みの 2 つの掟を作る。"
sidebar:
  order: 6
---

# The Little chibirigor Part 5　ナローイング：場合分けで絞る

前章（Part 4）でUnionを導入したことで、ひとつの変数が`User | nil`のように複数の型を持つようになりました。
型が増えたなら、次は**減らす**番です。
`if`/`case`の枝の中では、人間は無意識に「この枝なら型はこれ」と読んでいます。
それを型でも追うのが、この章の主題です（前章で書いた`IfNode`の型付けに、枝ごとの絞り込みを足していきます）。

---

## 5-1. 場合分けで型を絞る（ナローイング）

こういうRubyを見てください。

```ruby
x = find_user   # 型は User | nil（見つからなければ nil）
if x.nil?
  puts "いません"
else
  puts x.name   # ここでは x は絶対 nil じゃない → User
end
```

人間は当たり前に「`else`の中では`x`は`nil`じゃない」と読めます。
これを型でも追うのが**ナローイング（絞り込み）**です。
条件分岐の枝ごとに、変数の型を*狭める*のです。

- `if x.nil?`の**then節**では、`x`は`nil`。
- **else節**では、`x`は`nil`を除いた残り（`User | nil`から`User`）。

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

![図5-1　if x.nil? のナローイング](../figures/svg/little-5-1.svg)
> ▼ 図5-1　`if x.nil?`のナローイング

枝ごとに`x`の型を差し替えた**別のScope**で本体を型付けし、最後に両枝の結果をunionします。

> [!NOTE]
> **これは「nilで落ちる」を型で捕まえる話です**
>
> `User | nil`は「`nil`を含むUnion」です。
> ナローイングは、`if x.nil?`のelse節で「ここの`x`はもう`nil`じゃない」と型から`nil`を**剥がす**仕組みです。
> 剥がし切れていない（`nil`がまだ型に残っている）場所で`.name`を呼べば、そこが「`nil`で落ちる場所」です。
> ここで見方がひとつ変わります。
> `nil`で落ちるバグは「実行時にたまたま落ちるもの」ではなく、**型で表現でき、型で防げるバグ**だったのです。
> この見方の転換が「**null安全（null safety）**」と呼ばれるものです。
> 特別な構文を足さず、`nil`をただのUnionのメンバとして持ってガードで剥がすだけで、その入口に立てます。
> （Javaの`NullPointerException`「ぬるぽ」、Kotlinの`User?`、TypeScriptの`User | null`との対応は付録[a5-1](../appendix/a5-other-languages.md)へ。）

---

## 5-2. Rubyの「偽」は2つだけ（絞り込みの実装）

実装の前に、Rubyの大事な事実を一つ確認します。
**Rubyで「偽」とみなされるのは`false`と`nil`の2つだけです。**
`0`も`""`も真です。
だから`if x`は「`x`が`false`でも`nil`でもない」を意味します。

絞り込みは「条件を見て、枝ごとに変数の型を差し替えた**新しいスコープ**を作る」だけです。
スコープはPart 3で作った不変`Scope`（`scope.local(名前)`で型を引き、`scope.with_local(名前, 型)`で束縛を1つ足した新しい`Scope`を返す）をそのまま使います。

```ruby
def remove_nil(t)
  return t unless t.is_a?(Type::Union)
  # nil は nil リテラルなら Const[nil]、`x.nil?` の真の枝なら Nominal[:NilClass] で来る。両方剥がす。
  Type.union(t.members.reject { |m| m == Type::Const[nil] || m == Type::Nominal[:NilClass] })
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

`if`の型付けは、then節を「真に絞ったスコープ」で、else節を「偽に絞ったスコープ」でそれぞれ求め、最後にまとめます。

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

（`if cond; ...; end`のように**elseが無い**とき`node.subsequent`は`nil`です。
その場合は偽の枝の型を`nil`とします。
実際のRubyが、else無しの`if`が偽のとき`nil`を返すのに合わせています。）

動かすと、ちゃんと絞れます。

```ruby
# x : Integer | nil のとき
# then 節 → x は NilClass
# else 節 → x は Integer
```

<!-- run: examples/part5.rb -->
```text
nil? narrowing: OK (no errors)
expected String but got 1
```

`is_a?`でも同じ要領です（`if x.is_a?(String)`のthen節は`x`を`String`に絞ります）。
形が増えても`narrow`に分岐を足すだけです。
**偽の枝はスコープをそのまま返します**（`is_a?`の条件が成り立たなかった側は、型を変えずに元のScopeを引き継ぎます）。

ただし`is_a?`には落とし穴が一つあります。
`x`がもともと`Integer`のとき`if x.is_a?(String)`の中身を「`x`は`String`」と絞ると、その枝は*起き得ない*（IntegerはStringにならない）のに`x + 1`をStringの足し算とみなして**誤検知**します。
これは「動くコードを脅かさない」に反します。
だから**「そのクラスがあり得るときだけ絞る」**という方針をとります。
`x`が`Integer | String`のようにStringを含むときは絞り、`Integer`単体なら絞りません（その枝はdead branchなので触らない）。
`Dynamic`も絞りません（型が分からないものは、分からないまま通します）。

```ruby
check("x = 1\nif x.is_a?(String)\n x + 1\nend\n")              # OK（dead branch、誤検知しない）
check("x = c ? 1 : \"a\"\nif x.is_a?(String)\n x + 1\nend\n")  # String の足し算エラー（正しい）
```

> [!NOTE]
> **実装メモ：`possible?`ガード**　「そのクラスがあり得るか」を`narrow`に判定させるには小さなヘルパが要ります。
> `Dynamic`は「あり得ないとは言えない」のでfalse、Unionはメンバを探索、それ以外はクラスが一致するかで判定します。
>
> ```ruby
> def possible?(current, klass)
>   return false if current.is_a?(Type::Dynamic)
>   members = current.is_a?(Type::Union) ? current.members : [current]
>   members.any? { |m| Dispatch.class_of(m) == klass }
> end
> # narrow_typeのis_a? 節：klass && truthy && possible?(current, klass)のときだけ絞る
> ```
>
> このガードを入れないと、`Integer`単体に`is_a?(String)`を当てたときdead branchを`String`に絞ってしまい、`x + 1`が「Stringの足し算」として誤検知されます。

### 到達できない枝の報告（unreachable arm）

`is_a?`のdead branchを「触らない（絞らない）」のは誤検知を避ける消極的な対応ですが、Rigorはもう一歩進めて、その枝を**「絶対に通らない余分な分岐です」と指摘できる**ようにしています。
ただし**既定では黙ったまま**で、`check --unreachable`を明示したときだけ表に出るopt-inです（動くコードを脅かさないため）。
「足りない腕（missing arm）を書くまで止める」JavaとC# の網羅性検査とは逆に、**動くものには黙り、求められたときだけ通らない枝を指摘する**という姿勢です。
上の`possible?`ガードで「あり得ないときは絞らない」とした判断と、同じ軸の上にあります。

> [!NOTE]
> 到達できない枝の「型」（ボトム型`Bot`／`never`）は付録
> [a1](../appendix/a1-special-types.md)、Java/C# の網羅性検査との方向の違いは付録
> [a5-5](../appendix/a5-other-languages.md)で扱います。

---

## 5-3. 絞り込みの2つの掟（ここがRigorらしさ）

ナローイングには、Rigorが守っている掟が2つあります。
どちらも「脅かさない」ためです。

**掟その1：絞れない条件は、黙ってそのまま通す。**
`narrow`の最後の行`scope`（そのまま返す）がそれです。
`if complicated_check(x)`のような、私たちに読めない条件のときは、**何も主張しません**。
「絞れないから怪しい」とは言いません。

**掟その2：絞り込みは「事実を足す」だけ。間違えたら緩める側に倒す。**
型を*狭める*操作なので、やりすぎると「本当はあり得る値」を消してしまい、誤検知の元になります。
迷ったら絞りません。
なお、変数への**再代入**はそれ以前の全factsをリセットします。
事実は「変数名」ではなく「そのスコープ位置で確定した事実」に結びついているからです。
`x = something_else`を書いた瞬間、`x`に関するnarrowingの記憶はすべて消えます。

> [!NOTE]
> **Part 7への地ならし：Unionは「全メンバで考える」**
> Unionから何かを読むとき（例：`(Integer | String).to_s`）は、メンバを1つずつ考えてまとめるのが基本です。
> `to_s`はIntegerにもStringにもあるのでOKです。
> もし片方にしか無いメソッドなら、その分だけ怪しくなります。
> この「**全メンバを回して一番弱い結論を採る**」考え方は、Part 7の`accepts`（`:yes`/`:no`/`:maybe`）でそのまま再登場します。
> ここで身につけておいてください。

- **① 型理論**：場合分けで型情報が増えます（『しくみ』は扱わない独自地形）。
- **② Rubyだと**：`false`/`nil`だけが偽で、`x.nil?`/`is_a?`でガードするのが定石です。さらに、`x`が*局所変数*かどうかは「先に代入があるか」で決まります（無ければ`self.x`の呼び出し扱い）。[^bare]
- **③ Rigorだと**：絞り込みは*事実を足すだけ*です。読めない条件は黙り、間違えるなら緩める側に倒します。

[^bare]: この「裸の`x`が局所変数かメソッド呼び出しか」はPrismが文脈で決めます。ナローイングは局所変数にしか効きません。本編では深追いしません。

---

## 5-4. この章のまとめ

足したものは、道具`remove_nil`／`narrow`、そして`IfNode`の絞り込み付き型付けです。
`narrow`は実質7行です。
スコープはPart 3の不変`Scope`に`with_local`で束縛を足すだけです。

この章の三つの視点：

| | 内容 |
|---|---|
| ① 型理論 | 場合分けで型情報が増える（『しくみ』が扱わない独自地形。dead branch ＝ボトム型は付録a1） |
| ② Ruby/RBS | 偽は`false`/`nil`だけ、`x.nil?`/`is_a?`でガードが定石 |
| ③ Rigor実装の問題 | 絞り込みは*事実を足すだけ、読めなければ黙る、迷えば緩める*（誤検知を出さない） |

**続編に送ったもの**：

- 本物の**FactStore**（6種類の「事実の置き場」、いつ事実が無効になるか、再代入やブロックのクロージャ捕獲で事実を捨てる機微）。本編は素朴な`Scope`止まりです。この章で触れた「再代入でfactsが消える」話は、後編Part 6（完全なFactStore）で一般化します。
- `case`/`when`と`case`/`in`（パターンマッチ）の絞り込み、到達しない枝の検出（実RigorのADR-47）。本編は`if`の`nil?`/`is_a?`まで。
- **Unionのサイズ予算**：前章の`union`ヘルパは重複を消すだけですが、実RigorではUnionのメンバ数が設定上限を超えると、各メンバの名前的クラス（`Integer`や`String`など）のUnionに**強制的に拡大（widen）**します。定数畳み込みの「大きすぎたら丸める」と同じ発想で、「型も*予算を持つ*」という設計原則の別の現れです。

## 演習

1. `x : String | nil`のとき`if x`のthen節で`x`が`String`に絞れることを確かめよ
   （Rubyの偽は`false`/`nil`の2つだけ、を使う）。else節では`x`は何型か。
2. `x : Integer | String`のとき`if x.is_a?(Integer)`のthen節で`x`が`Integer`に絞れる
   ことを確かめよ。
3. `unless`も絞り込めるようにするには、`if`の型付けをどう変えればよいか方針を述べよ
   （ヒント：真の枝と偽の枝を入れ替える）。

---

**次章予告（Part 6）**：ハッシュや配列のリテラルに型をつけます（`HashShape`/`Tuple`）。
「symbolキーのオプションハッシュ」だらけのRubyで、型を*完全一致で要求すると誤検知の嵐になる*話に踏み込みます。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part5/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part5/lib)
