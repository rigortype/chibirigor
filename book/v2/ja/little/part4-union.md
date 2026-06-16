---
title: Part 4　Union：型が一本に決まらない
description: "型が一本に決まらないときの型 `Union` を導入し、`if`／三項の枝の型を 1 つにまとめる。"
sidebar:
  order: 5
---

# The Little chibirigor Part 4　Union：型が一本に決まらない

この章のゴールは、型が一本に決まらないときの型 `Union` を導入することです。
Ruby では `if` や三項演算子で枝ごとに別々の型を返すのが日常です。
そのとき型を 1 本に決めつけず、「どちらか」としてまとめて持ちます。
それが Union です。

> 私たちが作る Union（`Integer | String` のような**無タグのユニオン型**）は、実は参考書があえて避けた出発点です。
> 『しくみ』も TAPL も、持っているのは値に札を付けて区別するタグ付きの variant であって、無タグの Union とは別物です。
> でも Ruby を相手にする私たちには、無タグの Union が必須です。（この違いは付録 [a5-4](../appendix/a5-other-languages.md) へ。）

---

## 4-1. Union：型が一本に決まらない

こんな Ruby を考えます。

```ruby
x = rand < 0.5 ? 1 : "a"
```

`x` の型は `Integer`？ `String`？ どちらにもなり得ます。
こういうとき、型を一本に決めず「`Integer` か `String` のどちらか」という型にします。
これが **Union** です。

```ruby
module Type
  Union = Data.define(:members) do
    def to_s = members.map(&:to_s).join(" | ")   # 例: "Integer | String"
  end

  module_function

  # 型をまとめる小さな道具。入れ子をならし、重複を消す
  def union(types)
    flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
    flat.size == 1 ? flat.first : Union[flat]
  end
end
```

`union` の小道具がやっているのは 2 つだけです。
**入れ子をならす**（`Union` の中に `Union` が来たら平らにする）、そして**重複を消す**（同じ型が 2 度出てきたら 1 つにする）です。
まとめた結果がメンバ 1 個になったら、わざわざ `Union` で包まず、その型そのものを返します（`Integer | Integer` はただの `Integer`）。

`if`（三項演算子も Prism では同じ `IfNode`）の型は、then 節と else 節の型をまとめたものにします。

```ruby
when Prism::NilNode
  Type::Const[nil]            # nil リテラルの型。Union のメンバに普通に並ぶ
when Prism::IfNode
  then_type = type_of(node.statements.body.last, scope, diagnostics)
  else_type =
    if node.subsequent        # else（や elsif）があるか
      type_of(node.subsequent.statements.body.last, scope, diagnostics)
    else
      Type::Const[nil]        # else が無ければ、偽のとき nil ― 実際の Ruby に合わせる
    end
  Type.union([then_type, else_type])
```

> ここで `node.subsequent` は、`else` 節なら `Prism::ElseNode`（`elsif` なら `IfNode`）です。
> 型はその `.statements.body.last`（その節の最後の式）から求めます。
> `node.subsequent` をそのまま `type_of` に渡さない点に注意してください（渡すと未知のノードとして `untyped` に落ちてしまいます）。

`nil` も `Const[nil]` というふつうの型として扱い、else の無い `if` は「偽のとき `nil`」をそのまま Union に混ぜます。
だから `c ? 1 : nil` も `if cond then 1 end` も、素直に `1 | nil` です。

`annotate`／`type_of` で確かめると、ちゃんと Union が出ます。

```ruby
type_of(parse("rand < 0.5 ? 1 : \"a\""))   # => 1 | "a"（両枝とも Const のまま union）
```

![図 4-1　if／三項の型（then と else をまとめて Union）](../figures/svg/little-4-1.svg)
> ▼ 図 4-1　`if`／三項の型（then と else をまとめて Union、図 5-1 の逆向き）

- **① 型理論**：値が複数の型になり得るとき＝ユニオン型（『しくみ』はあえて避けた領域）。
- **② Ruby だと**：分岐で別々の型を返すのは日常です。`x = cond ? 1 : "a"` は普通に書きます。
- **③ Rigor だと**：一本に決めず Union で持ちます。決めつけないことが、後で困らないことにつながります。

