---
title: Part 8　RBS と型シグネチャ
description: "手書きの `METHODS` 表を RBS 由来に差し替え、その上で `def` を読み本体から戻り型を合成して RBS 風シグネチャを出力する。"
sidebar:
  order: 9
---

# The Little chibirigor Part 8　RBSと型シグネチャ

この章のゴールは、型を「コードの外」に出し、そして「コードの中」から取り戻すことです。前半で、手書きの`METHODS`表を別ファイル（RBS）から読み込んだ表に差し替えます。差し替えても診断は1つも変わらない、それが正しさの証拠です。

後半で、そのRBSの記法を手本に、注釈ゼロのメソッドの`def`を読み、本体から戻り型を合成してRBS風のシグネチャを出力します。ここで`chibirigor`が推論を土台にしていることが一番はっきりします。注釈ゼロのメソッドでも、本体の最後の式から戻り型が決まるのです。

> [!NOTE]
> 『しくみ』9章「ジェネリクス」（TAPL 23章「全称型／System F」）の*型代入*が遠い親戚ですが、ここで本当に出会うのはRuby/RBS固有の世界観です。「**型はコードではなく、別ファイル（.rbs）に書く**」という考え方です。
>
> 後半は逆向きに、TAPL 22章「型再構築」の側へ進みます。注釈の無いコードから型を立ち上げる方向です。

---

## 8-1. 型は「別ファイル」に書く（RBS）

ここまで、メソッドの型はRubyのコード（`METHODS`表）に直接書いてきました。でもRuby本来のやり方は違います。Rubyのコードには型注釈を**書きません**。代わりに、型は**RBS**という別ファイル（`.rbs`）に書きます。こうすると、動いているRubyのコードを1文字も変えずに、後から型情報だけを足せるのが狙いです。

```rbs
class Integer
  def +: (Integer) -> Integer
  def to_s: () -> String
end
```

初めて型を「書いた」記法かもしれないので、読み下しておきます。`def +: (Integer) -> Integer`は「`Integer#+`は`Integer`を1つ受け取り、`Integer`を返す」という意味です。`:`の右が型、`(...)`が引数の型、`->`の右が**戻り型**です。`def to_s: () -> String`なら「引数なしで`String`を返す」です。Rubyの`def`の頭に、引数と戻りの型を書き添えただけ、と読めば十分です。

Rubyだと`->`はラムダを作る記号ですが〔`square = ->(x) { x * x }`〕、RBSでは「引数を受け取り→戻り型を返す」という別物の矢印です。同じ記号で意味が違う、と割り切ってください。

これがRuby/RBSの世界観です。「コードは型のことを知らない。型は外から与える」。RigorはこのRBSを**正**として読み、その上にさらに精度を足していきます。

- **① 型理論**：宣言された型を引いて使います（『しくみ』 9章の型代入の遠縁）
- **② Rubyだと**：コードに型注釈は無く、型は`.rbs`に別書きします
- **③ Rigorだと**：RBSを真実の源として読みます。手書き表は、そのRBSのミニ版でした

---

## 8-2. ごく小さなRBSを読む

本物の`rbs` gemを使うのが理想ですが、ここではchibirigor流に最小限を自前で読みます。依存を増やさず、何が起きているか全部見えるようにするためです。扱う形は`class`と`def 名: (引数) -> 戻り`の2種類だけです。

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

`def +: (Integer) -> Integer`の1行が`[:Integer, :+] => { params: [Integer], returns: Integer }`になる、それだけです。本物のRBSはもっと豊かですが、骨は同じ「宣言を表にする」です。

---

## 8-3. 手書き表をRBS由来に差し替える

`Dispatch`の`METHODS`を、手書きリテラルからRBS読み込みに差し替えます。

```ruby
module Dispatch
  # 以前は手書きリテラル。いまは RBS テキストから生成。
  METHODS = Rbs.load(Rbs::CORE)
end
```

`Rbs::CORE`には、ディスパッチに必要なコア型のメソッドをRBSテキストで書いておきます。Part 2の手書き表と同じ内容に、後の章で使う`*`と`upcase`も含めた完全版です。

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

