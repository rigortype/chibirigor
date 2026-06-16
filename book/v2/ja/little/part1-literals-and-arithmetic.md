---
title: Part 1　リテラルと算術
description: 値に型をつける最小の仕組みを作り、`check` と `annotate` を動かす。`Const` と `type_of` を導入する。
sidebar:
  order: 2
---

# The Little chibirigor Part 1　リテラルと算術

この章のゴールは、値に型をつける一番小さな仕組みを作り、`check`と`annotate`を実Rubyで
動かすことです。
新しく出すのは`Const`という型ひとつと、`type_of`という関数ひとつだけです。

> [!NOTE]
> この章では『型システムのしくみ』第2章を何度か参照します。
> あの本はTypeScriptのミニ言語で同じことをやっています。
> 同じ考え方がRubyだとどう変わるかを、並べて見ていきます。

---

## 1-1. 型を「データ」として表す

型をむずかしく考える必要はありません。
ここでは「値につけた小さなラベル」だと思ってください。
`1`には「整数」、`"hi"`には「文字列」というラベルが付きます。
型チェックとは、このラベル同士がケンカしていないかを見ることです。

『型システムのしくみ』2章でも、型を`{ tag: "Number" }`のようなただのデータで表していました。
私たちも同じく、型をRubyのオブジェクトで表します。
ここで頭を一つ切り替えてください。**型は`Integer`のような「クラス」そのものではなく、型を表す*データ*です**。
`Const[1]`のように「クラスでは書けない細かい型」も作りたいので、専用のデータで表します。
この章で使うのは、次の3つだけです。

```ruby
module Chibirigor
  # 「この値そのもの」を表す型。例: Const[1], Const["hi"]
  Const = Data.define(:value) do
    def to_s = value.inspect
  end

  # 名前付きクラスを表す型。例: Nominal[:Integer]（1-2 の「丸め」で使う）
  Nominal = Data.define(:name) do
    def to_s = name.to_s
  end

  # 「知らない・確かめようがない」を表す型（あとで大活躍する）
  Dynamic = Data.define do
    def to_s = "untyped"
  end
end
```

ここで、三つの視点（この本の恒例の枠組み）で整理してみます。

- **① 型理論**：型は値につけるラベルで、内部ではただのデータです（『しくみ』 2章）。
- **② Rubyだと**：`1`のクラスは`Integer`、`"hi"`のクラスは`String`です。RubyもRBSも「`1`は`Integer`」とまでしか言いません。
- **③ Rigorだと**：Rigorはもう一歩踏み込んで、`1`という値そのものを型にします（`Const[1]`）。「`Integer`」ではなく「`1`」です。なぜそんな細かいことをするのか。これがあとで`case`の枝分けや定数の計算で効いてきます。いまは「Rigorは型を細かく覚える」とだけ覚えておけば十分です。

型がわからないときや確かめようがないときは、`Dynamic`（`untyped`）に逃がします。
「知らない、確かめようがないなら黙っておく」という印です。

> [!NOTE]
> **「分からない」への正反対の2流派**
>
> 「型が分からない」への態度は、言語によって逆向きの2流派に分かれます。
> TypeScriptの`unknown`は「分からないなら、使う前にまず*絞れ*」と迫る慎重派です。
> かたやRubyの`untyped`（やTypeScriptの`any`）は「分からないなら*黙って通す*」という寛容派で、chibirigorの`Dynamic`はこちら側です。
> 同じ「分からない」でも、健全性を取るか、*動くコードを止めない*ことを取るかで流儀が分かれます。
> 本書が寛容派なのは、まさに「動くコードを脅かさない」ためです。
> なお`untyped`が「なんでも入る」トップ型と混同されやすい理由（実は別物）と、各言語での呼び名の対応表は、付録[a1-1](../appendix/a1-special-types.md)にまとめてあります。

---

## 1-2. 式から型を求める`type_of`

型チェッカーの心臓は、「式を受け取って型を返す」関数ひとつです。
『しくみ』ではこれを`typecheck`と呼んでいました。
私たちは`type_of`と呼びます。

コードはPrismでパースします。
`Prism.parse("1").value`を辿ると、`1`は`IntegerNode`、`"hi"`は`StringNode`、というふうに種類ごとのノードになっています。
種類で場合分けするだけです。

```ruby
module Chibirigor
  module_function

  def type_of(node, diagnostics)
    case node
    when Prism::IntegerNode then Const[node.value]
    when Prism::FloatNode   then Const[node.value]
    when Prism::StringNode  then Const[node.unescaped]
    when Prism::TrueNode    then Const[true]
    when Prism::FalseNode   then Const[false]
    when Prism::CallNode    then type_of_call(node, diagnostics)
    else
      Dynamic.new   # 知らないノードは「脅かさない」── だまって untyped を返す
    end
  end
end
```

