---
title: "Part 5 ― 完全な FactStore"
description: "前編の素朴な Scope と narrow を、Rigor の本物の FactStore（6 バケツ設計）へ拡張して読み解く。"
sidebar:
  order: 5
draft: true
---

# 【ドラフト】The Seasoned chibirigor Part 5 ― 完全な FactStore

> 参考書（任意）：フロー解析の一般論（データフロー解析）。型理論の教科書（TAPL/『しくみ』）には
> 直接の対応章はありません ― ここは gradual・実用チェッカーの独自地形です。
> 前編の素朴な `Scope`／`narrow` を、Rigor の本物の **FactStore** へ拡張する章です。

前編 Part 3・4 で、私たちは型環境を `Scope`（変数名→型の Hash）として持ち、ナローイングを
「枝ごとに `Scope` を差し替える」素朴な仕組みで実装しました。実用規模ではこれでは足りません。
何が足りないか、そして Rigor がどう埋めたかを見ます。

---

## 5-1. 「型」だけでなく「事実」を持つ

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

> **`predicate` の `payload` が運ぶもの ― refinement carrier**
>
> 「`arr` は空でない」という事実の `payload`（FactStore が*その変数の型*として供給する値）は、
> Rigor では **`non-empty-array`** という特殊な型キャリアになります。
> `unless s.empty?` を通った文字列は `non-empty-string`、
> `if n > 0` を通った整数は `positive-int`、といった具合です。
>
> これらは前編 Part 1 の `Const[42]`（「値が 42」という超精密な型）とは異なります ―
> refinement carrier は「述語を満たす値の*集合*」を表す型で、値は 1 つに決まっていません。
> FactStore の `payload` フィールドがこれを運ぶことで、「述語を通った後は
> より精密な型で推論できる」という絞り込みが実現します。
>
> PHP のチェッカー PHPStan も全く同名の型（`non-empty-string`・`positive-int`・`literal-string`
> など）を持ちます（用語集「refinement carrier」参照）。同じ命名は偶然ではなく、
> 動的言語チェッカーが同じ問題に同じ答えを出してきた結果です。

---

## 5-2. 6 つの「置き場」（バケツ）

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

> ▼ 図 5-1　FactStore の 6 バケツ。上 5 つは「事実が*どの対象*に付くか」で分ける。
> `dynamic_origin` だけは対象スコープではなく「`untyped` が*どこで生まれたか*」を追う毛色違い。

なぜ分けるか ― **無効化（invalidation）のタイミングが違う**からです。ローカルへの再代入は
その local_binding の事実だけを消せばよいが、メソッド呼び出しは object_content を広く疑う
必要がある、というように。バケツ名は本物の Rigor の内部仕様（`inference-engine.md`）の
正式名と一致します。[^buckets]

> **コラム：ivar の型は「すべての代入の union」**
>
> `object_content` バケツには ivar（インスタンス変数）の型が入ります。Rigor はクラス内の
> `@x` への代入を**すべて収集**し、その型の union を `@x` の型とします：
>
> ```ruby
> class Foo
>   def initialize
>     @x = 1          # Const[1]
>   end
>
>   def reset
>     @x = nil        # Const[nil]
>   end
>
>   def use
>     @x              # => Integer | nil （すべての代入の union）
>   end
> end
> ```
>
> `@x` に書く場所が `initialize` だけなら `Integer`、`reset` が加わると
> `Integer | nil` になります。「どこかで `nil` が代入され得るなら、どこで読んでも
> `nil` を含む」― これは保守的ですが ivar の可視性（どのメソッドから書けるか）が
> ファイルを跨ぐと完全には追えないため、**すべての可視な代入の union** が安全な近似です。
>
> したがって `@x` を `nil` で初期化してすぐ設定するパターンでは `nil?` ガードが
> 必要になります。`object_content` バケツに `non-nil` 事実を追加する、つまり
> `@x` を読む前に `@x.nil?` で分岐するのが Rigor での定石です。

[^buckets]: 6 バケツのうち `local_binding`/`captured_local`/`object_content`/`global_storage`/
`relational` は「事実が付く対象」で分かれるのに対し、`dynamic_origin` だけは「`untyped` の
*由来*を追う」別系統です。位置づけが違うことに注意（実 spec でも 6 つ目として並ぶが役割は別）。

