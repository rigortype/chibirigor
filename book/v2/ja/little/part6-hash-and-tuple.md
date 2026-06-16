---
title: Part 6　ハッシュと配列の型
description: "ハッシュと配列のリテラルに構造的な型（`HashShape`／`Tuple`）をつけ、部分一致を許す設計にする。"
sidebar:
  order: 7
---

# The Little chibirigor Part 6　ハッシュと配列の型

前章（Part 5）ではUnionを場合分けで絞り込み、`if`の枝ごとに型を細くしました。今度は値の*中身*に目を向けます。ハッシュと配列のリテラルに構造的な型（`HashShape`／`Tuple`）をつけ、そこから値を読み出す型を求めます。Rubyのコードは「symbolキーのハッシュ」だらけなので、ここをうまく扱えると一気に実用的になります。

> [!NOTE]
> 『しくみ』5章「オブジェクト型」（TAPL 11章 §11.8「レコード」、§11.7「組」）に対応します。同書は同じものを`{ tag: "Object", props }`という型で表しました。私たちもほぼ同じことをRubyでやります。最後に一つ、`HashShape`を**open**（余分なキーを許す）にするという判断もします。

---

## 6-1. リテラルから型を起こす（HashShapeとTuple）

`{ foo: 1, bar: "a" }`の型は何でしょう。「`Hash`」では大ざっぱすぎます。**どのキーに何の型が入っているか**まで覚えたい。それが`HashShape`です。

```ruby
module Type
  HashShape = Data.define(:fields) do   # fields: { foo: Const[1], bar: Const["a"] }
    def to_s = "{" + fields.map { |k, v| "#{k}: #{v}" }.join(", ") + "}"
  end

  Tuple = Data.define(:elements) do     # 配列を「位置ごとの型」で覚える
    def to_s = "[" + elements.map(&:to_s).join(", ") + "]"
  end
end
```

`type_of`に2つcaseを足すだけです。Prismではハッシュは`HashNode`（各ペアが`AssocNode`、symbolキーは`SymbolNode`）、配列は`ArrayNode`です。

```ruby
when Prism::HashNode
  fields = node.elements.to_h { |a| [a.key.unescaped.to_sym, type_of(a.value, scope, diag)] }
  Type::HashShape[fields]
when Prism::ArrayNode
  Type::Tuple[node.elements.map { |el| type_of(el, scope, diag) }]
```

```ruby
type_of(parse(%q[{ foo: 1, bar: "a" }]))   # => {foo: 1, bar: "a"}
type_of(parse(%q[[1, "x"]]))               # => [1, "x"]
```

三つの視点で整理します。

- **① 型理論**：複数の値をラベルでまとめた型がレコード型です（『しくみ』 5章）。
- **② Rubyだと**：symbolキーのハッシュが至る所に使われ、配列もタプル的に使われます（`[name, age]`）。
- **③ Rigorだと**：`Hash`で丸めず、キーごと、位置ごとの型を覚えます（Part 1の「細かく覚える」の延長）。

---

## 6-2. 読み出す（`h[:foo]`と`a[0]`）

型に「どのキーが何の型か」が入っているので、読み出しは素直です。`h[:foo]`はPrismでは`[]`というメソッド送信です（`h.[](:foo)`）。引数が**リテラルのsymbolまたは整数**なら、型から引けます。

```ruby
def read_index(receiver, arg_node)
  if receiver.is_a?(Type::HashShape) && arg_node.is_a?(Prism::SymbolNode)
    # 未知キーは nil（実 Ruby が nil を返すから。エラーにしない）
    return receiver.fields.fetch(arg_node.unescaped.to_sym, Type::Const[nil])
  end
  if receiver.is_a?(Type::Tuple) && arg_node.is_a?(Prism::IntegerNode)
    return receiver.elements.fetch(arg_node.value, Type::Const[nil])
  end
  nil   # 特別扱いできない → 通常のディスパッチに回す
end
```

```ruby
# h : {foo: 1, bar: "a"} のとき
h[:foo]   # => 1     （Const[1]）
h[:zzz]   # => nil   （★エラーにしない）
a[0]      # => 1
a[9]      # => nil
```

`h[:zzz]`で**エラーを出さない**のがポイントです。理由は単純で、**実際のRubyが`{foo: 1}[:zzz]`で`nil`を返すから**です。存在しないキーの読みは「バグ」ではなく「nilが返る」が*正しい*挙動です。型もそれに合わせて`nil`を返します。

---

## 6-3. openかclosedか（余分なキーを許す）

ここがPart 6の山です。こういうRubyを考えます。

```ruby
def greet(user)        # user は { name: ... } を期待しているとする
  "Hello, #{user[:name]}"
end

greet({ name: "Alice", admin: true })   # ★ name 以外に admin も入っている
```

`greet`が欲しいのは`name`だけです。でも渡されたハッシュには`admin`も入っています。これは適合とすべきでしょうか、不適合とすべきでしょうか。

型の*等価*なら「プロパティが*完全一致*していないとダメ」ですが、**部分型**なら話は別です。「`{name:}`が欲しい所に`{name:, admin:}`を渡せる」のは健全で、これを**幅部分型（width subtyping）**と呼びます。『しくみ』も7章でこの幅部分型を採り、余分なプロパティを許します。

