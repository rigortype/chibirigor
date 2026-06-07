# The Little chibirigor Part 5 ― ハッシュと配列の型

この章のゴール：**ハッシュ・配列のリテラルに構造的な型をつける（`HashShape`／`Tuple`）。**
そして、そこから値を読み出す型を求めます。Ruby のコードは「symbol キーのハッシュ」だらけ
なので、ここをうまく扱えると一気に実用的になります。

> 『しくみ』5 章「オブジェクト型」（TAPL 11 章 §11.8「レコード」・§11.7「組」）に対応します。あの本は同じものを
> `{ tag: "Object", props }` という型で表しました。私たちもほぼ同じことを Ruby でやります ―
> ただし最後に一つ、**『しくみ』とは正反対の判断**をします。

---

## 5-1. リテラルから型を起こす ― HashShape と Tuple

`{ foo: 1, bar: "a" }` の型は何でしょう。「`Hash`」では大ざっぱすぎます。**どのキーに何の型が
入っているか**まで覚えたい。それが `HashShape`：

```ruby
HashShape = Data.define(:fields) do   # fields: { foo: Const[1], bar: Const["a"] }
  def to_s = "{" + fields.map { |k, v| "#{k}: #{v}" }.join(", ") + "}"
end

Tuple = Data.define(:elements) do     # 配列を「位置ごとの型」で覚える
  def to_s = "[" + elements.map(&:to_s).join(", ") + "]"
end
```

`type_of` に 2 つ case を足すだけ。Prism ではハッシュは `HashNode`（各ペアが `AssocNode`、
symbol キーは `SymbolNode`）、配列は `ArrayNode`：

```ruby
when Prism::HashNode
  fields = node.elements.to_h { |a| [a.key.unescaped.to_sym, type_of(a.value, scope, diag)] }
  HashShape[fields]
when Prism::ArrayNode
  Tuple[node.elements.map { |el| type_of(el, scope, diag) }]
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

## 5-2. 読み出す ― `h[:foo]` と `a[0]`

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

## 5-3. open か closed か ― 『しくみ』と逆を行く

ここが Part 5 の山です。こういう Ruby を考えます：

```ruby
def greet(user)        # user は { name: ... } を期待しているとする
  "Hello, #{user[:name]}"
end

greet({ name: "Alice", admin: true })   # ★ name 以外に admin も入っている
```

`greet` が欲しいのは `name` だけ。でも渡されたハッシュには `admin` も入っています。これ、
**OK にすべき？ NG にすべき？**

『しくみ』 5 章は **NG** にしました。「プロパティが*完全一致*していないとダメ」＝余分な `admin` が
あると拒否する。理由は健全性（きっちり管理したい）。

Rigor は逆に **OK** にします。理由は **Ruby の現実**です：

- Ruby では「大きなオプションハッシュを作って、各メソッドが必要なキーだけ拾う」のが**定石**。
- 余分なキーがあるたびに怒っていたら、**ちゃんと動いているコードが真っ赤**になる。

つまり Rigor の HashShape は、期待する側から見ると **「*少なくとも* これらのキーがあればよい」**
（open）。余分は気にしない。**「必要なキーが*無い*」ときだけ問題にする。** これが
「動くコードを脅かさない」の、構造的な型での現れ方です。

- **① 型理論**：レコードの部分型 ― キーが*多い*方が部分型（『しくみ』 5 章は完全一致でここを締めた）。
- **② Ruby だと**：options ハッシュに余分なキーは日常。完全一致を強いると現実に合わない。
- **③ Rigor だと**：期待は open（「少なくとも」）。余分は許し、不足だけ咎める ＝ 誤検知を避ける。

> 「期待するキーが揃っているか」を実際に判定するのは、Part 6 の `accepts` の仕事です（型同士が
> 合うかの三値判定）。ここでは「**余分を許す＝open という*方針***」を決めただけ。判定の実装は
> Part 6 で HashShape を `accepts` に通すときに書きます。

---

## 5-4. この章のまとめ

足したもの：型キャリア `HashShape`／`Tuple`、`type_of` の 2 case、読み出し `read_index`。
新しい判定ロジックはほぼ無く（読みは `fetch` の第 2 引数だけ）、難しさは概念 ―
「open という方針」― に集約しました。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 5 章 / TAPL 11 章 §11.8） | 値をラベルでまとめる＝レコード型。キーが多い方が部分型 |
| ② Ruby/RBS | symbol キーの options ハッシュが氾濫。完全一致は現実に合わない |
| ③ Rigor 実装の問題 | 期待は open（少なくとも）。余分を許し不足だけ咎める＝**『しくみ』と逆向き**で誤検知回避 |

**続編に送ったもの**：

- キーワード引数（`def f(name:, **opts)`）の本格対応。本編はハッシュ値としての扱い止まり。
- レコード部分型の*深さ*（値の型まで再帰的に比べる）・read-only など RBS record の細部。
- `Struct`/`Data.define` から起こす型（実 Rigor の `DataClass`/`DataInstance`）。

## 演習

1. ネストしたハッシュ `{ a: { b: 1 } }` の型は何になるか、`annotate` で確かめよ。
2. `a = [1, "x"]\na[99]` が `nil` になることを確かめ、なぜエラーにしないのかを説明せよ。
3. 文字列キー `{ "a" => 1 }` は今どう扱われるか（symbol キーのみ対応）。対応を広げると
   どんな注意が要るか。

---

**次章予告（Part 6）**：いよいよ「型同士が*合う*か」を判定する `accepts` を作ります。
`:yes`/`:no`/`:maybe` の三値で、ここで決めた「open」方針も実際に効いてきます。

