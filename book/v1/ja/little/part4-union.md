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
Union = Data.define(:members) do
  def to_s = members.map(&:to_s).join(" | ")   # 例: "Integer | String"
end

# 型をまとめる小さな道具。入れ子をならし、重複を消す
def union(types)
  flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
  flat.size == 1 ? flat.first : Union[flat]
end
```

`union` の小道具がやっているのは 2 つだけです。**入れ子をならす**（`Union` の中に
`Union` が来たら平らにする）、そして**重複を消す**（同じ型が 2 度出てきたら 1 つにする）。
まとめた結果がメンバ 1 個になったら、わざわざ `Union` で包まず、その型そのものを返します
（`Integer | Integer` はただの `Integer`）。

`if`（三項演算子も Prism では同じ `IfNode`）の型は、**then 節と else 節の型をまとめた
もの**にします：

```ruby
when Prism::IfNode
  then_type = type_of(node.statements.body.last, scope, diagnostics)
  else_type = type_of(node.subsequent.statements.body.last, scope, diagnostics)
  union([then_type, else_type])
```

`annotate`／`type_of` で確かめると、ちゃんと Union が出ます：

```ruby
type_of(parse("rand < 0.5 ? 1 : \"a\""))   # => Integer | String
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

1. `rand < 0.5 ? 1 : 2` の型を `annotate` で確かめ、なぜ `Integer | Integer` ではなく
   `Integer` になるのか、`union` の小道具の動きで説明せよ。
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
