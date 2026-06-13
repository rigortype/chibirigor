---
title: The Little chibirigor Part 4 ― Union：型が一本に決まらない
description: "型が一本に決まらないときの型 `Union` を導入し、`if`／三項の枝の型を 1 つにまとめる。"
sidebar:
  order: 5
---

# The Little chibirigor Part 4 ― Union：型が一本に決まらない

この章のゴール：**型が一本に決まらないときの型 `Union` を導入する。** Ruby では `if` や
三項演算子で枝ごとに別々の型を返すのが日常です。そのとき型を 1 本に決めつけず、「どちらか」
としてまとめて持つ ― それが Union です。

> 私たちが作る Union（`Integer | String` のような**無タグの合併型**）は、実は参考書が*あえて
> 避けた*出発点です。『しくみ』も TAPL も、持っているのは値に札を付けて区別する*タグ付き*の
> variant であって、無タグの Union とは別物 ― でも Ruby を相手にする私たちには、無タグの
> Union が必須です。（この違いは付録 [a5-4](../appendix/a5-other-languages.md) へ。）

---

## 4-1. 型が一本に決まらない ― Union

こんな Ruby を考えます：

```ruby
x = rand < 0.5 ? 1 : "a"
```

`x` の型は `Integer`？ `String`？ ── **どちらにもなり得る**。こういうとき、型を一本に
決めず「`Integer` か `String` のどちらか」という型にします。これが **Union**：

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

`union` の小道具がやっているのは 2 つだけです。**入れ子をならす**（`Union` の中に
`Union` が来たら平らにする）、そして**重複を消す**（同じ型が 2 度出てきたら 1 つにする）。
まとめた結果がメンバ 1 個になったら、わざわざ `Union` で包まず、その型そのものを返します
（`Integer | Integer` はただの `Integer`）。

`if`（三項演算子も Prism では同じ `IfNode`）の型は、**then 節と else 節の型をまとめた
もの**にします：

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

`nil` も `Const[nil]` というふつうの型として扱い、else の無い `if` は「偽のとき `nil`」を
そのまま Union に混ぜます。だから `c ? 1 : nil` も `if cond then 1 end` も、素直に `1 | nil` です。

`annotate`／`type_of` で確かめると、ちゃんと Union が出ます：

```ruby
type_of(parse("rand < 0.5 ? 1 : \"a\""))   # => 1 | "a"（両枝とも Const のまま union）
```

- **① 型理論**：値が複数の型になり得るとき＝合併型（『しくみ』はあえて避けた領域）。
- **② Ruby だと**：分岐で別々の型を返すのは日常。`x = cond ? 1 : "a"` は普通に書く。
- **③ Rigor だと**：一本に決めず Union で持つ。決めつけない＝後で困らない。

> Union のメンバには `nil`（`NilClass`）も普通に並びます。「`User` か `nil`」のような
> `User | nil` ―『見つかれば値、なければ nil』という Ruby で頻出の形も、ただの Union です。
> この `nil` を含む Union を*どう剥がすか*が、次章 Part 5 の主役になります。

> Union から何かを読むとき（例：`(Integer | String).to_s`）は、**メンバを 1 つずつ考えて
> まとめる**のが基本です ― この「全メンバを回して一番弱い結論を採る」考え方は、Part 7 の
> `accepts`（`:yes`/`:no`/`:maybe`）でそのまま再登場します。頭の片隅に置いておいてください。

> 「絞り尽くして候補がゼロになった枝」には**ボトム型**（`never`）という型理論上の名前が
> あります。本書本体ではボトム型を*型としては*作らず、その話は付録 a1 にまとめました。

---

## 4-1x. 発展：Union レシーバへのメソッド送信（分配して畳む）

> これは本筋から外した**発展ノート**です。この章は Union を*作る*話に集中し、できた Union に
> メソッドを送る話には踏み込みませんでした。ここでは Part 2 のディスパッチ表に*ひとさじ*足すと、
> Union レシーバがどう振る舞うかを重ねます。4-1 の `union` も `IfNode` の型付けもそのままです。

`x = cond ? 1 : 2` で `x` は `1 | 2`。では `x + 1` の型は？ この章の最小版（と Part 2 の素朴な
ディスパッチ表）は、レシーバの型を `class_of` で 1 つのクラス名に丸めて表を引きます。`Union` は
クラス名に丸まらない（`class_of` が `nil`）ので、**表が引けず黙って `untyped` に倒れる** ―
fail-soft の出口です。脅かしはしませんが、せっかくの `1 | 2` の精度は捨ててしまいます。

実物の `exe/chibirigor` は、ここで一歩踏み込みます。**Union レシーバはメンバごとに表を引き、
出てきた戻り型を `Type.union` で畳む**（`lib/chibirigor/dispatch.rb` の `dispatch_union`）：

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

