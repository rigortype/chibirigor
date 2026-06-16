---
title: Part 4　再帰型：μ と余帰納
description: "型が自分自身を参照する再帰型を μ 記法で扱い、Rigor の HKT と fuel 実装との対応を読み解く。"
sidebar:
  order: 14
---

# The Seasoned chibirigor Part 4　再帰型：μ と余帰納

> 参考書（任意）：TAPL 20章「再帰型」、21章「再帰型のメタ理論」、『しくみ』8章。
> 型が*自分自身*を参照する再帰型を扱い、Rigorの別解（HKTと`App`とfuel）と対比します。

前のPart 3では、ジェネリクスと型代入（一つの型が*別の型を引数に取る*仕掛け）を見ました。この章で扱う再帰型は、型が*自分自身*を引数のように呼び込む構造です。

前編は配列やハッシュを*有限の*構造として扱いましたが、現実のデータには自分自身を含むものがあります。JSON、木、連結リスト、ストリームがその例です。これらの型は**自分を参照**します。

---

## 4-1. なぜ再帰型が要るか

いちばん身近な例は**JSON**です。`JSON.parse`が返す値は次のように定義されます：

```
value = null | bool | number | string | Array[value] | Hash[String, value]
```

`value`の定義の中に`value`が出てきます。前編の型キャリアでは、この「自分を含む型」を書けませんでした。木やリストも同じです：

```ruby
# 連結リスト：nil か、[要素, 残りのリスト]
list = nil | [Integer, list]
```

これらを正しく型として持つには、**再帰型**が要ります。

---

## 4-2. μ 型　再帰を畳む記法

再帰型の標準記法は**μ（ミュー）型**です。`μX. T`は「`T`の中の`X`を`μX.T`自身で置き換えた型」を表します。たとえば`list`は：

```
μList. (nil | [Integer, List])
```

`X`（や`List`）は**型変数**で、「自分自身が入る穴」を指します。Part 3で見たジェネリクスの型変数とは役割が違い、ここでは「再帰で自分が入る位置」を表します。

**展開（unfold）**：μ 型は、変数に自分自身を代入して*一段ほどく*ことができます。

```
μList.(nil | [Integer, List])
  =  nil | [Integer, μList.(nil | [Integer, List])]   （一段展開）
```

```text
   畳んだ形                       一段展開した形
   ┌─────────────┐   unfold →    ┌──────────────────────────────┐
   │ μList.{ … List } │  ───────────  │ { … μList.{ … List } }          │
   └─────────────┘   ← fold       └──────────────────────────────┘
        （同じ型を指す。展開は無限に続けられる ― だから等価判定で止め方が要る）
```

![図4-1　μ型の畳（fold）と展開（unfold）](../figures/svg/seasoned-4-1.svg)
> ▼ 図4-1　μ 型の畳（fold）と展開（unfold）。両者は同じ型。展開は際限なく続くので、等価判定には「止める仕掛け」（§4-4の余帰納）が要る。

畳んだ形と一段展開した形は**同じ型**を表します。しかし*データ構造としては別物*です。この「畳と展開が等しい」をどう扱うかが、再帰型の実装の核です。

---

## 4-3. 同値再帰vs同型再帰

再帰型には2つの流儀があります（TAPL 20.2）：

- **同型再帰（iso-recursive）**：畳む、展開する場所を*ユーザーが明示*する（`fold`/`unfold`）
  - 実装は楽（等価判定で再帰を追わない）ですが、書く側が面倒です。
- **同値再帰（equi-recursive）**：畳んだ形と展開した形が*そのまま等しい*
  - 注釈不要で書きやすいですが、等価判定の実装が難しくなります（再帰を追う必要があります）。

TypeScriptも多くの実用言語も**同値再帰**を採り、『しくみ』8章もこちらを実装します。直感的だからです。ML系は同型再帰が多く、他機能との相性が理由です。

---

## 4-4. 等価判定の停止性と余帰納

同値再帰の難所は、**型の等価判定が止まらない**ことです。`μX.{foo:X}`と`μY.{foo:Y}`が等しいかを調べようとすると、Xを展開、Yを展開、`foo`の中を比べる、また元の比較に戻る、という無限ループになります。

解は**余帰納（coinduction）＝仮定集合**です。「いま比較中のペア」を集合`seen`に覚えておき、**同じペアを再び問われたら「等しい」と仮定して打ち切ります**。実際に動くRubyで書くとこうなります（型は基底＝`Symbol`、レコード＝`Obj`、μ＝`Rec`、型変数＝`Var`）：

