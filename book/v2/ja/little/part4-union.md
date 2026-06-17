---
title: Part 4　Union：型が一本に決まらない
description: "型が一本に決まらないときの型 `Union` を導入し、`if`／三項の枝の型を 1 つにまとめる。"
sidebar:
  order: 5
---

# The Little chibirigor Part 4　Union：型が一本に決まらない

この章のゴールは、型が一本に決まらないときの型`Union`を導入することです。Rubyでは`if`や三項演算子で枝ごとに別々の型を返すのが日常です。そのとき型を1本に決めつけず、「どちらか」としてまとめて持ちます。それがUnionです。

> [!NOTE]
> 私たちが作るUnion（`Integer | String`のような**無タグのユニオン型**）は、実は参考書があえて避けた出発点です。『しくみ』もTAPLも、持っているのは値に札を付けて区別するタグ付きのvariantであって、無タグのUnionとは別物です。でもRubyを相手にする私たちには、無タグのUnionが必須です。（この違いは付録[a5-4](../appendix/a5-other-languages.md)へ。）

---

## 4-1. Union：型が一本に決まらない

こんなRubyを考えます。

```ruby
x = rand < 0.5 ? 1 : "a"
```

`x`の型は`Integer`？ `String`？ どちらにもなり得ます。こういうとき、型を一本に決めず「`Integer`か`String`のどちらか」という型にします。これが**Union**です。

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

`union`の小道具がやっているのは2つだけです。**入れ子をならす**（`Union`の中に`Union`が来たら平らにする）、そして**重複を消す**（同じ型が2度出てきたら1つにする）です。まとめた結果がメンバ1個になったら、わざわざ`Union`で包まず、その型そのものを返します（`Integer | Integer`はただの`Integer`）。

`if`（三項演算子もPrismでは同じ`IfNode`）の型は、then節とelse節の型をまとめたものにします。

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

> [!NOTE]
> ここで`node.subsequent`は、`else`節なら`Prism::ElseNode`（`elsif`なら`IfNode`）です。型はその`.statements.body.last`（その節の最後の式）から求めます。`node.subsequent`をそのまま`type_of`に渡さない点に注意してください。渡すと未知のノードとして`untyped`に落ちてしまいます。

`nil`も`Const[nil]`というふつうの型として扱い、elseの無い`if`は「偽のとき`nil`」をそのままUnionに混ぜます。だから`c ? 1 : nil`も`if cond then 1 end`も、素直に`1 | nil`です。

`annotate`／`type_of`で確かめると、ちゃんとUnionが出ます。

```ruby
type_of(parse("rand < 0.5 ? 1 : \"a\""))   # => 1 | "a"（両枝とも Const のまま union）
```

![図4-1　if／三項の型（thenとelseをまとめてUnion）](../figures/svg/little-4-1.svg)
> ▼ 図4-1　`if`／三項の型（thenとelseをまとめてUnion、図5-1の逆向き）

- **① 型理論**：値が複数の型になり得るとき＝ユニオン型（『しくみ』はあえて避けた領域）。
- **② Rubyだと**：分岐で別々の型を返すのは日常です。`x = cond ? 1 : "a"`は普通に書きます。
- **③ Rigorだと**：一本に決めずUnionで持ちます。決めつけないことが、後で困らないことにつながります。

> [!NOTE]
> Unionのメンバには`nil`（`NilClass`）も普通に並びます。「`User`か`nil`」のような`User | nil`は、「見つかれば値、なければnil」というRubyで頻出の形であり、ただのUnionです。この`nil`を含むUnionをどう剥がすかが、次章Part 5の主役になります。

> [!NOTE]
> Unionから何かを読むとき（例：`(Integer | String).to_s`）は、メンバを1つずつ考えてまとめるのが基本です。この「全メンバを回して一番弱い結論を採る」考え方は、Part 7の`accepts`（`:yes`/`:no`/`:maybe`）でそのまま再登場します。頭の片隅に置いておいてください。

> [!NOTE]
> 「絞り尽くして候補がゼロになった枝」には**ボトム型**（`never`）という型理論上の名前があります。本書本体ではボトム型を型としては作らず、その話は付録[a1](../appendix/a1-special-types.md)にまとめました。

---

## 4-1x. 発展：Unionレシーバへのメソッド送信（分配して畳む）

> [!NOTE]
> これは本筋から外した発展ノートです。この章はUnionを作る話に集中し、できたUnionにメソッドを送る話には踏み込みませんでした。Part 2のディスパッチ表にひとさじ足すと、Unionレシーバがどう振る舞うかを扱います。4-1の`union`も`IfNode`の型付けもそのままです。

`x = cond ? 1 : 2`で`x`は`1 | 2`です。では`x + 1`の型は？ この章の最小版（とPart 2の素朴なディスパッチ表）は、レシーバの型を`class_of`で1つのクラス名に丸めて表を引きます。`Union`はクラス名に丸まらない（`class_of`が`nil`）ので、表が引けず黙って`untyped`に倒れます。fail-softの出口ですが、せっかくの`1 | 2`の精度は捨ててしまいます。