Rigorも`HashShape`を**適合**にします。静的に書くレコードを健全性のために扱うのではなく、相手は**Rubyのオプションハッシュ**で、狙いは**誤検知を出さない**ことです。

- Rubyでは「大きなオプションハッシュを作って、各メソッドが必要なキーだけ拾う」のが**定石**です。
- 余分なキーがあるたびに怒っていたら、**ちゃんと動いているコードが真っ赤**になります。

つまりRigorのHashShapeは、期待する側から見ると「*少なくとも*これらのキーがあればよい」（open）という設計です。余分は気にしません。「必要なキーが*無い*」ときだけ問題にします。これが「動くコードを脅かさない」の、構造的な型での現れ方です。

![図6-1　openなHashShape（余分は許し、不足だけ咎める）](../figures/svg/little-6-1.svg)
> ▼ 図6-1　openな`HashShape`（余分は許し、不足だけ咎める）

三つの視点（パースペクティブ）で整理します。

- **① 型理論**：レコードの**幅部分型**では、キーが*多い*方が部分型です（『しくみ』 7章も同じ幅部分型）。
  - 逆に見えるかもしれませんが、「`{name:}`が欲しい所には`{name:, admin:}`を**渡せる**（`name`はちゃんとある）。逆は渡せない」という関係です。
  - キーが多い方が、より多くの要求に通るので部分型になります。「部分型」は次のPart 7で『箱に入るか』として扱います。
- **② Rubyだと**：optionsハッシュに余分なキーは日常です。完全一致を強いると現実に合いません。
- **③ Rigorだと**：期待はopen（「少なくとも」）です。余分は許し、不足だけ咎めることで誤検知を避けます。

> [!NOTE]
> **`HashShape`はRigorの発明ではない**
>
> 「キーと値の型を覚えた構造的なハッシュ型」はRigorの発明ではありません。同じ問題に複数の型チェッカー（Hackの`shape`、PHPStan/Psalmの`array{...}`）がぶつかり、みな「余分は許す（open）」を選んできました。素朴なjoinでは値の型が`String | Integer`のように混ざってキーごとの情報が失われるからです。chibirigorの`HashShape`はその系譜の最小実装です（各ツールの構文と来歴は付録[a5-3](../appendix/a5-other-languages.md)へ）。

> [!NOTE]
> 「期待するキーが揃っているか」を実際に判定するのは、Part 7の`accepts`の仕事です（型同士が合うかの三値判定）。ここでは「**余分を許す＝openという*方針***」を決めただけで、判定の実装はPart 7でHashShapeを`accepts`に通すときに書きます。

---

## 6-4. この章のまとめ

足したものは、型キャリア`HashShape`／`Tuple`、`type_of`の2 case、読み出し`read_index`です。新しい判定ロジックはほぼ無く（読みは`fetch`の第2引数だけ）、難しさは概念「openという方針」に集約しました。

動かすとこうなります。

```ruby
Chibirigor.annotate("h = {foo: 1, bar: \"a\"}\nh[:foo]\nh[:bar]\nh[:zzz]\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
```

<!-- run: examples/part6.rb -->
```text
1: {foo: 1, bar: "a"}
2: 1
3: "a"
4: nil
```

`h`は各キーの型を覚える`HashShape`です。`h[:foo]`と`h[:bar]`は覚えた型をそれぞれ返し、未知のキー`h[:zzz]`は咎めずに`nil`を返します。「少なくとも」を許す*open*方針によるものです。

この章の三つの視点：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 5章 / TAPL 11章 §11.8） | 値をラベルでまとめる＝レコード型。キーが多い方が部分型 |
| ② Ruby/RBS | symbolキーのoptionsハッシュが多用される。完全一致は現実に合わない |
| ③ Rigor実装の問題 | 期待はopen（少なくとも）。余分を許し不足だけ咎める＝幅部分型を*動的ハッシュ*に適用し誤検知回避 |

**続編に送ったもの**：

- キーワード引数（`def f(name:, **opts)`）の本格対応。本編はハッシュ値としての扱い止まり。
- **`map`と`filter_map`の型の差**：Rigorでは`tuple.map { |x| f(x) }`は位置ごとの型を*保ちます*（`f`の戻り型をそれぞれ適用）。一方`filter_map`は結果サイズが述語次第で変わるため、位置ごとの情報を保てず`Array[T]`に**強制的に拡大（widen）**します。「位置を変えない操作だけがTupleの精度を保てる」という型理論の自然な帰結です。
- レコード部分型の*深さ*（値の型まで再帰的に比べる）、read-onlyなどRBS recordの細部。
- `Struct`/`Data.define`から起こす型（実Rigorの`DataClass`/`DataInstance`）。

## 演習

1. ネストしたハッシュ`{ a: { b: 1 } }`の型は何になるか、`annotate`で確かめよ。
2. `a = [1, "x"]\na[99]`が`nil`になることを確かめ、なぜエラーにしないのかを説明せよ。
3. 文字列キー`{ "a" => 1 }`は今どう扱われるか（symbolキーのみ対応）。対応を広げるとどんな注意が要るか。

---

**次章予告（Part 7）**：いよいよ「型同士が*合う*か」を判定する`accepts`を作ります。`:yes`/`:no`/`:maybe`の三値で、ここで決めた「open」方針も実際に効いてきます。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part6/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part6/lib)
