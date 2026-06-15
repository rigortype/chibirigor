---
title: "Part 6 ― ハッシュと配列の型"
description: "ハッシュ・配列のリテラルに構造的な型（`HashShape`／`Tuple`）をつけ、部分一致を許す設計にする。"
sidebar:
  order: 7
---

# The Little chibirigor Part 6 ― ハッシュと配列の型

前章（Part 5）では Union を場合分けで絞り込み、`if` の枝ごとに型を細くしました。今度は値の
*中身* に目を向けます。**ハッシュ・配列のリテラルに構造的な型をつける（`HashShape`／`Tuple`）。**
そして、そこから値を読み出す型を求めます。Ruby のコードは「symbol キーのハッシュ」だらけ
なので、ここをうまく扱えると一気に実用的になります。

> 『しくみ』5 章「オブジェクト型」（TAPL 11 章 §11.8「レコード」・§11.7「組」）に対応します。あの本は同じものを
> `{ tag: "Object", props }` という型で表しました。私たちもほぼ同じことを Ruby でやります ―
> 最後に一つ、`HashShape` を **open**（余分なキーを許す）にする、という判断もします。

---

## 6-1. リテラルから型を起こす ― HashShape と Tuple

`{ foo: 1, bar: "a" }` の型は何でしょう。「`Hash`」では大ざっぱすぎます。**どのキーに何の型が
入っているか**まで覚えたい。それが `HashShape`：

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

`type_of` に 2 つ case を足すだけ。Prism ではハッシュは `HashNode`（各ペアが `AssocNode`、
symbol キーは `SymbolNode`）、配列は `ArrayNode`：

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

- **① 型理論**：複数の値をラベルでまとめた型＝レコード型（『しくみ』 5 章）。
- **② Ruby だと**：symbol キーのハッシュが至る所に。配列もタプル的に使う（`[name, age]`）。
- **③ Rigor だと**：`Hash` で丸めず、キーごと・位置ごとの型を覚える（Part 1 の「細かく覚える」
  の延長）。

---

## 6-2. 読み出す ― `h[:foo]` と `a[0]`

型に「どのキーが何の型か」が入っているので、読み出しは素直です。`h[:foo]` は Prism では
`[]` というメソッド送信（`h.[](:foo)`）。引数が**リテラルの symbol/整数**なら、型から引けます：

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

`h[:zzz]` で **エラーを出さない**のがポイントです。理由は単純で、**実際の Ruby が
`{foo: 1}[:zzz]` で `nil` を返すから**。存在しないキーの読みは「バグ」ではなく「nil が返る」が
*正しい*挙動。型もそれに合わせて `nil` を返します。決めつけません。

---

## 6-3. open か closed か ― 余分なキーを許す

ここが Part 6 の山です。こういう Ruby を考えます：

```ruby
def greet(user)        # user は { name: ... } を期待しているとする
  "Hello, #{user[:name]}"
end

greet({ name: "Alice", admin: true })   # ★ name 以外に admin も入っている
```

`greet` が欲しいのは `name` だけ。でも渡されたハッシュには `admin` も入っています。これ、
**適合とすべき？ 不適合とすべき？**

型の*等価*なら「プロパティが*完全一致*していないとダメ」ですが、**部分型**なら話は別です ―
「`{name:}` が欲しい所に `{name:, admin:}` を渡せる」のは健全で、これを**幅部分型（width subtyping）**と
呼びます（『しくみ』も 7 章でこの幅部分型を採り、余分なプロパティを許します）。

Rigor も `HashShape` を**適合**にします。違うのは*動機と適用範囲*です。静的に書くレコードを
健全性のために扱うのではなく、相手は **Ruby のオプションハッシュ**、狙いは **誤検知を出さない**こと：

- Ruby では「大きなオプションハッシュを作って、各メソッドが必要なキーだけ拾う」のが**定石**。
- 余分なキーがあるたびに怒っていたら、**ちゃんと動いているコードが真っ赤**になる。

つまり Rigor の HashShape は、期待する側から見ると **「*少なくとも* これらのキーがあればよい」**
（open）。余分は気にしない。**「必要なキーが*無い*」ときだけ問題にする。** これが
「動くコードを脅かさない」の、構造的な型での現れ方です。

