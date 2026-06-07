# 【試し書き】chibirigor 本編 Part 1 ― リテラルと算術

> これは `20260607-chibirigor-tutorial-draft.md` のトーンと複雑さ予算を検証するための
> *サンプル章*。本番原稿ではなく、「とても易しく／実装は小さく／三題噺で書けるか」の実証。
> 文体・分量・コード規模がねらい通りか確かめる目的。

この章のゴール：**値に型をつける一番小さな仕組みを作り、`check` と `annotate` を実 Ruby で
動かす。** 新しく出すのは `Const` という型ひとつと、`type_of` という関数ひとつだけ。

> この章で何度か『型システムのしくみ』第2章を参照します。あの本は TypeScript のミニ言語で
> 同じことをやっています。同じ考え方が Ruby だとどう変わるかを並べて見ていきます。

---

## 1-1. 型を「データ」として表す

**型って何でしょう。** むずかしく考えず、ここでは **「値につけた小さなラベル」** だと思って
ください。`1` には「整数」、`"hi"` には「文字列」というラベルが付く。型検査とは、このラベル
同士がケンカしていないかを見ることです。

『型システムのしくみ』2 章でも、型を `{ tag: "Number" }` のような**ただのデータ**で表して
いました。私たちも同じく、型を Ruby のオブジェクトで表します。まずは 1 種類だけ：

```ruby
module ChibiRigor
  # 「この値そのもの」を表す型。例: Const[1], Const["hi"]
  Const = Data.define(:value) do
    def to_s = value.inspect
  end

  # 「知らない・確かめようがない」を表す型（あとで大活躍する）
  Dynamic = Data.define do
    def to_s = "untyped"
  end
end
```

ここで **三題噺**（この本のいつものパターン）を一つ：

- **① 型理論**：型は値につけるラベル。内部ではただのデータ（本書 2 章）。
- **② Ruby だと**：`1` のクラスは `Integer`、`"hi"` のクラスは `String`。Ruby も RBS も
  「`1` は `Integer`」とまでしか言いません。
- **③ Rigor だと**：Rigor はもう一歩踏み込んで、**`1` という値そのもの**を型にします
  （`Const[1]`）。「`Integer`」ではなく「`1`」。なぜそんな細かいことを？ ── これがあとで
  `case` の枝分けや定数の計算で効いてきます。いまは「Rigor は型を細かく覚える」とだけ。

---

## 1-2. `type_of` ― 式から型を求める

型検査器の心臓は、**「式を受け取って型を返す」関数ひとつ**です。本書ではこれを `typecheck`
と呼んでいました。私たちは `type_of` と呼びます。

コードは Prism でパースします。`Prism.parse("1").value` を辿ると、`1` は `IntegerNode`、
`"hi"` は `StringNode`、というふうに種類ごとのノードになっています。種類で場合分けするだけ：

```ruby
module ChibiRigor
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

本書の `typecheck` が `switch (t.tag)` だったのと、ほとんど同じ形ですね。違いは最後の行。
**本書は知らない構文に出会いません**（対象がきっちりしたミニ言語だから）。でも私たちが相手に
するのは*本物の Ruby* です。知らないものは必ず出てきます。そのとき **エラーにせず
`Dynamic`（untyped）を返す** ── これが Rigor の入口の姿勢です。

> **覚えておく原理**：`type_of` は*失敗しません*。型がわからなければ `Dynamic` を返すだけ。
> だから「型がつかないコード」を理由に怒ることが、そもそも起きません。

### `1 + 2` はどうなる？

ここで Ruby ならではの事実が一つ。**`1 + 2` の `+` も「メソッド送信」です。** Prism では
`1` に `+` というメッセージを送る `CallNode` になります（`1.+(2)` と同じ）。

いまは算術だけ、ごく素朴に書きます：

```ruby
module ChibiRigor
  module_function

  def type_of_call(node, diagnostics)
    recv = type_of(node.receiver, diagnostics)
    args = node.arguments&.arguments || []

    if node.name == :+ && integerish?(recv)
      arg = type_of(args.first, diagnostics)
      unless integerish?(arg)
        diagnostics << "整数に #{arg} は足せません"
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
end
```

（`Nominal[:Integer]` は「整数クラス」を表すラベル。`Const` の隣に小さく足しておきます：
`Nominal = Data.define(:name) { def to_s = name.to_s }`。）

ここで **三題噺の ③（Rigor が困った所）** が自然に顔を出します：

- `type_of(1)` は `Const[1]`、`type_of(2)` は `Const[2]`。
- では `type_of(1 + 2)` は `Const[3]` にすべき？ ── したい気もしますが、それには*足し算を
  実際に計算*しないといけません。`x + 2` ならもう値はわかりません。
- そこで **結果は `Integer` に丸めます**。「値そのもの」を覚えるのは便利だけれど、
  **どこかで手放して大ざっぱな型に戻す**必要がある。**この“いつ丸めるか”が、実 Rigor では
  『正規化』と『推論予算』という大きなテーマになります。** いまは「足し算の結果は
  `Integer`」とだけ覚えれば十分。深掘りは続編で。

---

## 1-3. `check` ― 矛盾を見つける（でも止まらない）

`type_of` ができたので、**矛盾の報告**＝ `check` を作ります。やることは「トップレベルの
文をひとつずつ `type_of` にかけ、その途中で見つかった文句（diagnostics）を集める」だけ：

```ruby
module ChibiRigor
  module_function

  def check(source)
    program = Prism.parse(source).value
    diagnostics = []
    program.statements.body.each { |stmt| type_of(stmt, diagnostics) }
    diagnostics
  end
