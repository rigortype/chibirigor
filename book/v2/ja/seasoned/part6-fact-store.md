---
title: Part 6　完全な FactStore
description: 前編の素朴な Scope と narrow を、Rigor の本物の FactStore（6 バケツ設計）へ拡張して読み解く。
sidebar:
  order: 16
---

# The Seasoned chibirigor Part 6　完全なFactStore

> [!TIP]
> 参考書（任意）：フロー解析の一般論（データフロー解析）。型理論の教科書（TAPL、『しくみ』）には直接の対応章はありません。ここはgradualな実用チェッカーの独自地形です。前編の素朴な`Scope`、`narrow`を、Rigorの本物の**FactStore**へ拡張する章です。

前編[Part 3（Scope）](../little/part3-scope-and-statements.md)で私たちは型環境を`Scope`（変数名→型のHash）として持ち、前編[Part 5（ナローイング）](../little/part5-narrowing.md)でナローイングを「枝ごとに`Scope`を差し替える」素朴な仕組みで実装しました。実用規模ではこれでは足りません。何が足りないか、そしてRigorがどう埋めたかを見ます。

後編[Part 5（本物の型推論）](part5-real-inference.md)で引数を埋めたのと同じ精神で、ここではフロー感応な型環境を実用に耐える設計へ一般化します。

> [!NOTE]
> **個別の絞り込みパターンは付録a2へ。**本章は6バケツ、stability、joinの骨格に絞ります。`&&`、`||`の積み下ろし、正規表現キャプチャ、refinement carrier（`non-empty-string`等）、ivarのunion、エスケープブロックといった**個別のナローイングのパターン**は付録[a2（ナローイングのパターン集）](../appendix/a2-narrowing-patterns.md)に集約しました。「この絞り込みは具体的にどう動くのか」と思ったらそちらを引いてください。

---

## 6-1. 「型」だけでなく「事実」を持つ

前編の`Scope`は「`x`は`Integer`」のような**型**を持ちました。でも実コードでは、もっと細かい**事実（fact）**を流したいことがあります：

- `x`はnilでない（`if x`を通った後）
- `h`はキー`:name`を持つ（`h.key?(:name)`の後）
- `arr`は空でない（`arr.empty?`がfalseの後）
- `x`と`y`は同じ値（`x == y`の後）

これらは「型」というより、その地点で成り立っている命題です。FactStoreは、型環境を**フロー感応な事実の集合**へ一般化したものです。本物のRigorでは、各事実（`FactStore::Fact`）は`bucket`、`target`（対象）、`predicate`（述語）、`payload`、`polarity`（極性）、`stability`（安定性）を持ちます。本章ではこのうち`bucket`、`target`、`predicate`の3つに絞って最小化します。

`payload`が運ぶ「述語を満たす値の集合」を表す精密な型（`non-empty-array`、`positive-int`などの**refinement carrier**）は、前編[Part 1](../little/part1-literals-and-arithmetic.md)の`Const[42]`（特定の1つの値）とは別概念です。詳しくは付録[a2（ナローイングのパターン集）](../appendix/a2-narrowing-patterns.md)のa2-6（refinement carrier）へ。

---

## 6-2. 6つの「置き場」（バケツ）

事実は、対象の種類で**バケツ**に分けて持ちます。Rigorは6種あります：

1. **local_binding** … ローカル変数（前編の`Scope#locals`がこれ）。
2. **captured_local** … ブロックに捕獲されたローカル。
3. **object_content** … オブジェクトの中身（ivar、ハッシュのキーなど）。
4. **global_storage** … グローバル変数、クラス変数など。
5. **dynamic_origin** … `untyped`の出どころ（どこで型を見失ったか）。
6. **relational** … 変数間の関係（`x == y`など）。

```text
   FactStore（不変）
   ├ local_binding    : x は non-nil          ┐
   ├ captured_local   : ブロックが書く y       │ どこの事実か（対象のスコープ）
   ├ object_content   : obj.name は設定済み    │ で 5 つに分ける
   ├ global_storage   : $cfg は Hash           │
   ├ relational       : a == b                 ┘
   └ dynamic_origin   : z は ◯◯行で untyped 化  ← 他5つと毛色が違う（由来追跡）
```

