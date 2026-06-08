---
title: "Part 1 ― リテラルと算術"
description: "値に型をつける最小の仕組みを作り、`check` と `annotate` を動かす。`Const` と `type_of` の導入。"
sidebar:
  order: 1
---

# The Little chibirigor Part 1 ― リテラルと算術

この章のゴール：**値に型をつける一番小さな仕組みを作り、`check` と `annotate` を実 Ruby で
動かす。** 新しく出すのは `Const` という型ひとつと、`type_of` という関数ひとつだけ。

> この章で何度か『型システムのしくみ』第2章を参照します。あの本は TypeScript のミニ言語で
> 同じことをやっています。同じ考え方が Ruby だとどう変わるかを並べて見ていきます。

---

## 1-1. 型を「データ」として表す

**型って何でしょう。** むずかしく考えず、ここでは **「値につけた小さなラベル」** だと思って
ください。`1` には「整数」、`"hi"` には「文字列」というラベルが付く。型チェックとは、このラベル
同士がケンカしていないかを見ることです。

『型システムのしくみ』2 章でも、型を `{ tag: "Number" }` のような**ただのデータ**で表して
いました。私たちも同じく、型を Ruby のオブジェクトで表します。この章で使うのは次の 3 つだけ：

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

ここで **三題噺**（この本の恒例の枠組み）を一つ：

- **① 型理論**：型は値につけるラベル。内部ではただのデータ（『しくみ』 2 章）。
- **② Ruby だと**：`1` のクラスは `Integer`、`"hi"` のクラスは `String`。Ruby も RBS も
  「`1` は `Integer`」とまでしか言いません。
- **③ Rigor だと**：Rigor はもう一歩踏み込んで、**`1` という値そのもの**を型にします
  （`Const[1]`）。「`Integer`」ではなく「`1`」。なぜそんな細かいことを？ ── これがあとで
  `case` の枝分けや定数の計算で効いてきます。いまは「Rigor は型を細かく覚える」とだけ。

---

## 1-2. `type_of` ― 式から型を求める

型チェッカーの心臓は、**「式を受け取って型を返す」関数ひとつ**です。『しくみ』ではこれを `typecheck`
と呼んでいました。私たちは `type_of` と呼びます。

コードは Prism でパースします。`Prism.parse("1").value` を辿ると、`1` は `IntegerNode`、
`"hi"` は `StringNode`、というふうに種類ごとのノードになっています。種類で場合分けするだけ：

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

『しくみ』の `typecheck` が `switch (t.tag)` だったのと、ほとんど同じ形ですね。違いは最後の行。
**『しくみ』は知らない構文に出会いません**（対象がきっちりしたミニ言語だから）。でも私たちが相手に
するのは*本物の Ruby* です。知らないものは必ず出てきます。そのとき **エラーにせず
`Dynamic`（untyped）を返す** ── これが Rigor の入口の姿勢です。

> **覚えておく原理**：`type_of` は*失敗しません*。型がわからなければ `Dynamic` を返すだけ。
> だから「型がつかないコード」を理由に怒ることが、そもそも起きません。

### `1 + 2` はどうなる？

ここで Ruby ならではの事実が一つ。**`1 + 2` の `+` も「メソッド送信」です。** Prism では
`1` に `+` というメッセージを送る `CallNode` になります（`1.+(2)` と同じ）。

いまは算術だけ、ごく素朴に書きます：