<!-- include: mu_typeeq.rb#type_eq -->
```ruby
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
```

`seen`の判定に使う`naive_eq`は、**束縛変数名の違いだけを吸収する α 同値**（展開しない）です。`μX.{foo:X}`と`μY.{foo:Y}`を「同じ」と見なすために、名前の対応表`map`を引き回します。これはPart 3のジェネリクスで出た α 同値（「根は一つ」＝束縛変数の名前は本質でない）と**同じ技法**です。

この`naive_eq`、`unfold`、`subst`を含む全体は、単体で走る設計スケッチ[`examples/mu_typeeq.rb`](examples/mu_typeeq.rb)にあり、`ruby mu_typeeq.rb`で次が**緑**になります：

<!-- run: mu_typeeq.rb -->
```text
PASS: muX{foo:X} == muY{foo:Y} (α + cycle)
PASS: muX{foo:X} == {foo: muX{foo:X}} (fold/unfold)
PASS: muX{foo:X} != muY{bar:Y} (field name)
PASS: stream fold == unfold (α)
PASS: Num == Num
PASS: Num != Bool
```

「証明しようとしているまさにそのことを、途中でまた問われたら、成り立つと認める」。これが余帰納の心です。TAPL 21章がこのアルゴリズムと、その正しさ（最大不動点としての部分型）を展開します。

> [!NOTE]
> **正確には**：このスケッチの`seen`判定は`naive_eq`（束縛変数の α 同値で、**展開しない**比較）です。これは健全な*簡約版*で、TAPL 21章の本来の余帰納（展開後に到達したペアの**最大不動点**を取る）より弱く、捕まえる「等しい」が少なめです。教育用には十分ですが、本式の等価判定は展開後のペアを`seen`に入れて回します。
>
> **参考書メモ**：『しくみ』8章は、単一再帰（図8.2）と相互再帰（図8.3）で*なぜ*ナイーブな展開が止まらないかを図示し、`seen`を持つ`typeEqSub`で解決します。TAPL 21章がその理論的背景（余帰納、最大不動点）です。

---

## 4-5. 発展ノート：Rigorの別解（HKTとfuel）

Rigorは、再帰型を μ と余帰納で*直接*は実装していません。代わりに**軽量HKT（higher-kinded type）**を使います（`Type::App`）。`JSON.parse`の戻りは`App[:"json::value", [String]]`という*不透明な*高階型適用（key型を引数に取るarity 1）で、必要なときに登録済みの本体へ**還元**されます：

```
App[:"json::value", [String]]
  → Value = Literal | Array[Value] | Hash[String, Value]   （再帰 union へ還元）
```

ここで余帰納の`seen`に当たるのが**fuel（燃料）予算**です。「再帰の展開をどう止めるか」という同じ問題に、『しくみ』とTAPLは*理論的*に（余帰納で正しく等価判定）、Rigorは*工学的*に（fuelで安全に打ち切り）答えています。**この「余帰納vs予算」という停止性の工学の総括は、健全性を扱う後編Part 7にまとめてあります**（gradualの割り切りとあわせて読み解きます）。

> [!NOTE]
> **`symbolize_names: true`をリテラルで渡すと型が変わる**
>
> `JSON.parse`の型は引数オプションに依存します。とくに`symbolize_names:`キーがリテラル`true`で渡されると、Rigorは還元後の型を切り替えます：
>
> ```ruby
> JSON.parse(s)
> # => App[:"json::value", [String]]
> #    還元: Literal | Array[json::value] | Hash[String, json::value]
>
> JSON.parse(s, symbolize_names: true)
> # => App[:"json::value", [Symbol]]   ← 同じURI、key型引数だけSymbolに
> #    還元: Literal | Array[json::value] | Hash[Symbol, json::value]
> ```
>
> これが可能なのは、RBSシグネチャで`symbolize_names:`の型を`true`のリテラル型（`Const[true]`）で宣言し、それがHKTの型引数として渡されるからです。呼び出し地点で`true`のリテラルが確認できた場合だけkey型引数が`Symbol`になり（`App[:"json::value", [Symbol]]`）、`false`や変数（`untyped`）なら既定の`String`版に戻ります。
>
> このように「引数のリテラル値が型を決める」のは前編の`Const`が生きている例です。`HashShape`のキー読み出し（`h[:foo]`の`:foo`がリテラル）と同じ仕組みが、HKTの型引数選択にも使われています。還元は無限に展開し得るので、**fuel（既定64）と進捗追跡**で安全側に打ち切ります。

