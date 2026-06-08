---
title: The Seasoned chibirigor Part 6 ― 完全な FactStore
description: 前編の素朴な Scope と narrow を、Rigor の本物の FactStore（6 バケツ設計）へ拡張して読み解く。
sidebar:
  order: 16
---

# The Seasoned chibirigor Part 6 ― 完全な FactStore

> 参考書（任意）：フロー解析の一般論（データフロー解析）。型理論の教科書（TAPL/『しくみ』）には
> 直接の対応章はありません ― ここは gradual・実用チェッカーの独自地形です。
> 前編の素朴な `Scope`／`narrow` を、Rigor の本物の **FactStore** へ拡張する章です。

前編 [Part 3（Scope）](../little/part3-scope-and-statements.md)で私たちは型環境を
`Scope`（変数名→型の Hash）として持ち、前編 [Part 5（ナローイング）](../little/part5-narrowing.md)で
ナローイングを「枝ごとに `Scope` を差し替える」素朴な仕組みで実装しました。実用規模では
これでは足りません。何が足りないか、そして Rigor がどう埋めたかを見ます。

後編 [Part 5（本物の型推論）](part5-real-inference.md)で引数を埋めたのと同じ精神で、
ここではフロー感応な型環境を実用に耐える設計へ一般化します。

> **個別の絞り込みパターンは付録 a2 へ。** 本章は 6 バケツ＋stability＋join の骨格に絞ります。
> `&&`/`||` の積み下ろし、正規表現キャプチャ、refinement carrier（`non-empty-string` 等）、
> ivar の union、エスケープブロックといった**個別のナローイング・パターン**は付録
> [a2 ― ナローイング・パターン集](../appendix/a2-narrowing-patterns.md)に集約しました。
> 「この絞り込みは具体的にどう動くのか」と思ったらそちらを引いてください。

---

## 6-1. 「型」だけでなく「事実」を持つ

前編の `Scope` は「`x` は `Integer`」のような**型**を持ちました。でも実コードでは、もっと
細かい**事実（fact）**を流したい：

- `x` は nil でない（`if x` を通った後）
- `h` はキー `:name` を持つ（`h.key?(:name)` の後）
- `arr` は空でない（`arr.empty?` が false の後）
- `x` と `y` は同じ値（`x == y` の後）

これらは「型」というより、その地点で*成り立っている命題*です。FactStore は、型環境を
**フロー感応な事実の集合**へ一般化したものです。本物の Rigor では、各事実（`FactStore::Fact`）は
**`bucket`・`target`（対象）・`predicate`（述語）・`payload`・`polarity`（極性）・`stability`
（安定性）** を持ちます。本章ではこのうち `bucket`・`target`・`predicate` の 3 つに絞って
最小化します。

`payload` が運ぶ「述語を満たす値の集合」を表す精密な型 ― `non-empty-array`・`positive-int`
などの **refinement carrier** ― は、前編 [Part 1](../little/part1-literals-and-arithmetic.md)の
`Const[42]`（特定の 1 つの値）とは別概念です。詳しくは付録
[a2 ― ナローイング・パターン集](../appendix/a2-narrowing-patterns.md)の a2-6（refinement carrier）。

---

## 6-2. 6 つの「置き場」（バケツ）

事実は、対象の種類で**バケツ**に分けて持ちます。Rigor は 6 種：

1. **local_binding** … ローカル変数（前編の `Scope#locals` がこれ）。
2. **captured_local** … ブロックに捕獲されたローカル。
3. **object_content** … オブジェクトの中身（ivar、ハッシュのキーなど）。
4. **global_storage** … グローバル・クラス変数など。
5. **dynamic_origin** … `untyped` の出どころ（どこで型を見失ったか）。
6. **relational** … 変数*間*の関係（`x == y` など）。

```text
   FactStore（不変）
   ├ local_binding    : x は non-nil          ┐
   ├ captured_local   : ブロックが書く y       │ どこの事実か（対象のスコープ）
   ├ object_content   : obj.name は設定済み    │ で 5 つに分ける
   ├ global_storage   : $cfg は Hash           │
   ├ relational       : a == b                 ┘
   └ dynamic_origin   : z は ◯◯行で untyped 化  ← 他5つと毛色が違う（由来追跡）
```