> Union のメンバには `nil`（`NilClass`）も普通に並びます。
> 「`User` か `nil`」のような `User | nil` は、「見つかれば値、なければ nil」という Ruby で頻出の形であり、ただの Union です。
> この `nil` を含む Union をどう剥がすかが、次章 Part 5 の主役になります。

> Union から何かを読むとき（例：`(Integer | String).to_s`）は、メンバを 1 つずつ考えてまとめるのが基本です。
> この「全メンバを回して一番弱い結論を採る」考え方は、Part 7 の `accepts`（`:yes`/`:no`/`:maybe`）でそのまま再登場します。
> 頭の片隅に置いておいてください。

> 「絞り尽くして候補がゼロになった枝」には**ボトム型**（`never`）という型理論上の名前があります。
> 本書本体ではボトム型を型としては作らず、その話は付録 a1 にまとめました。

---

## 4-1x. 発展：Union レシーバへのメソッド送信（分配して畳む）

> これは本筋から外した発展ノートです。
> この章は Union を作る話に集中し、できた Union にメソッドを送る話には踏み込みませんでした。
> Part 2 のディスパッチ表にひとさじ足すと、Union レシーバがどう振る舞うかを扱います。
> 4-1 の `union` も `IfNode` の型付けもそのままです。

`x = cond ? 1 : 2` で `x` は `1 | 2` です。では `x + 1` の型は？
この章の最小版（と Part 2 の素朴なディスパッチ表）は、レシーバの型を `class_of` で 1 つのクラス名に丸めて表を引きます。
`Union` はクラス名に丸まらない（`class_of` が `nil`）ので、表が引けず黙って `untyped` に倒れます。
fail-soft の出口ですが、せっかくの `1 | 2` の精度は捨ててしまいます。

実物の `exe/chibirigor` は、ここで一歩踏み込みます。
**Union レシーバはメンバごとに表を引き、出てきた戻り型を `Type.union` で畳みます**（`lib/chibirigor/dispatch.rb` の `dispatch_union`）。

```ruby
# Union レシーバの分配ディスパッチ。実行時はどのメンバにもなり得るので、
# メンバごとに dispatch して結果を union で畳む。
def dispatch_union(receiver_type, name, arg_types, node, diagnostics)
  buffers = []
  results = receiver_type.members.map do |member|
    buffers << (buffer = [])
    dispatch(member, name, arg_types, node, buffer)   # メンバ 1 つずつ表を引く
  end
  diagnostics.concat(merge_member_diagnostics(buffers))
  budgeted_union(results)                              # 結果を畳む（重なれば 1 つに）
end
```

引数側に Union が来ても同じ発想です。
2-7 の定数畳み込み段は、引数をメンバの直積に展開して組ごとに畳みます（`const_combinations`）。
`1 + (1 | 2)` なら `1+1` と `1+2` を両方計算して `2 | 3` になります。
実際に動かすと、レシーバ分配も引数分配もこう出ます（`exe/chibirigor annotate`）。

```text
x = cond ? 1 : 2 ; x + 1        # 2: 2 | 3      （レシーバ (1|2) を分配して畳む）
a = 1 ; a + (cond ? 1 : 2)      # 2: 2 | 3      （引数 (1|2) を直積に展開して畳む）
x = cond ? 1 : "a" ; x + 1      # 2: 2 | String （Integer 側は畳み、String 側は表の戻り型へ）
```

この挙動は誤検知ゼロの原則と地続きです。
分配の結果が割れたらどうするか。
`x = cond ? 1 : "a"` の `x + 1` は、`1 + 1` は通り `"a" + 1` は型エラーです。
でも実行時には `x` が `Integer` 側に転んでいれば動きます。
だから**全メンバで失敗したときだけ怒り、一部だけの失敗は黙ります**（`:maybe`）。
`x + "a"` のように `(1 | 2)` のどちらでも失敗する式だけが、診断 1 件になります。
未知メンバがいたらさらに保守的で、`x = cond ? 1 : nil` の `x + 1` は `nil.+` が表に無い時点で、Union 全体を `untyped` に倒します（一部でも型を見失えば、全体の精度を主張しません）。

