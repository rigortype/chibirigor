---
title: The Little chibirigor Part 8 ― RBS と型シグネチャ
description: "手書きの `METHODS` 表を RBS 由来に差し替え、その上で `def` を読み本体から戻り型を合成して RBS 風シグネチャを出力する。"
sidebar:
  order: 9
---

# The Little chibirigor Part 8 ― RBS と型シグネチャ

この章のゴール：**型を「コードの外」に出し、そして「コードの中」から取り戻す。**
前半で、手書きの `METHODS` 表を *別ファイル*（RBS）から読み込んだ表に差し替えます ―
差し替えても診断は 1 つも変わらない、それが正しさの証拠です。後半で、その RBS の記法を
お手本に、注釈ゼロのメソッドの **`def` を読み、本体から戻り型を合成して** RBS 風のシグネチャを
出力します。ここで `chibirigor` が**推論を土台にしている**ことが一番はっきりします ―
注釈ゼロのメソッドでも、本体の最後の式から戻り型が決まる。

> 『しくみ』9 章「ジェネリクス」（TAPL 23 章「全称型／System F」）の*型代入*が遠い親戚ですが、ここで本当に
> 出会うのは Ruby/RBS 固有の世界観 ―「**型はコードではなく、別ファイル（.rbs）に書く**」です。
> 後半は逆向きに、TAPL 22 章「型再構築」― 注釈の無いコードから型を立ち上げる側へ進みます。

---

## 8-1. 型は「別ファイル」に書く ― RBS

ここまで、メソッドの型は Ruby のコード（`METHODS` 表）に直接書いてきました。でも Ruby 本来の
やり方は違います。Ruby のコードには型注釈を**書きません**。代わりに、型は **RBS** という
*別ファイル*（`.rbs`）に書きます ― こうすると、**動いている Ruby のコードを 1 文字も変えずに、
後から型情報だけを足せる**のが狙いです：

```rbs
class Integer
  def +: (Integer) -> Integer
  def to_s: () -> String
end
```

初めて型を「書いた」記法かもしれないので、読み下しておきます。`def +: (Integer) -> Integer` は
「`Integer#+` は **`Integer` を 1 つ受け取り、`Integer` を返す**」。`:` の右が型、`(...)` が引数の型、
`->` の右が**戻り型**です。`def to_s: () -> String` なら「引数なしで `String` を返す」。
― Ruby の `def` の頭に、引数と戻りの型を書き添えただけ、と読めば十分です。
（Ruby だと `->` はラムダを作る記号ですが〔`square = ->(x) { x * x }`〕、RBS では「引数を受け取り→戻り型を返す」という*別物の矢印*です。同じ記号で意味が違う、と割り切ってください。）

これが Ruby/RBS の世界観です。「コードは型のことを知らない。型は外から与える」。Rigor は
この RBS を**正**として読み、その上にさらに精度を足していく。

- **① 型理論**：宣言された型を引いて使う（『しくみ』 9 章の型代入の遠縁）。
- **② Ruby だと**：コードに型注釈は無い。型は `.rbs` に別書き。
- **③ Rigor だと**：RBS を真実の源として読む。手書き表は、その RBS の*ミニ版*だった。

---

## 8-2. ごく小さな RBS を読む

本物の `rbs` gem を使うのが理想ですが、ここでは chibirigor 流に**最小限を自前で読みます**
（依存を増やさない／何が起きているか全部見える）。扱う形は `class` と `def 名: (引数) -> 戻り`
の 2 種類だけ：

```ruby
module Rbs
  CLASS_LINE = /\A\s*class\s+(\S+)\s*\z/
  DEF_LINE   = /\A\s*def\s+(\S+):\s*\((.*)\)\s*->\s*(\S+)\s*\z/

  def load(source)
    table = {}
    current = nil
    source.each_line do |line|
      if (m = CLASS_LINE.match(line))
        current = m[1].to_sym
      elsif current && (m = DEF_LINE.match(line))
        params = m[2].split(",").map(&:strip).reject(&:empty?).map { |t| Type::Nominal[t.to_sym] }
        table[[current, m[1].to_sym]] = { params: params.freeze, returns: Type::Nominal[m[3].to_sym] }
      end
    end
    table.freeze
  end
end
```