![図 6-1　FactStore の 6 バケツ](../figures/svg/seasoned-6-1.svg)
> ▼ 図 6-1　FactStore の 6 バケツ。上 5 つは「事実が*どの対象*に付くか」で分ける。
> `dynamic_origin` だけは対象スコープではなく「`untyped` が*どこで生まれたか*」を追う毛色違い。

なぜ分けるか ― **無効化（invalidation）のタイミングが違う**からです。ローカルへの再代入は
その local_binding の事実だけを消せばよいが、メソッド呼び出しは object_content を広く疑う
必要がある、というように。バケツ名は本物の Rigor の内部仕様（`inference-engine.md`）の
正式名と一致します。[^buckets]

なお `object_content` バケツに入る ivar（インスタンス変数）の型は「すべての可視な代入の
union」になります（`@x` をどこかで `nil` にし得るなら、どこで読んでも `nil` を含む保守的近似）。
この個別パターンは付録 [a2 ― ナローイング・パターン集](../appendix/a2-narrowing-patterns.md)の a2-4（ivar union）を参照。

[^buckets]: 6 バケツのうち `local_binding`/`captured_local`/`object_content`/`global_storage`/
`relational` は「事実が付く対象」で分かれるのに対し、`dynamic_origin` だけは「`untyped` の
*由来*を追う」別系統です。位置づけが違うことに注意（実 spec でも 6 つ目として並ぶが役割は別）。

---

## 6-3. 事実はいつ消えるか（stability）

前編の素朴な `Scope` には無かった最重要の概念が **stability（安定性）＝事実の寿命**です。
ナローイングで得た事実は、*ある操作で崩れます*：

- **再代入**：`x = …` は `x` に関する local_binding 事実を消す（前編でもやった）。
- **メソッド呼び出し**：`obj.mutate!` は、`obj` の object_content 事実を疑う（中身が変わったかも）。
- **エスケープ**：変数がブロックや別メソッドに渡ると、いつ変更されるか読めない → 保守的に消す。

各事実は「いつまで有効か」を持ち、対応する操作で**保守的に無効化**します。**迷ったら消す**
（緩める側に倒す）― 古い事実を信じて誤検知を出すより、事実を捨てて `untyped` に戻る方が
安全だからです。前編 [Part 5（ナローイング）](../little/part5-narrowing.md)の「絞り込みは
事実を足すだけ・間違えたら緩める」を、寿命まで含めて精密化したものです。

「エスケープ」とは、変数を捕獲したブロックが**呼び出し元の外へ脱出する**（`Thread.new`・
`define_method`・`Enumerator` などにブロックを渡して保存される）ケースです。`each`/`map` の
ような即時呼び出しブロックは事実をほぼ保持できますが、いつ走るか読めないエスケープブロックは
捕獲変数の `captured_local` 事実を保守的に無効化します ― この「即時 vs エスケープ」の判定の
詳細は付録 [a2 ― ナローイング・パターン集](../appendix/a2-narrowing-patterns.md)の a2-3
（エスケープブロック）に集約しました。

この「不変ストア＋バケツ指定の無効化」を、動く最小スケッチにしたのが
[`examples/fact_invalidation.rb`](examples/fact_invalidation.rb) です。`with_fact`/
`invalidate_target` は*新しい*ストアを返します（不変）：

<!-- include: fact_invalidation.rb#factstore -->
```ruby
# 不変な事実の束。with_fact / invalidate_target は *新しい* ストアを返す。
class FactStore
  def initialize(facts = [])
    @facts = facts.freeze
  end

  def with_fact(bucket, target, predicate)
    FactStore.new(@facts + [Fact.new(bucket, target, predicate)])
  end

  # target に関する事実を消した新ストア。buckets を指定すると、そのバケツだけ消す。
  def invalidate_target(target, buckets: nil)
    kept = @facts.reject do |f|
      f.target == target && (buckets.nil? || buckets.include?(f.bucket))
    end
    FactStore.new(kept)
  end

  def predicates_for(target)
    @facts.select { |f| f.target == target }.map(&:predicate)
  end
end
```

`ruby fact_invalidation.rb` で、**再代入で `x` の事実が消える**こと、そして**メソッド呼び出しは
`object_content` だけ落として `local_binding` は残す**ことが**緑**になります：

<!-- run: fact_invalidation.rb -->
```text
PASS: fact is present after narrowing
PASS: reassignment clears x's local_binding fact
PASS: method call drops object_content but keeps local_binding
```

---

## 6-4. クロージャ捕獲という難所