引数側に Union が来ても同じ発想です。2-7 の定数畳み込み段は、引数を**メンバの直積**に展開して
組ごとに畳みます（`const_combinations`）― `1 + (1 | 2)` なら `1+1` と `1+2` を両方計算して `2 | 3`。
実際に動かすと、レシーバ分配も引数分配もこう出ます（`exe/chibirigor annotate`）：

```text
x = cond ? 1 : 2 ; x + 1        # 2: 2 | 3      （レシーバ (1|2) を分配して畳む）
a = 1 ; a + (cond ? 1 : 2)      # 2: 2 | 3      （引数 (1|2) を直積に展開して畳む）
x = cond ? 1 : "a" ; x + 1      # 2: 2 | String （Integer 側は畳み、String 側は表の戻り型へ）
```

この挙動は誤検知ゼロの原則と地続きです。**分配の結果が割れたら**どうするか ―
`x = cond ? 1 : "a"` の `x + 1` は、`1 + 1` は通り `"a" + 1` は型エラー。でも実行時には `x` が
`Integer` 側に転んでいれば動きます。だから**全メンバで失敗したときだけ怒り、一部だけの失敗は
黙る**（`:maybe`）。`x + "a"` のように `(1 | 2)` のどちらでも失敗する式だけが、診断 1 件になります。
**未知メンバがいたら**さらに保守的で、`x = cond ? 1 : nil` の `x + 1` は `nil.+` が表に無い時点で、
Union 全体を `untyped` に倒します（一部でも型を見失えば、全体の精度を主張しない）。

実物の挙動は **`test/test_union_dispatch.rb`** が仕様兼サンプルです（レシーバ分配・引数の直積・
全メンバ失敗時だけ怒る・未知メンバで untyped・メンバ数予算でクラスに丸める、を網羅）。
4-1 の `annotate` 出力（`rand < 0.5 ? 1 : "a"` が `1 | "a"`）の*続き*として、その `x` に
メソッドを送ると分配が起きる、と読んでください。手元の `exe/chibirigor` で `(1 | 2) + 1` が
`2 | 3` と出る（章の最小版なら `untyped`）のは、この分配が Dispatch 側に入っているからです。

> **実 Rigor では**、`Union` レシーバは「各メンバを個別にディスパッチし、全メンバが解決したら
> 戻り型を union、どれか 1 つでも解決しなければ全体を `nil`（解決失敗）にする」と定めています
> （`rigor/docs/internal-spec/inference-engine.md`「`Union` receivers MUST dispatch each member
> individually」）。chibirigor の「未知メンバがいたら全体を untyped」は、この縮図です。

---

## 4-2. この章のまとめ

足したもの：型キャリア `Union` ひとつ、まとめ道具 `union`、そして `IfNode` の型付け
（then 節と else 節の型を `union` する）。`union` の小道具は「入れ子をならす・重複を消す」の
2 つだけ。これで「型が一本に決まらない」Ruby を、型のレベルで素直に表せるようになりました。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論 | 値が複数の型になり得る＝合併（『しくみ』が*あえて避けた*領域。TAPL も直接の章なし） |
| ② Ruby/RBS | 分岐で別々の型を返すのは日常。`x = cond ? 1 : "a"` も `User | nil` も普通に書く |
| ③ Rigor 実装の問題 | 一本に決めつけず Union で持つ。決めつけない＝後で困らない |

<!-- run: examples/part4.rb -->
```text
c ? 1 : "a"  ->  1 | "a"
c ? 1 : nil   ->  1 | nil
```

## 演習

1. `rand < 0.5 ? 1 : 2` の型を `annotate` で確かめると `1 | 2` になる（両枝とも `Const` のまま）。
   では `rand < 0.5 ? 1 : 1` なら何になるか ―『union』の小道具が*同じメンバ*をどう畳むかで説明せよ。
2. else の無い `if cond\n  1\nend` の型を `annotate` で確かめると `1 | nil` になる
   （実際の Ruby が、else 無しの `if` が偽のとき `nil` を返すのに合わせている）。
   `union` がこの 2 つをどうまとめるか、メンバの並びで説明せよ。
3. `Union[[Integer, Union[[String, Integer]]]]` を `union` に通すと何が返るか。
   「入れ子をならす」「重複を消す」「メンバ 1 個なら包まない」の 3 つを順に当てはめて答えよ。

---

**次章予告（Part 5）**：Union は型を*増やす*操作でした。次章ではその逆 ― Union を*減らす*
**ナローイング（絞り込み）**を作ります。`if x.nil?` の else 節で「ここの `x` はもう `nil`
じゃない」と型を狭める、あの当たり前を型でも追えるようにします。偽は `false`/`nil` の 2 つだけ・
`narrow` の実装・`is_a?` の dead branch・絞り込みの 2 つの掟・再代入でのリセットまで、
そこで扱います。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part4/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part4/lib)