![図6-1　FactStoreの6バケツ](../figures/svg/seasoned-6-1.svg)
> ▼ 図6-1　FactStoreの6バケツ。上5つは「事実がどの対象に付くか」で分ける。
> `dynamic_origin`だけは対象スコープではなく「`untyped`がどこで生まれたか」を追う毛色違い。

バケツに分ける理由は、**無効化（invalidation）のタイミングが違う**からです。ローカルへの再代入はそのlocal_bindingの事実だけを消せばよいですが、メソッド呼び出しはobject_contentを広く疑う必要があります。バケツ名は本物のRigorの内部仕様（`inference-engine.md`）の正式名と一致します。[^buckets]

なお`object_content`バケツに入るivar（インスタンス変数）の型は「すべての可視な代入のunion」になります（`@x`をどこかで`nil`にし得るなら、どこで読んでも`nil`を含む保守的近似）。この個別パターンは付録[a2（ナローイングのパターン集）](../appendix/a2-narrowing-patterns.md)のa2-4（ivar union）へ。

[^buckets]: 6バケツのうち`local_binding`、`captured_local`、`object_content`、`global_storage`、`relational`は「事実が付く対象」で分かれるのに対し、`dynamic_origin`だけは「`untyped`の由来を追う」別系統です。位置づけが違うことに注意したい（実specでも6つ目として並ぶが役割は別）。

---

## 6-3. 事実はいつ消えるか（stability）

前編の素朴な`Scope`には無かった最重要の概念が**stability（安定性）**、すなわち事実の寿命です。ナローイングで得た事実は、ある操作で崩れます：

- **再代入**：`x = …`は`x`に関するlocal_binding事実を消す（前編でもやった）
- **メソッド呼び出し**：`obj.mutate!`は、`obj`のobject_content事実を疑う（中身が変わったかも）
- **エスケープ**：変数がブロックや別メソッドに渡ると、いつ変更されるか読めないので保守的に消す

各事実は「いつまで有効か」を持ち、対応する操作で**保守的に無効化**します。迷ったら消す（緩める側に倒す）という方針です。古い事実を信じて誤検知を出すより、事実を捨てて`untyped`に戻る方が安全だからです。前編[Part 5（ナローイング）](../little/part5-narrowing.md)の「絞り込みは事実を足すだけ、間違えたら緩める」を、寿命まで含めて精密化したものです。

「エスケープ」とは、変数を捕獲したブロックが呼び出し元の外へ脱出するケースです。`Thread.new`、`define_method`、`Enumerator`などにブロックを渡して保存される場合がこれにあたります。`each`、`map`のような即時呼び出しブロックは事実をほぼ保持できますが、いつ走るか読めないエスケープブロックは捕獲変数の`captured_local`事実を保守的に無効化します。この「即時vsエスケープ」の判定の詳細は付録[a2（ナローイングのパターン集）](../appendix/a2-narrowing-patterns.md)のa2-3（エスケープブロック）に集約しました。

この「不変ストア＋バケツ指定の無効化」を、動く最小スケッチにしたのが[`examples/fact_invalidation.rb`](examples/fact_invalidation.rb)です。`with_fact`、`invalidate_target`は新しいストアを返します（不変）：

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

`ruby fact_invalidation.rb`で、再代入で`x`の事実が消えること、そしてメソッド呼び出しは`object_content`だけ落として`local_binding`は残すことが**緑**になります：

<!-- run: fact_invalidation.rb -->
```text
PASS: fact is present after narrowing
PASS: reassignment clears x's local_binding fact
PASS: method call drops object_content but keeps local_binding
```

---

## 6-4. クロージャ捕獲という難所

Rubyのブロックは、外側のローカルを**捕獲して書き換え**られます：

```ruby
x = nil
[1, 2, 3].each { |i| x = i }   # ブロックが x を書き換える
# ここで x は nil とは限らない
```

ナローイングが`x`を「nilでない」と絞っても、ブロックが`x`に再代入し得るなら、その事実は危うくなります。FactStoreは、ブロックが外側のローカルを書くことを検知して、そのcaptured_local事実を無効化します。