『しくみ』の`typecheck`が`switch (t.tag)`だったのと、ほとんど同じ形です。
違いは最後の行です。
『しくみ』は知らない構文に出会いません（対象がきっちりしたミニ言語だから）。
でも私たちが相手にするのは*本物のRuby*です。
知らないものは必ず出てきます。
そのときエラーにせず`Dynamic`（untyped）を返します。
これがRigorの入口の姿勢です。

> [!IMPORTANT]
> **覚えておく原理**：`type_of`は*失敗しません*。
> 型がわからなければ`Dynamic`を返すだけです。
> だから「型がつかないコード」を理由に怒ることが、そもそも起きません。

### `1 + 2`はどうなる？

ここでRubyならではの事実が一つあります。
`1 + 2`の`+`も「メソッド送信」です。
Prismでは`1`に`+`というメッセージを送る`CallNode`になります（`1.+(2)`と同じ）。

いまは算術だけを、ごく素朴に書きます。

```ruby
module Chibirigor
  module_function

  def type_of_call(node, diagnostics)
    recv = type_of(node.receiver, diagnostics)
    args = node.arguments&.arguments || []

    if node.name == :+ && integerish?(recv)
      arg = type_of(args.first, diagnostics)
      unless integerish?(arg)
        diagnostics << diagnostic(node, "can't add #{arg} to an integer")
        return Dynamic.new
      end
      # ★ ここがポイント：Const[3] とは計算せず、Integer に「丸める」
      return Nominal[:Integer]
    end

    Dynamic.new   # それ以外のメソッドはまだ知らない → 脅かさない
  end

  def integerish?(t)
    (t.is_a?(Const) && t.value.is_a?(Integer)) || t == Nominal[:Integer]
  end

  # 診断は「どの行の・何が問題か」を持つ小さなハッシュ
  def diagnostic(node, message)
    { line: node.location.start_line, message: message }
  end
end
```

（`Nominal[:Integer]`は1-1で定義した「整数クラス」を表すラベルです。）

ここで、三つの視点の ③（Rigorが困った所）が自然に顔を出します。

- `type_of(1)`は`Const[1]`、`type_of(2)`は`Const[2]`です。
- では`type_of(1 + 2)`は`Const[3]`にすべきでしょうか。したい気もしますが、それには*足し算を実際に計算*しないといけません。`x + 2`ならもう値はわかりません。
- そこで結果は`Integer`に丸めます。「値そのもの」を覚えるのは便利ですが、どこかで手放して大ざっぱな型に戻す必要があります。いまは「足し算の結果は`Integer`」とだけ覚えれば十分です（この"いつ丸めるか"を実Rigorがどう体系立てるかは、後編で扱います）。

## 1-3. 矛盾を見つける`check`（でも止まらない）

`type_of`ができたので、矛盾の報告である`check`を作ります。
やることは「トップレベルの文をひとつずつ`type_of`にかけ、その途中で見つかった文句（diagnostics）を集める」だけです。

```ruby
module Chibirigor
  module_function

  def check(source)
    program = Prism.parse(source).value
    diagnostics = []
    program.statements.body.each { |stmt| type_of(stmt, diagnostics) }
    diagnostics
  end
end
```

`check`の戻り値は`{line:, message:}`の配列です（どの行で何が問題か）。
動かしてみます。

```ruby
Chibirigor.check("1 + 2")       # => []                                          （文句なし）
Chibirigor.check('1 + "x"')     # => [{ line: 1, message: "can't add \"x\" to an integer" }]
Chibirigor.check("foo.bar")     # => []   ← 知らないメソッドは黙って通す
```

『しくみ』の`typecheck`は矛盾を見つけると`throw`してそこで止まりました。
私たちは違います。
文句を配列に貯めて、最後まで読み進めます。
一つ目のエラーで止まらないし、わからない所（`foo.bar`）はそっと通します。
これも「動くコードを脅かさない」の一部です。

- **① 型理論**：型チェックは、ラベルの矛盾検出です（『しくみ』 2章）。
- **② Rubyだと**：`1 + "x"`は実行すれば`TypeError`になります。`foo.bar`は`foo`次第で動くかもしれません。
- **③ Rigorだと**：確実に矛盾する所だけ報告し、わからない所は黙ります（*止まらず、脅かさず*）。

---

## 1-4. 求めた型を見せる`annotate`

ここまで来ると、おまけがほぼタダで手に入ります。
`type_of`は型を*作っている*のだから、それを出力するだけで「推論した型を見せる」`annotate`になります。