> **コラム：`&&` チェーンで事実が積み上がる**
>
> `&&` 演算子は左から右へ**逐次評価**されるため、FactStore の事実も左から順に積み上げ
> られます：
>
> ```ruby
> if x.is_a?(Integer) && x > 0
>   # ここでは local_binding に 2 つの事実が積まれている
>   #   1. x is_a? Integer   （is_a? ナローイング）
>   #   2. x > 0             （比較述語）
>   # 合成されると x : positive-int と読める
> end
> ```
>
> 左側の `is_a?(Integer)` が通過した時点で `x` の型が `Integer` に絞られ、その状態で
> 右側の `x > 0` が評価されます。「`Integer` かつ `> 0`」が積み重なるので、Rigor は
> これを **`positive-int` リファインメント**として扱えます。
>
> 逆に `||` チェーンは「どちらか一方が成立した場合」なので、合流点で join（共通事実のみ
> 残す）が走り、片方にしかない事実は消えます（§5-5 の合流の話）。`&&` が*足す*、
> `||` が*削る* ― FactStore が左右を対称に扱わない理由です。

> **コラム：正規表現の名前付きキャプチャもナローイングする**
>
> Ruby の `=~` と名前付きキャプチャ（`(?<name>...)`）は、**マッチ成功時にローカル変数へ
> `String` を束縛する**という、他の言語にほぼ無い独自の挙動を持ちます：
>
> ```ruby
> if /(?<year>\d{4})-(?<month>\d{2})/ =~ str
>   # year, month が String として束縛されている
>   year.upcase   # OK（year は String）
> end
> ```
>
> Rigor はこれを**名前付きキャプチャ・ナローイング**として認識します。`if` ブロック内で
> `year` と `month` の `local_binding` に `String` 事実を追加します（マッチ失敗なら
> nil なので、`if` 外ではどちらも `String | nil`）。
>
> Prism では `=~` の左辺が `RegexpNode` かつ名前付きキャプチャを含む場合、Rigor は
> 捕獲グループ名を読み出して FactStore に直接事実を挿入します。`is_a?` の型述語や
> `nil?` の nil ガードと同じ仕組みで、ただし**変数名が正規表現の本文から来る**点が特殊です。
>
> | パターン | ナローイング対象 | 追加される事実 |
> |---|---|---|
> | `is_a?(String)` | 左辺の変数 | `String` |
> | `nil?` 否定 | 左辺の変数 | `non-nil` |
> | `=~` 名前付きキャプチャ | キャプチャ名の変数 | `String` |

---

## 5-3. 事実はいつ消えるか（stability）

前編の素朴な `Scope` には無かった最重要の概念が **stability（安定性）＝事実の寿命**です。
ナローイングで得た事実は、*ある操作で崩れます*：

- **再代入**：`x = …` は `x` に関する local_binding 事実を消す（前編でもやった）。
- **メソッド呼び出し**：`obj.mutate!` は、`obj` の object_content 事実を疑う（中身が変わったかも）。
- **エスケープ**：変数がブロックや別メソッドに渡ると、いつ変更されるか読めない → 保守的に消す。

各事実は「いつまで有効か」を持ち、対応する操作で**保守的に無効化**します。**迷ったら消す**
（緩める側に倒す）― 古い事実を信じて誤検知を出すより、事実を捨てて `untyped` に戻る方が
安全だからです。前編 Part 4 の「絞り込みは事実を足すだけ・間違えたら緩める」を、寿命まで
含めて精密化したものです。

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