end
```

動かしてみます：

```ruby
ChibiRigor.check("1 + 2")       # => []                     （文句なし）
ChibiRigor.check('1 + "x"')     # => ["整数に \"x\" は足せません"]
ChibiRigor.check("foo.bar")     # => []   ← 知らないメソッドは黙って通す
```

本書の `typecheck` は矛盾を見つけると **`throw` してそこで止まりました**。私たちは違います：
**文句を配列に貯めて、最後まで読み進めます。** 一つ目のエラーで止まらないし、わからない所
（`foo.bar`）は*そっと通す*。これも「動くコードを脅かさない」の一部です。

- **① 型理論**：型検査＝ラベルの矛盾検出（本書 2 章）。
- **② Ruby だと**：`1 + "x"` は実行すれば `TypeError`。`foo.bar` は `foo` 次第で動くかも。
- **③ Rigor だと**：確実に矛盾する所だけ報告し、わからない所は黙る。*止まらず・脅かさず*。

---

## 1-4. `annotate` ― 求めた型を見せる

ここまで来ると、おまけがほぼタダで手に入ります。`type_of` は型を*作っている*のだから、
それを**出力するだけ**で「推論した型を見せる」`annotate` になります：

```ruby
module ChibiRigor
  module_function

  def annotate(source)
    program = Prism.parse(source).value
    program.statements.body.map do |stmt|
      type = type_of(stmt, [])     # 文句は今は捨てる
      line = stmt.location.start_line
      "#{line}: #{type}"
    end
  end
end
```

```ruby
puts ChibiRigor.annotate(<<~RUBY)
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

---

## 1-5. この章のまとめ

作ったもの：型 `Const`／`Dynamic`／`Nominal`、関数 `type_of`／`check`／`annotate`。
全部で 50 行ほど。本書 2 章の `typecheck`（約 40 行）に、「丸め」「止まらない」「untyped に
逃がす」が少し足された規模感です。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（本書 2 章） | 型は値につけるラベル＝ただのデータ。式から型を求める関数が心臓 |
| ② Ruby/RBS | `1` は `Integer`、`+` すらメソッド送信。RBS は「`1` は `Integer`」止まり |
| ③ Rigor 実装の問題 | 値そのものを型にする（`Const[1]`）と細かいが、*いつ `Integer` に丸めるか*が問題に＝正規化・予算（続編で深掘り） |

**次章予告**：`1 + 2` で素通りした「`+` もメソッド送信」を正面から扱います。Ruby は何でも
メソッド送信なので、**「どのクラスのどのメソッドが何を返すか」を引く表（ディスパッチ）** が
要ります。そして「知らないメソッドは `Dynamic` に逃がす」の続きを作ります。

> **検証メモ（この試し書きの自己評価）**
> - トーン：数式・専門用語ゼロで書けた。「正規化」「予算」は名前だけ出して続編送りにできた。○
> - 複雑さ予算：1 step に難所 1 つを守れた（1-2 で `type_of`、1-3 で `check`…）。新カリアは
>   `Const`/`Dynamic`/`Nominal` の 3 つだけ。FactStore も Scope もまだ出さない。○
> - 三題噺：①②③ が毎 step 自然に閉じた。特に「丸め問題」が③として綺麗に出た。○
> - 本編/続編の線：丸め＝正規化・予算を続編に送れた。基礎（脅かさない・untyped に逃がす）は
>   本編に残った。○
> - 要検討：`Nominal` を 1-2 で“こっそり”足したのは少し反則気味。1-1 で `Const`/`Dynamic`/
>   `Nominal` を 3 つ並べて出す方が素直かも。
