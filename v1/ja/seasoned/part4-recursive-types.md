---
title: The Seasoned chibirigor Part 4 ― 再帰型：μ と余帰納
description: "型が自分自身を参照する再帰型を μ 記法で扱い、Rigor の HKT／fuel 実装との対応を読み解く。"
sidebar:
  order: 14
---

# The Seasoned chibirigor Part 4 ― 再帰型：μ と余帰納

> 参考書（任意）：TAPL 20 章「再帰型」・21 章「再帰型のメタ理論」／『しくみ』8 章。
> 型が*自分自身*を参照する再帰型を扱い、Rigor の別解（HKT/`App`＋fuel）と対比します。

前の Part 3 では、ジェネリクスと型代入 ― 一つの型が*別の型を引数に取る*仕掛け ― を見ました。
この章で扱う再帰型は、型が*自分自身*を引数のように呼び込む構造です。前編は配列やハッシュを
*有限の*構造として扱いましたが、現実のデータには、自分自身を含むものがあります ― JSON、木、
連結リスト、ストリーム。これらの型は**自分を参照**します。

---

## 4-1. なぜ再帰型が要るか

いちばん身近な例は **JSON** です。`JSON.parse` が返す値は：

```
value = null | bool | number | string | Array[value] | Hash[String, value]
```

`value` の定義の中に `value` が出てきます。前編の型キャリアでは、この「自分を含む型」を
書けませんでした。木やリストも同じ：

```ruby
# 連結リスト：nil か、[要素, 残りのリスト]
list = nil | [Integer, list]
```

これらを正しく型として持つには、**再帰型**が要ります。

---

## 4-2. μ 型 ― 再帰を畳む記法

再帰型の標準記法は **μ（ミュー）型**です。`μX. T` は「`T` の中の `X` を `μX.T` 自身で置き換えた
型」を表します。たとえば `list` は：

```
μList. (nil | [Integer, List])
```

`X`（や `List`）は**型変数**で、「自分自身が入る穴」です。Part 3 で見たジェネリクスの
型変数とは役割が違い、ここでは「再帰で自分が入る位置」を指します。

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

> ▼ 図 4-1　μ 型の畳（fold）と展開（unfold）。両者は同じ型。展開は際限なく続くので、
> 等価判定には「止める仕掛け」（§4-4 の余帰納）が要る。

畳んだ形と一段展開した形は**同じ型**を表します ― でも*データ構造としては別物*。この
「畳／展開が等しい」をどう扱うかが、再帰型の実装の核です。

---

## 4-3. 同値再帰 vs 同型再帰

再帰型には 2 つの流儀があります（TAPL 20.2）：

- **同型再帰（iso-recursive）**：畳む／展開する場所を、*ユーザーが明示*する（`fold`/`unfold`）。
  実装は楽（等価判定で再帰を追わない）が、書く側が面倒。
- **同値再帰（equi-recursive）**：畳んだ形と展開した形が*そのまま等しい*。注釈不要で書きやすい
  が、等価判定の実装が難しい（再帰を追う必要がある）。

TypeScript も多くの実用言語も**同値再帰**を採り、`『しくみ』`8 章もこちらを実装します。
直感的だからです。ML 系は同型再帰が多い（他機能との相性のため）。

---

## 4-4. 等価判定の停止性と余帰納

同値再帰の難所は、**型の等価判定が止まらない**ことです。`μX.{foo:X}` と `μY.{foo:Y}` が
等しいかを調べようとすると ― X を展開、Y を展開、`foo` の中を比べる → また元の比較に戻る →
無限ループ。

解は **余帰納（coinduction）＝仮定集合**です。「いま比較中のペア」を集合 `seen` に覚えておき、
**同じペアを再び問われたら「等しい」と仮定して打ち切る**。実際に動く Ruby で書くと、こうなります
（型は基底＝`Symbol`、レコード＝`Obj`、μ＝`Rec`、型変数＝`Var`）：

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