§6-3で見たエスケープ（呼ばれるタイミングが読めない）に加えて、捕獲固有の難所はこの「ブロックが外側を書き換える」点です。`each`のように即時に呼ばれるブロックでも、外側のローカルへ代入していればナローイングは崩れます。捕獲した変数を読むだけか書くかで扱いが変わり、書くなら即時呼びでも`captured_local`事実を落とします。

前編の素朴な`Scope`はここをまったく扱いませんでした（だから本編は`each`等のブロック内ナローイングに踏み込まなかった）。実用ではここが誤検知の温床で、Rigorが最も気を遣う所です。

---

## 6-5. 合流（join）

`if`の二枝が合流したあと、どの事実が生き残るかを考えます。答えは「両方の枝で成り立つ事実だけ」です：

```ruby
if cond
  # 枝A: x は Integer
else
  # 枝B: x は String
end
# 合流後: x は Integer | String（両枝の事実の「共通部分」＝join）
```

FactStoreの`join`は、二つの入り口の事実集合の共通部分だけを残します（型はunion、事実は積）。「join」はデータフロー解析の慣用語（合流点の演算）で、型格子のjoin（上限）ではなく事実格子のmeet（共通部分）に相当します。

前編は`if`の結果型を`Type.union`で合わせましたが、事実レベルのjoinまではやっていませんでした。後編のFactStoreはそこを一般化します。

---

## 6-6. それでも不変、フロー感応

ここまで足しても、設計の芯は前編と同じです：

- **不変**：FactStoreも`Scope`同様イミュータブルです
  - `with_fact`、`invalidate`は新しいストアを返します
  - 「どの地点で何が成り立つか」が、状態の破壊なしに追えます
- **フロー感応**：事実はプログラムの各点で違います
  - 同じ`x`でも、`if`の中と外で別の事実を持ちます
- **narrowingは事実を足すだけ**：型代入ではなく事実の追加です（前編[Part 5（ナローイング）](../little/part5-narrowing.md)の方針のまま）

---

## 6-7. まとめ

- 型環境を、フロー感応な**事実の集合**（FactStore）に一般化します。
- 事実は対象の種類で**6バケツ**に分け、無効化のタイミングを分けます。
- **stability（寿命）**：再代入、メソッド呼び、エスケープで保守的に消します。迷ったら消す。
- **クロージャ捕獲**：ブロックが外側を書くと事実を無効化します。呼ばれ方（即時、遅延）で扱いを変えます。
- **join**：分岐合流では両枝で成り立つ事実だけ残します。
- 不変、フロー感応、「事実を足すだけ」は前編から変わりません。
- 個別の絞り込みパターン（`&&`、`||`、正規表現キャプチャ、refinement carrier、ivar union、エスケープブロック）は付録[a2](../appendix/a2-narrowing-patterns.md)に集約しています。

## 演習

1. **再代入で事実が消える**：`examples/fact_invalidation.rb`で、`x = nil; arr.each { |i| x = i }`の後に`x`の「non-nil」事実がどのバケツでなぜ無効化されるべきかを述べよ（`local_binding`か`captured_local`か）。
2. **バケツ指定の無効化**：`obj.mutate!`が`obj`の`object_content`事実だけを落とし、`local_binding`（`obj`がUserである、など）を残すのが安全な理由を1文で。逆に全部消すと何が困るか。
3. **joinをトレース**：`if cond; x=1 else x="a" end`の合流後、`x`についてどんな事実が残るか（両枝の積）。もし「片方で成り立つ事実」も残したら、なぜ誤検知になるかを述べよ。

---

**次章予告（Part 7）**：ここまで「迷ったら消す」「わざと緩める」を積んできました。次章ではそれを扱います。**健全性（soundness）と正規化**、そしてRigorがわざとunsoundにする4箇所を扱います。gradualの2つの規律（consistencyとguarantee）と「余帰納vs予算」を一箇所にまとめ、後編Part 1の双方向の地図と対をなします。

---

> **この章の設計スケッチ** → [`examples/fact_invalidation.rb`](examples/fact_invalidation.rb)（`ruby fact_invalidation.rb`で自己チェック）