![図 6-1　open な HashShape ― 余分は許し、不足だけ咎める](../figures/svg/little-6-1.svg)
> ▼ 図 6-1　open な `HashShape` ― 余分は許し、不足だけ咎める

- **① 型理論**：レコードの**幅部分型** ― キーが*多い*方が部分型（『しくみ』 7 章も同じ幅部分型）。
  逆に見える？ 一言で言うと「`{name:}` が欲しい所には `{name:, admin:}` を**渡せる**（`name` は
  ちゃんとある）。逆は渡せない」 ― *キーが多い方が、より多くの要求に通る＝部分型*、とだけ掴めば
  ここは進めます（「部分型」は次の Part 7 で『箱に入るか』として正面から扱います）。
- **② Ruby だと**：options ハッシュに余分なキーは日常。完全一致を強いると現実に合わない。
- **③ Rigor だと**：期待は open（「少なくとも」）。余分は許し、不足だけ咎める ＝ 誤検知を避ける。

> **コラム：`HashShape` は Rigor の発明ではない**
>
> 「キーと値の型を覚えた構造的なハッシュ型」は Rigor の発明ではありません。同じ問題に複数の
> 型チェッカー（Hack の `shape`、PHPStan/Psalm の `array{...}`）がぶつかり、みな「余分は許す
> （open）」を選んできました ― 素朴な join では値の型が `String | Integer` のように混ざって
> キーごとの情報が失われるからです。chibirigor の `HashShape` はその系譜の最小実装です。
> （各ツールの構文と来歴は付録 [a5-3](../appendix/a5-other-languages.md) へ。）

> 「期待するキーが揃っているか」を実際に判定するのは、Part 7 の `accepts` の仕事です（型同士が
> 合うかの三値判定）。ここでは「**余分を許す＝open という*方針***」を決めただけ。判定の実装は
> Part 7 で HashShape を `accepts` に通すときに書きます。

---

## 6-4. この章のまとめ

足したもの：型キャリア `HashShape`／`Tuple`、`type_of` の 2 case、読み出し `read_index`。
新しい判定ロジックはほぼ無く（読みは `fetch` の第 2 引数だけ）、難しさは概念 ―
「open という方針」― に集約しました。

動かすとこうなります：

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

`h` は各キーの型を覚える `HashShape`。`h[:foo]`・`h[:bar]` は覚えた型をそれぞれ返し、未知の
キー `h[:zzz]` は咎めずに `nil` を返します（「少なくとも」を許す *open* 方針）。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 5 章 / TAPL 11 章 §11.8） | 値をラベルでまとめる＝レコード型。キーが多い方が部分型 |
| ② Ruby/RBS | symbol キーの options ハッシュが多用される。完全一致は現実に合わない |
| ③ Rigor 実装の問題 | 期待は open（少なくとも）。余分を許し不足だけ咎める＝幅部分型を*動的ハッシュ*に適用し誤検知回避 |

**続編に送ったもの**：

- キーワード引数（`def f(name:, **opts)`）の本格対応。本編はハッシュ値としての扱い止まり。
- **`map` vs `filter_map` の型の差**：Rigor では `tuple.map { |x| f(x) }` は
  位置ごとの型を*保ちます*（`f` の戻り型をそれぞれ適用）。一方 `filter_map` は
  結果サイズが述語次第で変わるため、位置ごとの情報を保てず `Array[T]` に**強制的に拡大（widen）**します。
  「位置を変えない操作だけが Tuple の精度を保てる」という型理論の自然な帰結です。
- レコード部分型の*深さ*（値の型まで再帰的に比べる）・read-only など RBS record の細部。
- `Struct`/`Data.define` から起こす型（実 Rigor の `DataClass`/`DataInstance`）。

## 演習

1. ネストしたハッシュ `{ a: { b: 1 } }` の型は何になるか、`annotate` で確かめよ。
2. `a = [1, "x"]\na[99]` が `nil` になることを確かめ、なぜエラーにしないのかを説明せよ。
3. 文字列キー `{ "a" => 1 }` は今どう扱われるか（symbol キーのみ対応）。対応を広げると
   どんな注意が要るか。

---

**次章予告（Part 7）**：いよいよ「型同士が*合う*か」を判定する `accepts` を作ります。
`:yes`/`:no`/`:maybe` の三値で、ここで決めた「open」方針も実際に効いてきます。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part6/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part6/lib)