`def +: (Integer) -> Integer` の 1 行が `[:Integer, :+] => { params: [Integer], returns: Integer }` に
なる、それだけ。本物の RBS はもっと豊かですが、骨は同じ「宣言を表にする」です。

---

## 8-3. 手書き表を RBS 由来に差し替える

`Dispatch` の `METHODS` を、手書きリテラルから RBS 読み込みに差し替えます：

```ruby
module Dispatch
  # 以前は手書きリテラル。いまは RBS テキストから生成。
  METHODS = Rbs.load(Rbs::CORE)
end
```

`Rbs::CORE` には、ディスパッチに必要なコア型のメソッドを RBS テキストで書いておきます
（Part 2 の手書き表と同じ内容＋、後の章で使う `*`・`upcase` も含めた“完全版”）：

```ruby
module Rbs
  CORE = <<~RBS
    class Integer
      def +: (Integer) -> Integer
      def -: (Integer) -> Integer
      def *: (Integer) -> Integer
      def to_s: () -> String
    end
    class String
      def +: (String) -> String
      def *: (Integer) -> String
      def length: () -> Integer
      def upcase: () -> String
    end
  RBS
end
```

内容が手書き表と同じなので、差し替えても **診断は 1 つも変わりません**。Part 1〜7 のテストが
全て緑のまま、というのがその証拠（＝ふるまいを変えずに土台だけ入れ替える、安全なリファクタ）。

```console
$ ruby test/test_part1.rb  # … 緑
$ ruby test/test_part7.rb  # … 緑（表の出どころが変わっただけ）
```

- **① 型理論**：型の出どころを宣言（RBS）に一元化。
- **② Ruby だと**：`.rbs` が型の単一の源。
- **③ Rigor だと**：手書き表 → RBS 由来へ。ふるまいは変わらない（differ 置換 ― 外から見た挙動を変えずに内部実装だけ入れ替えるリファクタの呼び方）。

ここまでで、型を「コードの外」（RBS）から読む土台ができました。次は逆向きです ― 注釈の
**無い**メソッドのコードを読んで、その RBS 記法のシグネチャを*こちらから合成して*みせます。

---

## 8-4. 戻り型は本体から合成できる

Ruby のメソッドには型注釈がありません。でも**戻り型は本体から分かる**ことが多い：

```ruby
def greet
  "hi".upcase   # String を返す
end
```

`"hi".upcase` の型は（前節までの RBS 表から）`String`。メソッドの戻り型は本体の最後の式の型
そのもの。だから合成できます。`type_of` に `def` を足します：

```ruby
when Prism::DefNode then type_of_def(node, scope, diagnostics)

def type_of_def(node, scope, diagnostics)
  method_return_type(node, scope, diagnostics)  # 本体を型チェック（診断も集まる）
  Type::Const[node.name]                        # def 式の値はメソッド名シンボル
end

def method_return_type(node, scope, diagnostics)
  # 仮引数は untyped（本編は引数推論しない＝続編）
  body_scope = method_param_names(node).reduce(scope) { |s, n| s.with_local(n, Type::Dynamic.new) }
  type_of_body(node.body, body_scope, diagnostics)
end
```

ここで使った小さな道具が 2 つあります。`method_param_names` は必須の仮引数名を取り出すだけ。
`type_of_body` は「文の並びを上から評価して、**最後の文の型**を返す」ヘルパで、Part 3 の
`eval_statement`（文を 1 つ評価して `[型, スコープ]` を返す）を使い回します（`if` の枝の本体や
`def` の本体は、どれも「文の並び」なので同じ道具で扱えます）：