> **コラム：エスケープするブロックは「いつ呼ばれるか分からない」**
>
> `each` や `map` のブロックは即時呼び出しなので、ナローイングの事実はブロック終了後まで
> ほぼ保持できます。問題は**エスケープするブロック** ― ブロックが呼び出し元の外に「脱出」する
> 場合です：
>
> ```ruby
> if x.is_a?(Integer)
>   # ここで x の local_binding に "is Integer" が入る
>   Thread.new { x.some_integer_method }   # ← x を捕獲して別スレッドへ
> end
> # Thread がいつ走るかは不明 → x の narrowing を保持し続けるのは危険
> ```
>
> `Thread.new` に渡したブロックは*任意のタイミング*で動きます。その時点で `x` が
> 再代入されていたり、すでに別の型になっている可能性を排除できません。
> FactStore はこの「エスケープ」を検知すると、そのブロックが**捕獲した変数すべての
> `captured_local` 事実を保守的に無効化**します。
>
> `define_method` や `Proc#curry`、`Enumerator` のような「ブロックをオブジェクトとして
> 保存する」パターンも同様に扱います。「即時呼び出しか」「後で呼ばれるか」の判定は
> Rigor が RBS のシグネチャアノテーション（`&block` が `Proc` か `yield` か、等）から
> 推定します。判断できない場合はエスケープと見なして**迷ったら消す**です。

---

## 5-4. クロージャ捕獲という難所

Ruby のブロックは、外側のローカルを**捕獲して書き換え**られます：

```ruby
x = nil
[1, 2, 3].each { |i| x = i }   # ブロックが x を書き換える
# ここで x は nil とは限らない
```

ナローイングが `x` を「nil でない」と絞っても、ブロックが `x` に再代入し得るなら、その事実は
*危うい*。FactStore は、ブロックが**外側のローカルを書く**ことを検知して、その captured_local
事実を無効化します。さらに、ブロックが**いつ呼ばれるか**（即時／遅延／エスケープ）で扱いを
変えます ― `each` は即時呼びだが、保存されて後で呼ばれるブロックは、より保守的に。

前編の素朴な `Scope` はここを*まったく*扱いませんでした（だから本編は `each` 等のブロック内
ナローイングに踏み込まなかった）。実用ではここが誤検知の温床で、Rigor が最も気を遣う所です。

---

## 5-5. 合流（join）― 分岐が合わさるとき

`if` の two 枝が合流したあと、どの事実が生き残るか。答えは「**両方の枝で成り立つ事実だけ**」：

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

## 5-6. それでも不変・フロー感応

ここまで足しても、設計の芯は前編と同じです：

- **不変**：FactStore も `Scope` 同様イミュータブル。`with_fact`/`invalidate` は*新しい*ストアを
  返す。「どの地点で何が成り立つか」が、状態の破壊なしに追える。
- **フロー感応**：事実はプログラムの各点で違う。同じ `x` でも、`if` の中と外で別の事実を持つ。
- **narrowing は事実を*足す*だけ**：型代入ではなく事実の追加（前編 Part 4 の方針のまま）。

---

## 5-7. まとめ

- 型環境を、フロー感応な**事実の集合**＝FactStore に一般化する。
- 事実は対象の種類で **6 バケツ**に分け、**無効化のタイミング**を分ける。
- **stability（寿命）**：再代入・メソッド呼び・エスケープで保守的に消す。迷ったら消す。
- **クロージャ捕獲**：ブロックが外側を書くと事実を無効化。呼ばれ方（即時／遅延）で扱いを変える。
- **join**：分岐合流では両枝で成り立つ事実だけ残す。
- 不変・フロー感応・「事実を足すだけ」は前編から不変。

## 演習

1. **再代入で事実が消える**：`examples/fact_invalidation.rb` で、`x = nil; arr.each { |i| x = i }`
   の後に `x` の「non-nil」事実がどのバケツでなぜ無効化されるべきかを述べよ（`local_binding` か
   `captured_local` か）。
2. **バケツ指定の無効化**：`obj.mutate!` が `obj` の `object_content` 事実だけを落とし、
   `local_binding`（`obj` が User である、など）を残すのが安全な理由を 1 文で。逆に全部消すと
   何が困るか。
3. **join をトレース**：`if cond; x=1 else x="a" end` の合流後、`x` についてどんな事実が残るか
   （両枝の積）。もし「片方で成り立つ事実」も残したら、なぜ誤検知になるかを述べよ。

**次章（Part 6）**：前編 Part 7 の RBS で*なぞった*型変数を、本式に。型代入 `subst`・α 同値・
変数捕獲を `fresh` 変数で避ける ― ジェネリクスの本丸へ。