> [!TIP]
> **参考書メモ**：HKT（型を取って型を返す型）の一次根拠は、再帰型の20章、21章ではなく**TAPL 29章「型演算子とカインド（kinding）」**です。`App[F, A]`のような型適用の正しさは、型に*種類*（kind）を付ける29章の枠組みで保証されます。Rigorの軽量HKTはそのdefunctionalizeした実装版です。

> [!NOTE]
> **HKT条件型の判定も`:yes/:no/:maybe`の三値**
>
> `App[F, A]`を「`T`と互換か」と問われたとき、Rigorは**fuel内で還元を試み**、結果が分かれば`:yes`か`:no`、fuelが尽きれば`:maybe`を返します。
>
> これは前編Part 7で実装した`accepts`の三値（`:yes/:no/:maybe`）と**まったく同じ枠組み**です。通常の型では「知識不足 → `:maybe`」だったものが、HKTでは「fuel不足 → `:maybe`」に変わるだけで、判定ロジックの外側から見ると区別できません。
>
> | 判定ができない理由 | `accepts`の返値 |
> |---|---|
> | `untyped`が混入している | `:maybe` |
> | 型シグネチャが未登録 | `:maybe` |
> | HKT還元のfuel切れ | `:maybe` |
>
> Rigorは「分からなければ黙って通す」を一貫して`:maybe`という単一の語で表現します。HKTの複雑さが漏れ出さず、`accepts`の呼び出し元は三値の意味だけを考えればよい、という設計の利点です。

---

## 4-6. chibirigor本編では作らなかった

前編は再帰型を**非対象**にしました（前編Part 6で`HashShape`と`Tuple`を有限のまま扱い、HKTと`App`、fuelは不実装と宣言しています）。理由は複雑さ予算です。μ と余帰納もHKTとfuelも、最小版の主旨（双方向、gradual、フロー）から外れます。後編で*概念として*回収するのが正解でした。

もし最小実装を足すなら、型キャリアに`Rec(name, body)`と`TypeVar(name)`を加え、`accepts`と等価判定に`seen`を持つ形になります。これが μ と余帰納の最小形（『しくみ』8章のRuby移植）です。HKT版はURI参照とfuelで別実装になります。

---

## 4-7. まとめ

| 論点 | 要点 |
|---|---|
| なぜ再帰型 | JSON、木、リスト、ストリームは、型が自分を参照する**再帰型** |
| μ 型 | 再帰を畳み、展開で一段ほどく。畳と展開は同じ型 |
| 2つの流儀 | **同値再帰**（注釈不要、実装難）vs **同型再帰**（注釈必要、実装易）。実用は同値再帰が主流 |
| 等価判定 | 止まらない → **余帰納（`seen`仮定集合）**で打ち切る（TAPL 21章） |
| Rigorの別解 | **HKTと`App`とfuel**（HKTの根拠はTAPL 29章）：余帰納の代わりに*燃料予算*で打ち切る（停止性の工学の総括はPart 7） |

## 演習

1. **畳と展開をトレース**：`μX.{foo:X}`と`{foo: μX.{foo:X}}`（一段展開）が等しいことを、`examples/mu_typeeq.rb`の`type_eq`に沿って手でたどれ（`seen`に入るペアを1つ書け）。
2. **なぜ止まるか**：`μX.{foo:X} == μY.{foo:Y}`の判定で、`seen`を使わなかったら何が起きるかを述べ、`seen`が無限ループをどう断つかを1文で説明せよ。
3. **理論vs工学**：再帰の展開を止める方法として「余帰納（`seen`）」と「fuel予算」を比べ、なぜ実用チェッカー（Rigor）が後者を選べるのか（gradualの割り切り）を述べよ。

**次章（Part 5）**：前編の素朴な型推論を、Rigorの本物の**型推論**へ拡張します。呼び出し元から引数の型を埋め戻す単一化（unification）を扱い、ここでTypeProfとの対比を一本化します。

---

> **この章の設計スケッチ** → [`examples/mu_typeeq.rb`](examples/mu_typeeq.rb)（`ruby mu_typeeq.rb`で自己チェック）