内容が手書き表と同じなので、差し替えても診断は1つも変わりません。Part 1から7のテストが全て緑のまま、というのがその証拠です。ふるまいを変えずに土台だけ入れ替える、安全なリファクタです。

```console
$ ruby test/test_part1.rb  # … 緑
$ ruby test/test_part7.rb  # … 緑（表の出どころが変わっただけ）
```

- **① 型理論**：型の出どころを宣言（RBS）に一元化します
- **② Rubyだと**：`.rbs`が型の単一の源です
- **③ Rigorだと**：手書き表からRBS由来へ差し替えます。ふるまいは変わりません。外から見た挙動を変えずに、内部実装だけを差し替えるわけです

ここまでで、型を「コードの外」（RBS）から読む土台ができました。次は逆向きです。注釈の**無い**メソッドのコードを読んで、そのRBS記法のシグネチャをこちらから合成してみせます。

---

## 8-4. 戻り型は本体から合成できる

Rubyのメソッドには型注釈がありません。でも**戻り型は本体から分かる**ことが多いです。

```ruby
def greet
  "hi".upcase   # String を返す
end
```

`"hi".upcase`の型は（前節までのRBS表から）`String`です。メソッドの戻り型は本体の最後の式の型そのものです。だから合成できます。`type_of`に`def`を足します。

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

ここで使った小さな道具が2つあります。`method_param_names`は必須の仮引数名を取り出すだけです。`type_of_body`は「文の並びを上から評価して、最後の文の型を返す」ヘルパで、Part 3の`eval_statement`を使い回します。`eval_statement`は文を1つ評価して`[型, スコープ]`を返すメソッドです。`if`の枝の本体や`def`の本体は、どれも「文の並び」なので同じ道具で扱えます。

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

これで`def`の本体も型チェックされるようになりました。`check`が`def bad; 1 + "x"; end`の中のエラーを拾います。引数は`untyped`なので、`def ok(x); x + 1; end`は誤検知しません。`untyped + Integer`は`:maybe`で黙ります。

> [!NOTE]
> `type_of_body`は空の本体を`nil`型にしましたが、RBSにはもう一段ゆるい戻り型 **`void`**（「値は返るが当てにするな」）もあります。これが効くのは**契約**の面です。
>
> 戻り型を`-> nil`と宣言すると「nilを返す」と約束したことになり、後で別の値を返すよう実装を変えると契約違反になります。いっぽう`void`は呼び出し側に「戻り値に依存するな」と約束させるので、**実装が後で戻り値を変えても破壊的変更（BC break）になりません**。副作用のために呼ぶメソッドに`void`を選ぶ実益がここにあります。
>
> chibirigorは戻り型を*合成する*側（注釈を*検証*するのではなく*作る*）なので`void`は登場しません。私たちは常に「最後の式の型」という具体的な型を出します。`void`／`never`／`untyped`の特別な型3種の総括は**Part 9**、格子上での位置づけ（⊤ の別名）は付録[a1-2](../appendix/a1-special-types.md)にまとめてあります。

---

## 8-5. RBS風に見せる

`annotate`は、文が`def`のときだけシグネチャ文字列を、それ以外は今までどおり推論した型を返します。文の種類で分岐するだけです。

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

`check`と`annotate`は**同じ推論エンジン**（`type_of`／`method_return_type`）を使います。推論が土台で、チェックも表示もその出力を使います。これがPart 0で言った「推論を土台にした型チェッカー」の姿です。

---

## 8-6. `untyped`がどこに出るか＝推論の弱点

引数を`untyped`にしているので、それが戻りまで流れると`untyped`が顔を出します。`n が untyped → n * 2 も untyped`のように伝播します。

<!-- run: examples/part8.rb -->
```text
1: def double: (untyped) -> untyped
1: def mystery: (untyped) -> untyped
```

この **`untyped`の出方そのものが「推論が型を見失った場所」** です。どこを直せば型が通るようになるかが、ひと目で分かります。

これはRigorの`sig-gen`（RBSを生成する機能）の発想の芽です。生成されたRBSの`untyped`は「人間が型を足すべき場所」を指しています。