```ruby
def method_param_names(node)
  node.parameters&.requireds&.map(&:name) || []
end

# 文の並びを評価し、最後の文の型を返す（枝の中でもスコープを縫う）
def type_of_body(statements_node, scope, diagnostics)
  return Type::Const[nil] if statements_node.nil?   # 空の本体は nil

  last = Type::Const[nil]
  statements_node.body.each { |stmt| last, scope = eval_statement(stmt, scope, diagnostics) }
  last
end
```

これで **`def` の本体も型チェックされる**ようになりました（`check` が `def bad; 1 + "x"; end` の
中のエラーを拾う）。引数は `untyped` なので、`def ok(x); x + 1; end` は誤検知しません
（`untyped + Integer` は `:maybe` → 黙る）。

> `type_of_body` は空の本体を `nil` 型にしましたが、RBS にはもう一段ゆるい戻り型 **`void`**
> （「値は返るが当てにするな」）もあります。これが効くのは**契約**の面です ― 戻り型を `-> nil` と
> 宣言すると「nil を返す」と約束したことになり、後で別の値を返すよう実装を変えると契約違反。
> いっぽう `void` は呼び出し側に「戻り値に依存するな」と約束させるので、**実装が後で戻り値を
> 変えても破壊的変更（BC break）になりません**。副作用のために呼ぶメソッドに `void` を選ぶ実益が
> ここにあります。chibirigor は戻り型を*合成する*側（注釈を*検証*するのではなく*作る*）なので
> `void` は登場しません ― 私たちは常に「最後の式の型」という具体的な型を出します。`void`／`never`
> ／`untyped` の特別な型 3 種の総括は **Part 9**、格子上での位置づけ（⊤ の別名）は付録
> [a1-2](../appendix/a1-special-types.md) にまとめてあります。

---

## 8-5. RBS 風に見せる

`annotate` は、文が `def` のときだけシグネチャ文字列を、それ以外は今までどおり推論した型を
返します。文の種類で分岐するだけ：

```ruby
def annotate(source)
  program = Prism.parse(source).value
  scope = Scope.new
  ignored = []
  program.statements.body.map do |stmt|
    if stmt.is_a?(Prism::DefNode)
      { line: stmt.location.start_line, type: method_signature(stmt, scope, ignored) }
    else
      type, scope = eval_statement(stmt, scope, ignored)
      { line: stmt.location.start_line, type: type }
    end
  end
end

def method_signature(node, scope, diagnostics)
  params = method_param_names(node).map { "untyped" }.join(", ")
  "def #{node.name}: (#{params}) -> #{method_return_type(node, scope, diagnostics)}"
end
```

```console
$ printf 'def greet\n  "hi".upcase\nend\n' | ruby exe/chibirigor annotate /dev/stdin
```

<!-- run: examples/part8.rb -->
```text
1: def greet: () -> String
```

`check` と `annotate` は**同じ推論エンジン**（`type_of`/`method_return_type`）を使います。
推論が土台で、チェックも表示もその出力を使う ― これが Part 0 で言った「推論を土台にした
型チェッカー」の姿です。

---

## 8-6. `untyped` がどこに出るか＝推論の弱点

引数を `untyped` にしているので、それが戻りまで流れると `untyped` が顔を出します
（`n が untyped → n * 2 も untyped` のように伝播する）：

<!-- run: examples/part8.rb -->
```text
1: def double: (untyped) -> untyped
1: def mystery: (untyped) -> untyped
```

この **`untyped` の出方そのものが「推論が型を見失った場所」** です。どこを直せば型が
通るようになるかが、ひと目で分かる。これは Rigor の `sig-gen`（RBS を生成する機能）の発想の
芽です ― 生成された RBS の `untyped` は「人間が型を足すべき場所」を指しています。