```ruby
module Chibirigor
  module_function

  def type_of_call(node, diagnostics)
    recv = type_of(node.receiver, diagnostics)
    args = node.arguments&.arguments || []

    if node.name == :+ && integerish?(recv)
      arg = type_of(args.first, diagnostics)
      unless integerish?(arg)
        diagnostics << diagnostic(node, "整数に #{arg} は足せません")
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

（`Nominal[:Integer]` は 1-1 で定義した「整数クラス」を表すラベルです。）

ここで **三題噺の ③（Rigor が困った所）** が自然に顔を出します：

- `type_of(1)` は `Const[1]`、`type_of(2)` は `Const[2]`。
- では `type_of(1 + 2)` は `Const[3]` にすべき？ ── したい気もしますが、それには*足し算を
  実際に計算*しないといけません。`x + 2` ならもう値はわかりません。
- そこで **結果は `Integer` に丸めます**。「値そのもの」を覚えるのは便利だけれど、
  **どこかで手放して大ざっぱな型に戻す**必要がある。**この“いつ丸めるか”が、実 Rigor では
  『正規化』と『推論予算』という大きなテーマになります。** いまは「足し算の結果は
  `Integer`」とだけ覚えれば十分。深掘りは続編で（畳める所は*畳む*話は、すぐ下の「発展」で）。

> ちなみに、ここは型ツールの個性が出る所です。Ruby 同梱の TypeProf は `1` を**最初から
> `Integer` に潰します**（全体を型レベルで実行するので、`Const[1]` のような細かい型を持つと
> 状態が爆発する）。chibirigor（と Rigor）は局所的に型を組み立てるので `Const[1]` を**保てて**、
> 後で `case` の枝分けや定数の計算に効かせられます。「精度をどこで手放すか」は、型ツールの
> 根本的な選択なのです。

## 1-3. `check` ― 矛盾を見つける（でも止まらない）

`type_of` ができたので、**矛盾の報告**＝ `check` を作ります。やることは「トップレベルの
文をひとつずつ `type_of` にかけ、その途中で見つかった文句（diagnostics）を集める」だけ：

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

`check` の戻り値は **`{line:, message:}` の配列**（どの行で何が問題か）。動かしてみます：

```ruby
Chibirigor.check("1 + 2")       # => []                                          （文句なし）
Chibirigor.check('1 + "x"')     # => [{ line: 1, message: "整数に \"x\" は足せません" }]
Chibirigor.check("foo.bar")     # => []   ← 知らないメソッドは黙って通す
```

『しくみ』の `typecheck` は矛盾を見つけると **`throw` してそこで止まりました**。私たちは違います：
**文句を配列に貯めて、最後まで読み進めます。** 一つ目のエラーで止まらないし、わからない所
（`foo.bar`）は*そっと通す*。これも「動くコードを脅かさない」の一部です。

- **① 型理論**：型チェック＝ラベルの矛盾検出（『しくみ』 2 章）。
- **② Ruby だと**：`1 + "x"` は実行すれば `TypeError`。`foo.bar` は `foo` 次第で動くかも。
- **③ Rigor だと**：確実に矛盾する所だけ報告し、わからない所は黙る。*止まらず・脅かさず*。

---

## 1-4. `annotate` ― 求めた型を見せる

ここまで来ると、おまけがほぼタダで手に入ります。`type_of` は型を*作っている*のだから、
それを**出力するだけ**で「推論した型を見せる」`annotate` になります：

`check` が `{line:, message:}` の配列を返すのに合わせ、`annotate` は各文の `{line:, type:}` の
配列を返します（行番号と、推論した型）：

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

表示してみます（型は `to_s` で `1`／`Integer`／`untyped` のように出ます）：

```ruby
Chibirigor.annotate(<<~RUBY).each { |a| puts "#{a[:line]}: #{a[:type]}" }
  42
  "hello"
  1 + 2
  foo.bar
RUBY
# 1: 42
# 2: "hello"
# 3: Integer
# 4: untyped
```

`42` は `Const[42]` なので `42` と細かく出る。`1 + 2` は丸めたので `Integer`。`foo.bar` は
わからないので `untyped`。**「`untyped` がどこに出るか」＝「Rigor が型を見失った場所」** で、
これが見えること自体が `annotate` の値打ちです（実 Rigor の `sig-gen` の発想の芽）。

> ここまでのコードは `1 + 2` を `Integer` に*丸める*だけ。でも最終的な道具は、畳める所は
> **畳んで `3`** と出します ― それを次の「発展：定数畳み込み」で見ます（手元の `exe/chibirigor`
> で `1 + 2` が `3` と出るのはそのためです）。

---

## 1-4b. 診断を読みやすく ― 位置とキャレット

診断は `行` だけでなく **`列`・`長さ`** も持たせると、どこが問題かを*指せ*ます。`diagnostic` を
ひとさじ拡張します（Prism のノードは位置情報 `location` を持っています）：

```ruby
def diagnostic(node, message)
  location = node.location
  { line: location.start_line, column: location.start_column, length: location.length, message: message }
end
```

すると CLI（`exe/chibirigor`）は、該当行の下に **キャレット `^^^`** を引けます：

```console
$ ruby exe/chibirigor check bad.rb
bad.rb:2:1: Integer が必要ですが "bad" が渡されました
  1 + "bad"
  ^^^^^^^^^
