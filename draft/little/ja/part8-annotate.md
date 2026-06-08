---
title: "Part 8 ― annotate を仕上げる"
description: "`def` を読み、本体から戻り型を合成して RBS 風シグネチャを出力する `annotate` を完成させる。"
sidebar:
  order: 8
---

# The Little chibirigor Part 8 ― annotate を仕上げる

この章のゴール：**`def` を読み、本体から戻り型を合成して RBS 風シグネチャで見せる。**
ここで `chibirigor` が**推論を土台にしている**ことが一番はっきりします ― 注釈ゼロのメソッドから、
型が立ち上がってくる。

---

## 8-1. 戻り型は本体から合成できる

Ruby のメソッドには型注釈がありません。でも**戻り型は本体から分かる**ことが多い：

```ruby
def greet
  "hi".upcase   # String を返す
end
```

`"hi".upcase` の型は（Part 2/7 の表から）`String`。メソッドの戻り型は本体の最後の式の型
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

> **コラム：「値を返すが、見るな」 ― `void` 型**
>
> 上の `type_of_body` は空の本体を `nil` 型としました。Ruby のメソッドは*必ず*何かを返す
> （副作用目的のメソッドも最後に `nil` などを返す）ので、戻り型としての `nil` は自然です。
> でも RBS には、もう一段ゆるい戻り型 **`void`** があります。意味は「**値は返るが、その値を
> 当てにするな（見るな）**」。`Array#each` のように「副作用のために呼ぶ」メソッドに付けます。
>
> ここで Ruby ならではの妙があります。Ruby のメソッドは**最後に評価した式が暗黙の戻り値**に
> なるので、`void` を付けたメソッドも実行時には*何か*を返します ― しかも実装を書き替えれば
> その値はいくらでも変わりえます。`void` はその現実を型として正直に認めた宣言で、「このメソッドは
> **どんな値を返す可能性もある**」と言っているに等しい。だからこそ重要なのが**契約**の面です：
> 戻り型 `void` は呼び出し側に「戻り値に依存するな」と約束させるので、**実装が後から戻り値を
> 変えても破壊的変更（BC break）にならない**。逆に `-> nil` と宣言してしまうと「nil を返す」を
> 約束したことになり、後で別の値を返すよう変えると契約違反 ― ここに `void` を選ぶ実益があります。
>
> ここが初学者の混同ポイント。前に見た 3 つの「特別な型」を並べると軸がはっきりします：
>
> | 型 | 制御は戻る？ | 値は？ | 一言 |
> |---|---|---|---|
> | `unknown` / `untyped`（Part 1） | ― | あるが**型が不明** | 「中身が分からない」 |
> | `void`（この章） | **戻る** | あるが**見るな** | 「戻り値を当てにするな」 |
> | `never` / `Bot`（Part 4） | **戻らない** | **無い**（住人ゼロ） | 「そもそも返ってこない」 |
>
> `void` は「値の中身が分からない」（`unknown`）でも「返ってこない」（`never`）でもなく、
> **戻り位置に立てる『黙れ』の印** ― チェッカーに「ここの戻り値は照合しなくていい」と告げる、
> という点で `untyped` の親戚です（ただし対象は*値*ではなく*戻り値の使われ方*）。
> TypeScript の `void`、C/Java の `void`（こちらは「値が無い」寄り）も同じ位置の家族です。
>
> 実は RBS の型システム上、`void` は**トップ型（⊤）の別名**として扱われます ― 「どんな値でも
> 受け付ける／その値からは何も引き出せない」という*格子上のふるまい*はトップ型とまったく同じ
> だからです（だから上の表で `void` を `unknown`＝トップ型の隣に置きました）。違うのは**ニュアンス
> （込めたメッセージ）**だけ。トップ型／`unknown` は「ここには意味のある値がある。使う前に絞れ」と
> 言い、`void` は「ここに値はあるが意味は無い。**そもそも使うな**」と言う。同じ格子の位置から、
> 読み手に正反対の合図を送る ― 型システムは同一でも*意図*が違う、という良い例です。
>
> chibirigor は戻り型を**本体から合成する**側（注釈を*検証*するのではなく*作る*）なので、
> `void` は登場しません ― 私たちは常に「最後の式の型」という具体的な型を出します。
> `void` が効くのは、RBS で*宣言された*戻り型を本体に照合する後編 Part 1 の `⇐` 側です。

---

## 8-2. RBS 風に見せる

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
1: def greet: () -> String
```

`check` と `annotate` は**同じ推論エンジン**（`type_of`/`method_return_type`）を使います。
推論が土台で、チェックも表示もその出力を使う ― これが Part 0 で言った「推論を土台にした
検査器」の姿です。

---

## 8-3. `untyped` がどこに出るか＝推論の弱点

引数を `untyped` にしているので、それが戻りまで流れると `untyped` が顔を出します：

```console
1: def double: (untyped) -> untyped     # n が untyped → n * 2 も untyped
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
> 当てる「本物の推論」は、後編 Part 3 で正面から扱います。**

- **① 型理論**：本体から戻り型を合成する（注釈なしでも型が立つ）。
- **② Ruby/RBS**：メソッドに注釈は無いが、戻りは本体から分かることが多い。
- **③ Rigor 実装の問題**：合成した型を RBS 風に見せ、`untyped` で推論の穴を可視化する。

---

## 8-4. この章のまとめ

足したもの：`type_of` の `DefNode` 対応（本体チェック＋戻り型合成）、`annotate` の `method_signature`。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（TAPL 22 章「型再構築」） | 本体から戻り型を合成（注釈ゼロでも型が立つ） |
| ② Ruby/RBS | メソッドに注釈は無いが戻りは本体から分かる |
| ③ Rigor 実装の問題 | RBS 風 sig で見せ、`untyped` で推論の弱点を可視化（sig-gen の芽） |

**続編に送ったもの**：

- **引数の推論**（本体での使われ方から `x` の型を当てる＝ capability / duck 推論）。本編は
  引数＝`untyped` 止まり。これが『しくみ』 9 章演習「正解を知らない」と言った前線（TAPL なら 22 章「型再構築」が型推論の本拠地）。
- 複数 `return`・early return・条件分岐をまたぐ戻り型の合流。
- 生成した RBS の書き出し（`erasure`／sig-gen 本体）。

## 演習

1. `def f\n  1 + 2\nend` のシグネチャを `annotate` で確かめよ。
2. `def g(x)\n  x.upcase\nend` の戻り型はなぜ `untyped` か。`String` を出すには何が必要か
   （ヒント：引数の型推論＝続編の話）。
3. 本体にエラーのある `def bad\n  1 + "x"\nend` を `check` し、診断の行番号が本体の行を指す
   ことを確かめよ。

---

**次章予告（Part 9・最終章）**：ここまでを `gradual` の哲学で締めます。`untyped` の伝播を
仕上げ、「chibirigor はわざと見逃すことで動くコードを脅かさない」を総括し、『しくみ』
『型システムのしくみ』のおわりに（gradual typing への伏線）と接続します。