実物の`exe/chibirigor`は、ここで一歩踏み込みます。**Unionレシーバはメンバごとに表を引き、出てきた戻り型を`Type.union`で畳みます**（`lib/chibirigor/dispatch.rb`の`dispatch_union`）。

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

引数側にUnionが来ても同じ発想です。2-7の定数畳み込み段は、引数をメンバの直積に展開して組ごとに畳みます（`const_combinations`）。`1 + (1 | 2)`なら`1+1`と`1+2`を両方計算して`2 | 3`になります。

実際に動かすと、レシーバ分配も引数分配もこう出ます（`exe/chibirigor annotate`）。

```text
x = cond ? 1 : 2 ; x + 1        # 2: 2 | 3      （レシーバ (1|2) を分配して畳む）
a = 1 ; a + (cond ? 1 : 2)      # 2: 2 | 3      （引数 (1|2) を直積に展開して畳む）
x = cond ? 1 : "a" ; x + 1      # 2: 2 | String （Integer 側は畳み、String 側は表の戻り型へ）
```

この挙動は誤検知ゼロの原則と地続きです。分配の結果が割れたらどうするか。`x = cond ? 1 : "a"`の`x + 1`は、`1 + 1`は通り`"a" + 1`は型エラーです。でも実行時には`x`が`Integer`側に転んでいれば動きます。だから**全メンバで失敗したときだけ怒り、一部だけの失敗は黙ります**（`:maybe`）。`x + "a"`のように`(1 | 2)`のどちらでも失敗する式だけが、診断1件になります。

未知メンバがいたらさらに保守的で、`x = cond ? 1 : nil`の`x + 1`は`nil.+`が表に無い時点で、Union全体を`untyped`に倒します。一部でも型を見失えば、全体の精度を主張しません。

実物の挙動は **`test/test_union_dispatch.rb`** が仕様兼サンプルです（レシーバ分配、引数の直積、全メンバ失敗時だけ怒る、未知メンバでuntyped、メンバ数予算でクラスに丸めるを網羅）。4-1の`annotate`出力（`rand < 0.5 ? 1 : "a"`が`1 | "a"`）の続きとして、その`x`にメソッドを送ると分配が起きると読んでください。手元の`exe/chibirigor`で`(1 | 2) + 1`が`2 | 3`と出る（章の最小版なら`untyped`）のは、この分配がDispatch側に入っているからです。

> [!NOTE]
> **実Rigorでは**、`Union`レシーバは「各メンバを個別にディスパッチし、全メンバが解決したら戻り型をunion、どれか1つでも解決しなければ全体を`nil`（解決失敗）にする」と定めています（`rigor/docs/internal-spec/inference-engine.md`「`Union` receivers MUST dispatch each member
> individually」）。chibirigorの「未知メンバがいたら全体をuntyped」は、この縮図です。

---

## 4-2. この章のまとめ

足したものは、型キャリア`Union`ひとつ、まとめ道具`union`、そして`IfNode`の型付け（then節とelse節の型を`union`する）です。`union`の小道具は「入れ子をならす、重複を消す」の2つだけです。これで「型が一本に決まらない」Rubyを、型のレベルで素直に表せるようになりました。

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

`c ? 1 : "a"`はthen節が`1`、else節が`"a"`です。どちらか一方に決めつけず`1 | "a"`というUnionにまとめます。else側が`nil`でも同じく`1 | nil`です。

この章の三つの視点：

| | 内容 |
|---|---|
| ① 型理論 | 値が複数の型になり得る＝ユニオン型（『しくみ』があえて避けた領域。TAPLも直接の章なし） |
| ② Ruby/RBS | 分岐で別々の型を返すのは日常。`x = cond ? 1 : "a"`も`User | nil`も普通に書く |
| ③ Rigor実装の問題 | 一本に決めつけずUnionで持つ。決めつけない＝後で困らない |

## 演習

1. `rand < 0.5 ? 1 : 2`の型を`annotate`で確かめると`1 | 2`になる（両枝とも`Const`のまま）。
   では`rand < 0.5 ? 1 : 1`なら何になるか。`union`の小道具が同じメンバをどう畳むかで説明せよ。
2. elseの無い`if cond\n  1\nend`の型を`annotate`で確かめると`1 | nil`になる
   （実際のRubyが、else無しの`if`が偽のとき`nil`を返すのに合わせている）。
   `union`がこの2つをどうまとめるか、メンバの並びで説明せよ。
3. `Union[[Integer, Union[[String, Integer]]]]`を`union`に通すと何が返るか。
   「入れ子をならす」「重複を消す」「メンバ1個なら包まない」の3つを順に当てはめて答えよ。

---

**次章予告（Part 5）**：Unionは型を増やす操作でした。次章ではその逆、Unionを減らす **ナローイング（絞り込み）** を作ります。`if x.nil?`のelse節で「ここの`x`はもう`nil`じゃない」と型を狭める、あの当たり前を型でも追えるようにします。偽は`false`/`nil`の2つだけ、`narrow`の実装、`is_a?`のdead branch、絞り込みの2つの掟、再代入でのリセットまで、そこで扱います。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part4/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part4/lib)