```

たったこれだけで「どの行の・どの語が」一目で分かります。実 Rigor の診断はここをさらに作り込み、
SARIF や GitHub の注釈に変換されます（ADR-51）。なお `baseline`（前編 Part 9）の照合は**行と
メッセージだけ**で行い、列は含めません ― 同じ行を編集して桁がズレても baseline が外れないように。

---

## 1-4c. 発展：定数畳み込み（畳めれば畳む）

ここまで `1 + 2` は `Integer` に**丸めて**きました。でも `1` も `2` も*既知の値*です ― なら
**実際に足して `Const[3]` に畳める**はず。「値そのもの」をもう一段保てれば、`annotate` の精度が
上がります（実 Rigor の `Constant<3>` リテラル精度の縮図）。

やることは「両オペランドが既知値の `Const` なら計算する、ただし*大きくなりすぎたら*丸める」だけ。
`+` の分岐に*ひとさじ*足すと、こうなります（丸める前に一度だけ畳みを試す）：

```ruby
# 両方が既知値の Const なら計算して畳む。予算（大きさ）を超えたら丸めに任せる。
if recv.is_a?(Const) && arg.is_a?(Const)
  result = recv.value + arg.value
  return Const[result] if result.abs <= 1_000_000   # 予算内 → 畳む
end
return Nominal[:Integer]                              # 畳めない → 丸める
```

これで `annotate` はこう変わります：

```text
1 + 2          # => 3          （畳めた）
1 + 2 + 3      # => 6          （再帰で 1+2→3、3+3→6 と畳み続く）
"a" * 3        # => "aaa"      （文字列も畳める）
100000 * 100   # => Integer    （1,000,000 超 ＝ 予算超過 → 丸める）
1 + x          # => Integer    （x が値不明 ＝ 畳めない → 丸める）
```

ポイントは 2 つ：

- **広げ規則（widening）**：際限なく大きな `Const` を抱えないよう、閾値を超えたら丸める。この
  「いつ畳むのをやめるか」が、§1-2 で予告した**正規化・推論予算**の最小版です。
- **誤検知ゼロ**：畳み込みは*精度を足すだけ*。`Const[3]` も `Integer` の所に通るので、新しい
  診断は一切増えません（`1 + "x"` のように畳めない式は、これまで通り丸めて元の挙動のまま）。

実際の `chibirigor` では、この畳み込みはメソッドの**表**（Part 2 で作る `Dispatch`）側に置いて
あります。だから表を引くどの演算でも効き、`x = 1; 1 + x` のように*変数が既知の `Const` を運んで
いれば*、それも `2` に畳めます。手元の `exe/chibirigor` で `1 + 2` が `3` と出るのはこのためです。

---

## 1-5. この章のまとめ

作ったもの：型 `Const`／`Dynamic`／`Nominal`、関数 `type_of`／`check`／`annotate`。
全部で 50 行ほど。『しくみ』 2 章の `typecheck`（約 40 行）に、「丸め」「止まらない」「untyped に
逃がす」が少し足された規模感です。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 2 章 / TAPL 8 章） | 型は値につけるラベル＝ただのデータ。式から型を求める関数が心臓 |
| ② Ruby/RBS | `1` は `Integer`、`+` すらメソッド送信。RBS は「`1` は `Integer`」止まり |
| ③ Rigor 実装の問題 | 値そのものを型にする（`Const[1]`）と細かいが、*いつ `Integer` に丸めるか*が問題に＝正規化・予算（続編で深掘り） |

## 演習

1. `annotate("3.14")` と `annotate("true")` の結果を確かめ、`Const#to_s` がどう効いているか
   説明せよ。
2. いまの `type_of_call` は `+` だけを丸める。`1 - 2` も `Integer` になるよう拡張せよ
   （ヒント：条件を `:+`/`:-` の両方に）。
3. `1 + 2 + 3` を `check` すると診断は出ない。なぜ矛盾なく通るのか、`type_of` の再帰で説明せよ。

---

**次章予告**：`1 + 2` で素通りした「`+` もメソッド送信」を正面から扱います。Ruby は何でも
メソッド送信なので、**「どのクラスのどのメソッドが何を返すか」を引く表（ディスパッチ）** が
要ります。そして「知らないメソッドは `Dynamic` に逃がす」の続きを作ります。