`seen` の判定に使う `naive_eq` は、**束縛変数名の違いだけを吸収する α 同値**（展開しない）です ―
`μX.{foo:X}` と `μY.{foo:Y}` を「同じ」と見なすために、名前の対応表 `map` を引き回します。
これは Part 3 のジェネリクスで出た α 同値（「根は一つ」＝束縛変数の名前は本質でない）と
**同じ技法**です。この `naive_eq`/`unfold`/`subst` を含む全体は、単体で走る設計スケッチ
[`examples/mu_typeeq.rb`](examples/mu_typeeq.rb) にあり、`ruby mu_typeeq.rb` で次が**緑**になります：

<!-- run: mu_typeeq.rb -->
```text
PASS: muX{foo:X} == muY{foo:Y} (α + cycle)
PASS: muX{foo:X} == {foo: muX{foo:X}} (fold/unfold)
PASS: muX{foo:X} != muY{bar:Y} (field name)
PASS: stream fold == unfold (α)
PASS: Num == Num
PASS: Num != Bool
```

「証明しようとしているまさにそのことを、途中でまた問われたら、成り立つと認める」 ― これが
余帰納の心です。TAPL 21 章がこのアルゴリズムと、その正しさ（最大不動点としての部分型）を
展開します。

> **正確には**：このスケッチの `seen` 判定は `naive_eq`（束縛変数の α 同値・**展開しない**比較）
> です。これは健全な*簡約版*で、TAPL 21 章の本来の余帰納 ― 展開後に到達したペアの**最大不動点**
> を取る ― より弱い（捕まえる「等しい」が少なめ）です。教育用には十分ですが、本式の等価判定は
> 展開後のペアを `seen` に入れて回します。
>
> **参考書メモ**：『しくみ』8 章は、単一再帰（図 8.2）と相互再帰（図 8.3）で*なぜ*ナイーブな
> 展開が止まらないかを図示し、`seen` を持つ `typeEqSub` で解決します。TAPL 21 章がその理論的
> 背景（余帰納・最大不動点）です。

---

## 4-5. 発展ノート：Rigor の別解 ― HKT と fuel

Rigor は、再帰型を μ＋余帰納で*直接*は実装していません。代わりに **軽量 HKT
（higher-kinded type）** を使います（`Type::App`）。`JSON.parse` の戻りは
`App[:"json::value", []]` という*不透明な*高階型適用で、必要なときに登録済みの本体へ**還元**
されます：

```
App[:"json::value", []]
  → Value = Literal | Array[Value] | Hash[String, Value]   （再帰 union へ還元）
```

ここで余帰納の `seen` に当たるのが **fuel（燃料）予算**です。「再帰の展開をどう止めるか」と
いう同じ問題に、`『しくみ』`/TAPL は*理論的*に（余帰納で正しく等価判定）、Rigor は*工学的*に
（fuel で安全に打ち切り）答えています。**この「余帰納 vs 予算」という停止性の工学の総括は、
健全性を扱う後編 Part 7 にまとめてあります**（gradual の地に足とあわせて読み解きます）。

> **コラム：`symbolize_names: true` をリテラルで渡すと型が変わる**
>
> `JSON.parse` の型は引数オプションに依存します。とくに `symbolize_names:` キーが
> リテラル `true` で渡されると、Rigor は還元後の型を切り替えます：
>
> ```ruby
> JSON.parse(s)
> # => App[:"json::value", []]
> #    還元: Literal | Array[json::value] | Hash[String, json::value]
>
> JSON.parse(s, symbolize_names: true)
> # => App[:"json::symbolized_value", []]
> #    還元: Literal | Array[json::symbolized_value] | Hash[Symbol, json::symbolized_value]
>                                                               ^^^^^^
>                                                               キーが Symbol に変わる
> ```
>
> これが可能なのは、RBS シグネチャで `symbolize_names:` の型を `true` の
> リテラル型（`Const[true]`）で宣言し、それが HKT の型引数として渡されるからです。
> 呼び出し地点で `true` のリテラルが確認できた場合だけ `App[:"json::symbolized_value", []]`
> が選ばれ、`false` や変数（`untyped`）なら汎用版にフォールバックします。
>
> このように「引数のリテラル値が型を決める」は前編の `Const` が生きている例です。
> `HashShape` のキー読み出し（`h[:foo]` の `:foo` がリテラル）と同じ仕組みが、
> HKT の型引数選択にも使われています。還元は無限に展開し得るので、
**fuel（既定 64）＋進捗追跡**で安全側に打ち切ります。