> [!NOTE]
> ここで`def double(n)`の引数`n`を`untyped`のままにしているのは、**設計判断**です。Ruby同梱のTypeProfなら、`double`が`double(3)`のように**呼ばれている場所**を見つけて`n`を`Integer`まで逆算し、`(Integer) -> Integer`を埋めてくれます。
>
> chibirigor（とRigor）はあえてそれをしません。呼び出し元を全部たどる代わりに、各メソッドをローカルに見て、分からない引数は`untyped`に倒します。その方がスケールするし、誤検知も出ません。
>
> **引数を使われ方から当てる「本物の推論」は、後編Part 5で扱います。**

- **① 型理論**：本体から戻り型を合成します（注釈なしでも型が立ちます）
- **② Ruby/RBS**：メソッドに注釈は無いですが、戻りは本体から分かることが多いです
- **③ Rigor実装の問題**：合成した型をRBS風に見せ、`untyped`で推論の穴を可視化します

---

## 8-7. この章のまとめ

足したもの（前半）は、`Rbs.load`（ごく小さなRBSリーダー）と`Rbs::CORE`です。`Dispatch::METHODS`の出どころだけが変わり、ふるまいは変わりませんでした。足したもの（後半）は、`type_of`の`DefNode`対応（本体チェックと戻り型合成）と`annotate`の`method_signature`です。型を「外」から読む土台の上に、型を「中」から立ち上げる仕掛けが乗りました。

この章の三つの視点：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 9章 / TAPL 22、23章） | 宣言された型を引いて使う（型代入の遠縁）／本体から戻り型を合成（注釈ゼロでも型が立つ） |
| ② Ruby/RBS | 型はコードに書かず別ファイル`.rbs`に書く／メソッドに注釈は無いが戻りは本体から分かる |
| ③ Rigor実装の問題 | RBSを真実の源に（挙動を変えず土台だけ差し替え）／RBS風sigで見せ`untyped`で推論の弱点を可視化（sig-genの芽） |

前編で組んだのは「型は別ファイル、戻りは本体から」という骨格までです。残りは後編で正式な名前とともに扱い直す宿題です。前編の最後に、その行き先を一望しておきます。

**続編／後のPartに送ったもの**：

- **引数の推論**（本体での使われ方から`x`の型を当てる）。本編は引数＝`untyped`止まりです。この型推論の本丸は**後編Part 5**で扱います
- 本物の`rbs` gemを使った完全なRBS読み込み（union、optional、ブロック、ジェネリクス）と型変数の置換（`Array[Elem]`→`Array[String]`）、継承チェーンのメソッド解決
- 複数`return`をまたぐ戻り型の合流と、生成したRBSの書き出し（erasure）。後編Part 3で詳しく扱います

## 演習

1. `Rbs::CORE`に`String#downcase: () -> String`を足し、`"A".downcase`が通ることを確かめよ。
2. 自前ミニRBSリーダーが**扱えない** RBS構文を1つ挙げよ（例：union型`Integer | String`、
   optionalの`?`、ブロック）。扱うには`DEF_LINE`の正規表現に何が要るか。
3. 表をRBS由来に差し替えてもPart 1〜7のテストが緑のままであることを確かめ、「ふるまいを
   変えずに土台を差し替える」とはどういうことか、自分の言葉で説明せよ。
4. `def f\n  1 + 2\nend`のシグネチャを`annotate`で確かめよ。
5. `def g(x)\n  x.upcase\nend`の戻り型はなぜ`untyped`か。`String`を出すには何が必要か
   （ヒント：引数の型推論＝後編Part 5の話）。
6. 本体にエラーのある`def bad\n  1 + "x"\nend`を`check`し、診断の行番号が本体の行を指す
   ことを確かめよ。

---

**次章予告（Part 9、最終章）**：ここまでを`gradual`の哲学で締めます。`untyped`の伝播を仕上げ、`untyped`／`void`／`never`の「特別な型3種」を総括します。「chibirigorはわざと見逃すことで動くコードを脅かさない」を語り切り、『しくみ』が結びで発展先の一つに挙げたgradual typingへと接続して、本編を閉じます。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part8/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part8/lib)