実物の挙動は **`test/test_union_dispatch.rb`** が仕様兼サンプルです（レシーバ分配、引数の直積、全メンバ失敗時だけ怒る、未知メンバで untyped、メンバ数予算でクラスに丸めるを網羅）。
4-1 の `annotate` 出力（`rand < 0.5 ? 1 : "a"` が `1 | "a"`）の続きとして、その `x` にメソッドを送ると分配が起きると読んでください。
手元の `exe/chibirigor` で `(1 | 2) + 1` が `2 | 3` と出る（章の最小版なら `untyped`）のは、この分配が Dispatch 側に入っているからです。

> **実 Rigor では**、`Union` レシーバは「各メンバを個別にディスパッチし、全メンバが解決したら戻り型を union、どれか 1 つでも解決しなければ全体を `nil`（解決失敗）にする」と定めています
> （`rigor/docs/internal-spec/inference-engine.md`「`Union` receivers MUST dispatch each member
> individually」）。
> chibirigor の「未知メンバがいたら全体を untyped」は、この縮図です。

---

## 4-2. この章のまとめ

足したものは、型キャリア `Union` ひとつ、まとめ道具 `union`、そして `IfNode` の型付け（then 節と else 節の型を `union` する）です。
`union` の小道具は「入れ子をならす、重複を消す」の 2 つだけです。
これで「型が一本に決まらない」Ruby を、型のレベルで素直に表せるようになりました。

動かすとこうなります。

```ruby
x_int_str = Chibirigor.annotate("x = c ? 1 : \"a\"\nx\n").last[:type]
x_int_nil = Chibirigor.annotate("x = c ? 1 : nil\nx\n").last[:type]
puts "c ? 1 : \"a\"  ->  #{x_int_str}"
puts "c ? 1 : nil   ->  #{x_int_nil}"
```

<!-- run: examples/part4.rb -->
```text
c ? 1 : "a"  ->  1 | "a"
c ? 1 : nil   ->  1 | nil
```

`c ? 1 : "a"` は then 節が `1`、else 節が `"a"` です。
どちらか一方に決めつけず `1 | "a"` という Union にまとめます。
else 側が `nil` でも同じく `1 | nil` です。

この章の三つの視点：

| | 内容 |
|---|---|
| ① 型理論 | 値が複数の型になり得る＝ユニオン型（『しくみ』があえて避けた領域。TAPL も直接の章なし） |
| ② Ruby/RBS | 分岐で別々の型を返すのは日常。`x = cond ? 1 : "a"` も `User | nil` も普通に書く |
| ③ Rigor 実装の問題 | 一本に決めつけず Union で持つ。決めつけない＝後で困らない |

## 演習

1. `rand < 0.5 ? 1 : 2` の型を `annotate` で確かめると `1 | 2` になる（両枝とも `Const` のまま）。
   では `rand < 0.5 ? 1 : 1` なら何になるか。`union` の小道具が同じメンバをどう畳むかで説明せよ。
2. else の無い `if cond\n  1\nend` の型を `annotate` で確かめると `1 | nil` になる
   （実際の Ruby が、else 無しの `if` が偽のとき `nil` を返すのに合わせている）。
   `union` がこの 2 つをどうまとめるか、メンバの並びで説明せよ。
3. `Union[[Integer, Union[[String, Integer]]]]` を `union` に通すと何が返るか。
   「入れ子をならす」「重複を消す」「メンバ 1 個なら包まない」の 3 つを順に当てはめて答えよ。

---

**次章予告（Part 5）**：Union は型を増やす操作でした。
次章ではその逆、Union を減らす**ナローイング（絞り込み）**を作ります。
`if x.nil?` の else 節で「ここの `x` はもう `nil` じゃない」と型を狭める、あの当たり前を型でも追えるようにします。
偽は `false`/`nil` の 2 つだけ、`narrow` の実装、`is_a?` の dead branch、絞り込みの 2 つの掟、再代入でのリセットまで、そこで扱います。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part4/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part4/lib)