Ruby のブロックは、外側のローカルを**捕獲して書き換え**られます：

```ruby
x = nil
[1, 2, 3].each { |i| x = i }   # ブロックが x を書き換える
# ここで x は nil とは限らない
```

ナローイングが `x` を「nil でない」と絞っても、ブロックが `x` に再代入し得るなら、その事実は
*危うい*。FactStore は、ブロックが**外側のローカルを書く**ことを検知して、その captured_local
事実を無効化します。

§6-3 で見たエスケープ（呼ばれる*タイミング*が読めない）に加えて、捕獲固有の難所はこの
「ブロックが外側を**書き換える**」点です。`each` のように即時に呼ばれるブロックでも、外側の
ローカルへ代入していればナローイングは崩れる。捕獲した変数を**読むだけ**か**書くか**で扱いが
変わり、書くなら即時呼びでも `captured_local` 事実を落とします。

前編の素朴な `Scope` はここを*まったく*扱いませんでした（だから本編は `each` 等のブロック内
ナローイングに踏み込まなかった）。実用ではここが誤検知の温床で、Rigor が最も気を遣う所です。

---

## 6-5. 合流（join）― 分岐が合わさるとき

`if` の二枝が合流したあと、どの事実が生き残るか。答えは「**両方の枝で成り立つ事実だけ**」：

```ruby
if cond
  # 枝A: x は Integer
else
  # 枝B: x は String
end
# 合流後: x は Integer | String（両枝の事実の「共通部分」＝join）
```

FactStore の `join` は、二つの入り口の事実集合の*共通部分*だけを残します（型は union、事実は
積）。前編は `if` の結果型を `Type.union` で合わせましたが、事実レベルの join まではやって
いませんでした。後編の FactStore はそこを一般化します。

---

## 6-6. それでも不変・フロー感応

ここまで足しても、設計の芯は前編と同じです：

- **不変**：FactStore も `Scope` 同様イミュータブル。`with_fact`/`invalidate` は*新しい*ストアを
  返す。「どの地点で何が成り立つか」が、状態の破壊なしに追える。
- **フロー感応**：事実はプログラムの各点で違う。同じ `x` でも、`if` の中と外で別の事実を持つ。
- **narrowing は事実を*足す*だけ**：型代入ではなく事実の追加（前編 [Part 5（ナローイング）](../little/part5-narrowing.md)の方針のまま）。

---

## 6-7. まとめ

- 型環境を、フロー感応な**事実の集合**＝FactStore に一般化する。
- 事実は対象の種類で **6 バケツ**に分け、**無効化のタイミング**を分ける。
- **stability（寿命）**：再代入・メソッド呼び・エスケープで保守的に消す。迷ったら消す。
- **クロージャ捕獲**：ブロックが外側を書くと事実を無効化。呼ばれ方（即時／遅延）で扱いを変える。
- **join**：分岐合流では両枝で成り立つ事実だけ残す。
- 不変・フロー感応・「事実を足すだけ」は前編から不変。
- 個別の絞り込みパターン（`&&`/`||`・正規表現キャプチャ・refinement carrier・ivar union・
  エスケープブロック）は付録 [a2](../appendix/a2-narrowing-patterns.md) に集約。

## 演習

1. **再代入で事実が消える**：`examples/fact_invalidation.rb` で、`x = nil; arr.each { |i| x = i }`
   の後に `x` の「non-nil」事実がどのバケツでなぜ無効化されるべきかを述べよ（`local_binding` か
   `captured_local` か）。
2. **バケツ指定の無効化**：`obj.mutate!` が `obj` の `object_content` 事実だけを落とし、
   `local_binding`（`obj` が User である、など）を残すのが安全な理由を 1 文で。逆に全部消すと
   何が困るか。
3. **join をトレース**：`if cond; x=1 else x="a" end` の合流後、`x` についてどんな事実が残るか
   （両枝の積）。もし「片方で成り立つ事実」も残したら、なぜ誤検知になるかを述べよ。

---

**次章予告（Part 7）**：ここまで「迷ったら消す」「わざと緩める」を積んできました。次章では
それを正面から ― **健全性（soundness）と正規化**、そして Rigor が*わざと* unsound にする
4 箇所を扱います。gradual の 2 つの規律（consistency と guarantee）と「余帰納 vs 予算」を
一箇所にまとめ、後編 Part 1 の双方向の地図とブックエンドを成します。