`check`が`{line:, message:}`の配列を返すのに合わせ、`annotate`は各文の`{line:, type:}`の
配列を返します（行番号と、推論した型）。

```ruby
module Chibirigor
  module_function

  def annotate(source)
    program = Prism.parse(source).value
    program.statements.body.map do |stmt|
      type = type_of(stmt, [])     # 文句は今は捨てる
      { line: stmt.location.start_line, type: type }
    end
  end
end
```

表示してみます（型は`to_s`で`1`／`Integer`／`untyped`のように出ます）。

```ruby
Chibirigor.annotate(<<~RUBY).each { |a| puts "#{a[:line]}: #{a[:type]}" }
  42
  "hello"
  1 + 2
  foo.bar
RUBY
```

<!-- run: examples/part1.rb -->
```text
1: 42
2: "hello"
3: Integer
4: untyped
```

`42`は`Const[42]`なので`42`と細かく出ます。
`1 + 2`は丸めて`Integer`になります。
`foo.bar`はわからないので`untyped`です。
「`untyped`がどこに出るか」は「Rigorが型を見失った場所」であり、これが見えること自体が`annotate`の値打ちです（実Rigorの`sig-gen`の発想の芽）。

> [!NOTE]
> chibirigorの`annotate`は、推論した内部の型をそのまま見せる最小版です。
> 実Rigorの`annotate`は、「内部では精密に知り、RBSの境界では粗く丸める」二重構造を持ちます（だから推論はもっと知っていても、外向けのシグネチャは粗くなる）。
> その仕組みは付録[a3-2](../appendix/a3-tooling.md)で扱います。

> [!NOTE]
> ここまでのコードは`1 + 2`を`Integer`に*丸める*だけです。
> でも`1`も`2`も既知の値なので、実は実際に足して`3`に畳める余地があります。
> この「畳めれば畳む」定数畳み込みはPart 2の発展ノートで扱います（手元の`exe/chibirigor`で`1 + 2`が`3`と出るのはそのため）。

---

## 1-4b. 診断を読みやすくする（位置とキャレット）

診断は`行`だけでなく`列`や`長さ`も持たせると、どこが問題かを*指せ*ます。
`diagnostic`をひとさじ拡張します（Prismのノードは位置情報`location`を持っています）。

```ruby
def diagnostic(node, message)
  location = node.location
  { line: location.start_line, column: location.start_column, length: location.length, message: message }
end
```

するとCLI（`exe/chibirigor`）は、該当行の下にキャレット`^^^`を引けます。

```console
$ ruby exe/chibirigor check bad.rb
bad.rb:2:1: expected Integer but got "bad"
  1 + "bad"
  ^^^^^^^^^
```

たったこれだけで「どの行の、どの語が」問題かが一目で分かります。
実Rigorの診断はここをさらに作り込み、SARIFやGitHubの注釈に変換されます（ADR-51）。

---

## 1-5. この章のまとめ

作ったものは、型`Const`／`Dynamic`／`Nominal`、関数`type_of`／`check`／`annotate`です。
全部で50行ほどです。
『しくみ』 2章の`typecheck`（約40行）に、「丸め」「止まらない」「untypedに逃がす」が少し足された規模感です。

この章の三つの視点：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 2 章 / TAPL 8 章） | 型は値につけるラベル＝ただのデータ。式から型を求める関数が心臓 |
| ② Ruby/RBS | `1` は `Integer`、`+` すらメソッド送信。RBS は「`1` は `Integer`」止まり |
| ③ Rigor 実装の問題 | 値そのものを型にする（`Const[1]`）と細かいが、*いつ `Integer` に丸めるか*が問題になる（続編で詳しく扱う） |

## 演習

1. `annotate("3.14")`と`annotate("true")`の結果を確かめ、`Const#to_s`がどう効いているか
   説明せよ。
2. いまの`type_of_call`は`+`だけを丸めます。`1 - 2`も`Integer`になるよう拡張せよ
   （ヒント：条件を`:+`/`:-`の両方に）。
3. `1 + 2 + 3`を`check`すると診断は出ません。なぜ矛盾なく通るのか、`type_of`の再帰で説明せよ。

---

**次章予告**：`1 + 2`で素通りした「`+`もメソッド送信」を、きちんと扱います。
Rubyは何でもメソッド送信なので、「どのクラスのどのメソッドが何を返すか」を引く表（ディスパッチ）が要ります。
そして「知らないメソッドは`Dynamic`に逃がす」の続きと、§1-4で予告した定数畳み込み（畳めれば畳む）を発展ノートで扱います。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part1/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part1/lib)