> **参考書メモ**：HKT（型を取って型を返す型）の一次根拠は、再帰型の 20/21 章ではなく **TAPL 29
> 章「型演算子とカインド（kinding）」** です。`App[F, A]` のような型適用の正しさは、型に*種類*
> （kind）を付ける 29 章の枠組みで保証されます。Rigor の軽量 HKT はその defunctionalize した
> 実装版です。

> **コラム：HKT 条件型の判定も `:yes/:no/:maybe` の三値**
>
> `App[F, A]` を「`T` と互換か」と問われたとき、Rigor は **fuel 内で還元を試み**、
> 結果が分かれば `:yes` か `:no`、fuel が尽きれば `:maybe` を返します。
>
> これは前編 Part 7 で実装した `accepts` の三値（`:yes/:no/:maybe`）と**まったく同じ
> 枠組み**です。通常の型では「知識不足 → `:maybe`」だったものが、HKT では
> 「fuel 不足 → `:maybe`」に変わるだけで、判定ロジックの外側から見ると区別できません。
>
> | 判定ができない理由 | `accepts` の返値 |
> |---|---|
> | `untyped` が混入している | `:maybe` |
> | 型シグネチャが未登録 | `:maybe` |
> | HKT 還元の fuel 切れ | `:maybe` |
>
> Rigor は「分からなければ黙って通す」を一貫して `:maybe` という単一の語で表現します。
> HKT の複雑さが漏れ出さず、`accepts` の呼び出し元は三値の意味だけを考えればよい、
> という設計のきれいさです。

---

## 4-6. chibirigor 本編では作らなかった

前編は再帰型を**非対象**にしました（前編 Part 6 で `HashShape`/`Tuple` を有限のまま扱い、
HKT/`App`・fuel は不実装と宣言）。理由は複雑さ予算です ― μ＋余帰納も HKT＋fuel も、最小版の
主旨（双方向＋gradual＋フロー）から外れます。後編で*概念として*回収するのが正解でした。

もし最小実装を足すなら：型キャリアに `Rec(name, body)` と `TypeVar(name)` を加え、`accepts`/
等価判定に `seen` を持つ ― これが μ＋余帰納の最小形（`『しくみ』`8 章の Ruby 移植）です。
HKT 版は URI 参照＋fuel で別実装になります。

---

## 4-7. まとめ

| 論点 | 要点 |
|---|---|
| なぜ再帰型 | JSON・木・リスト・ストリームは、型が自分を参照する**再帰型** |
| μ 型 | 再帰を畳み、展開で一段ほどく。畳／展開は同じ型 |
| 2 つの流儀 | **同値再帰**（注釈不要・実装難）vs **同型再帰**（注釈必要・実装易）。実用は同値再帰が主流 |
| 等価判定 | 止まらない → **余帰納（`seen` 仮定集合）**で打ち切る（TAPL 21 章） |
| Rigor の別解 | **HKT/`App`＋fuel**（HKT の根拠は TAPL 29 章）：余帰納の代わりに*燃料予算*で打ち切る（停止性の工学の総括は Part 7） |

## 演習

1. **畳/展開をトレース**：`μX.{foo:X}` と `{foo: μX.{foo:X}}`（一段展開）が等しいことを、
   `examples/mu_typeeq.rb` の `type_eq` に沿って手でたどれ（`seen` に入るペアを 1 つ書け）。
2. **なぜ止まるか**：`μX.{foo:X} == μY.{foo:Y}` の判定で、`seen` を使わなかったら何が起きるかを
   述べ、`seen` が無限ループをどう断つかを 1 文で説明せよ。
3. **理論 vs 工学**：再帰の展開を止める方法として「余帰納（`seen`）」と「fuel 予算」を比べ、
   なぜ実用チェッカー（Rigor）が後者を選べるのか（gradual の地に足）を述べよ。

**次章（Part 5）**：前編の素朴な型推論を、Rigor の本物の**型推論**へ拡張します。呼び出し元から
引数の型を埋め戻す単一化（unification）を扱い、ここで TypeProf との対比を一本化します。