> ここで `def double(n)` の引数 `n` を `untyped` のままにしているのは、**設計判断**です。
> Ruby 同梱の TypeProf なら、`double` が `double(3)` のように**呼ばれている場所**を見つけて
> `n` を `Integer` まで逆算し、`(Integer) -> Integer` を埋めてくれます。chibirigor（と Rigor）は
> あえてそれをしません ― 呼び出し元を全部たどる代わりに、各メソッドをローカルに見て、分から
> ない引数は `untyped` に倒す（その方がスケールするし、誤検知も出ない）。**引数を使われ方から
> 当てる「本物の推論」は、後編 Part 5 で正面から扱います。**

- **① 型理論**：本体から戻り型を合成する（注釈なしでも型が立つ）。
- **② Ruby/RBS**：メソッドに注釈は無いが、戻りは本体から分かることが多い。
- **③ Rigor 実装の問題**：合成した型を RBS 風に見せ、`untyped` で推論の穴を可視化する。

---

## 8-7. この章のまとめ

足したもの（前半）：`Rbs.load`（ごく小さな RBS リーダー）と `Rbs::CORE`。`Dispatch::METHODS` の
*出どころ*だけが変わり、ふるまいは変わりませんでした。
足したもの（後半）：`type_of` の `DefNode` 対応（本体チェック＋戻り型合成）、`annotate` の
`method_signature`。型を「外」から読む土台の上に、型を「中」から立ち上げる仕掛けが乗りました。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 9 章 / TAPL 22・23 章） | 宣言された型を引いて使う（型代入の遠縁）／本体から戻り型を合成（注釈ゼロでも型が立つ） |
| ② Ruby/RBS | 型はコードに書かず別ファイル `.rbs` に書く／メソッドに注釈は無いが戻りは本体から分かる |
| ③ Rigor 実装の問題 | RBS を真実の源に（differ 置換）／RBS 風 sig で見せ `untyped` で推論の弱点を可視化（sig-gen の芽） |

前編で組んだのは「型は別ファイル・戻りは本体から」という骨格までです。残りは後編で*正式な
名前*とともに扱い直す宿題 ― 前編の最後に、その行き先を一望しておきます。

**続編／後の Part に送ったもの**：

- **引数の推論**（本体での使われ方から `x` の型を当てる）。本編は引数＝`untyped` 止まり ―
  この型推論の本丸は **後編 Part 5** で正面から扱う。
- 本物の `rbs` gem を使った完全な RBS 読み込み（union・optional・ブロック・ジェネリクス）と
  型変数の置換（`Array[Elem]`→`Array[String]`）、継承チェーンのメソッド解決。
- 複数 `return` をまたぐ戻り型の合流と、生成した RBS の書き出し（erasure）。深掘りは後編 Part 3。

## 演習

1. `Rbs::CORE` に `String#downcase: () -> String` を足し、`"A".downcase` が通ることを確かめよ。
2. 自前ミニ RBS リーダーが**扱えない** RBS 構文を 1 つ挙げよ（例：union 型 `Integer | String`、
   optional の `?`、ブロック）。扱うには `DEF_LINE` の正規表現に何が要るか。
3. 表を RBS 由来に差し替えても Part 1〜7 のテストが緑のままであることを確かめ、「differ 置換」
   （ふるまいを変えずに土台を入れ替える）の意味を自分の言葉で説明せよ。
4. `def f\n  1 + 2\nend` のシグネチャを `annotate` で確かめよ。
5. `def g(x)\n  x.upcase\nend` の戻り型はなぜ `untyped` か。`String` を出すには何が必要か
   （ヒント：引数の型推論＝後編 Part 5 の話）。
6. 本体にエラーのある `def bad\n  1 + "x"\nend` を `check` し、診断の行番号が本体の行を指す
   ことを確かめよ。

---

**次章予告（Part 9・最終章）**：ここまでを `gradual` の哲学で締めます。`untyped` の伝播を
仕上げ、`untyped`／`void`／`never` の「特別な型 3 種」を総括し、「chibirigor はわざと見逃すことで
動くコードを脅かさない」を語り切ります。『しくみ』が結びで発展先の一つに挙げた
gradual typing へと接続して、本編を閉じます。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part8/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part8/lib)
