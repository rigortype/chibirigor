---
title: The Little chibirigor Part 2 ― メソッド送信とディスパッチ
description: メソッド呼び出しの型付けを手書きの「ディスパッチ表」で実装し、引数の型不整合を診断する。
sidebar:
  order: 3
---

# The Little chibirigor Part 2 ― メソッド送信とディスパッチ

この章のゴール：**メソッド呼び出しの型付けを、手書きの「ディスパッチ表」に委ねる。**
Ruby は何でもメソッド送信なので、ここが土台になります。

> 『しくみ』3 章「関数型」（TAPL 9 章「単純型付きラムダ計算」）に対応します。あの本は関数の型を
> `{ params, retType }` というデータで持ちました。私たちもほぼ同じ情報を、ただし*メソッドごとに
> 表で*持ちます。

---

## 2-0. 先に小さく整理する ― 型を `Type::` にまとめる

メソッドが増えると型キャリアも増えるので、Part 1 で `Chibirigor` 直下に置いた `Const`/`Nominal`/
`Dynamic` を、`Chibirigor::Type` モジュールにまとめておきます（以降は `Type::Const` のように
書きます。`diagnostic` ヘルパは Part 1 で作ったものをそのまま使います）。

```ruby
module Chibirigor
  module Type
    Const   = Data.define(:value) { def to_s = value.inspect }
    Nominal = Data.define(:name)  { def to_s = name.to_s }
    Dynamic = Data.define         { def to_s = "untyped" }
  end
end
```

これで土台が揃いました。本題に入ります。

---

## 2-1. Ruby は何でもメソッド送信

Part 1 で `1 + 2` の `+` を特別扱いしたとき、こう書きました ―「`+` はメソッド送信
（`1.+(2)`）です」と。これは `+` に限った話ではありません。

```ruby
1 + 2          # 1.+(2)
"ab".length    # "ab".length()
"a" * 3        # "a".*(3)
```

**全部、レシーバ（受け手）にメッセージを送っている**だけ。だから「式の型を求める」の大半は、
結局「**このレシーバのこのメソッドは、何を返すか**」を知ることに尽きます。Part 1 の `+` 専用
コードを捨てて、ここを一般化します。

---

## 2-2. 手書きのディスパッチ表

「どのクラスの・どのメソッドが・どんな引数を取り・何を返すか」を、素朴な表で持ちます：

```ruby
module Dispatch
  I = Type::Nominal[:Integer]
  S = Type::Nominal[:String]

  # [レシーバのクラス, メソッド名] => { params: [引数の型...], returns: 戻り型 }
  METHODS = {
    %i[Integer +]      => { params: [I], returns: I },
    %i[Integer to_s]   => { params: [],  returns: S },
    %i[String  +]      => { params: [S], returns: S },
    %i[String  length] => { params: [],  returns: I },
    # ...
  }.freeze
end
```

表を引くには、型を「クラス名」に丸める道具が要ります（`Const[1]` も `Nominal[:Integer]` も
`:Integer` に）：

```ruby
def class_of(type)
  case type
  when Type::Const   then type.value.class.name.to_sym
  when Type::Nominal then type.name
  end # Dynamic などは nil（＝引けない）
end
```

- **① 型理論**：関数（メソッド）の型は「引数の型 → 戻りの型」（『しくみ』 3 章 `{params, retType}`）。
- **② Ruby だと**：`+` も `length` も全部メソッド送信。型情報はメソッドごとに要る。
- **③ Rigor だと**：関数型のキャリアは持たず、`(クラス, メソッド) → 型` を*表で引く*（本物の
  Rigor はこの表が RBS）。ただし実物の Rigor は単純な「表引き」一段ではなく、複数の解決経路を
  順に当てる **5 段カスケード**で引きます（詳しくは付録 a3）。本書ではいまは「素朴な表引き」に
  留めます。

---

## 2-3. 表に委ねる

ディスパッチの流れはこうです ― レシーバとメソッド名で表を引き、見つかれば引数を確かめて
戻り型を返す、見つからなければ黙って `untyped`：

```text
  1 + "x"
    │ レシーバの型 = Integer、メソッド = :+、引数の型 = [String]
    ▼
  METHODS[[:Integer, :+]] ─ 見つかる ─→ 引数を accepts で確認 ─ 合わない ─→ 診断
    │                                                        └ 合う ─→ 戻り型 Integer
    └─ 見つからない（未知メソッド）─→ untyped（脅かさない）
```

> ▼ 図 2-1　メソッド送信のディスパッチ（`[図: 後で清書]`）

`type_of` のメソッド呼び出し部分は、**レシーバと各引数の型を求めて、表に渡すだけ**になります：

```ruby
def type_of_call(node, diagnostics)
  receiver = node.receiver ? type_of(node.receiver, diagnostics) : Type::Dynamic.new
  arg_types = (node.arguments&.arguments || []).map { |arg| type_of(arg, diagnostics) }
  Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
end
```

Part 1 では `+` の引数しか見ませんでしたが、いまは**全部の引数を `type_of` にかけます**。
おかげで `puts(1 + true)` のように*奥に潜んだ*エラーも見つかります（`puts` 自体は知らなくても、
引数 `1 + true` を型付けする途中で気づく）。

---

## 2-4. 引数の数・型を見る

`dispatch` の中身。表が見つかったら、引数の**数**と**型**を確かめます：

```ruby
def dispatch(receiver_type, name, arg_types, node, diagnostics)
  signature = METHODS[[class_of(receiver_type), name]]
  return Type::Dynamic.new unless signature # 知らないメソッド → 脅かさない（2-5）

  if arg_types.size != signature[:params].size
    diagnostics << Chibirigor.diagnostic(
      node, "#{name} の引数の数が違います（#{signature[:params].size} 個必要、#{arg_types.size} 個渡された）"
    )
    return signature[:returns]
  end

  signature[:params].zip(arg_types).each do |param, arg|
    next if matches?(param, arg)

    diagnostics << Chibirigor.diagnostic(node, "#{param} が必要ですが #{arg} が渡されました")
  end

  signature[:returns]
end
```

引数の型が合うかは、いまは素朴に「クラスが一致するか」で見ます：

```ruby
def matches?(param, arg)
  return true if param.is_a?(Type::Dynamic) || arg.is_a?(Type::Dynamic) # 不明は通す
  class_of(param) == class_of(arg)
end
```

```ruby
check('"a" + 1')        # ["String が必要ですが 1 が渡されました"]
check('"ab".length(1)') # ["length の引数の数が違います（0 個必要、1 個渡された）"]
```

> この `matches?` は **手書きの仮判定**です。Part 7 で、これを `:yes`/`:no`/`:maybe` の
> 三値を返す本物の `accepts` に置き換えます（「Part 1/2 の場当たりは accepts の手書き版だった」
> の回収）。いまは「クラス一致」で十分。

---

## 2-5. 知らないメソッドは脅かさない

表に無い `[クラス, メソッド]`、あるいはレシーバが `Dynamic`（型を見失っている）のときは、
**診断を出さず `Dynamic` を返します**（`dispatch` の最初の `return`）。

```ruby
check("foo.bar(1, 2)")   # []   ← foo も bar も知らない。黙って通す
```

これは Ruby の現実への態度です。Ruby は**オープンクラス**（既存クラスにメソッドを足せる）で、
`method_missing` もあり、メソッドは無数にあります。**全部を表に書くのは不可能**。だから
「表に無い＝怪しい」とは絶対にしない。知らないものは知らないまま、`untyped` で先へ進みます。

- **① 型理論**：未知の呼び出しをどう型付けするか。
- **② Ruby だと**：オープンクラス・無数のメソッド・`method_missing`。表は必ず不完全。
- **③ Rigor だと**：未知は `Dynamic` に degrade。本物の Rigor は手書き表の代わりに **RBS＋継承
  チェーン解決**で表を「本物」に近づける（Part 8 でひとさじ、本格解決は続編）。

---

## 2-6. この章のまとめ

足したもの：`Dispatch` モジュール（`METHODS` 表・`class_of`・`matches?`・`dispatch`）。
`type_of` 側はむしろ*短くなりました*（`+` 専用コードが消え、表に委ねるだけ）。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 3 章 / TAPL 9 章） | メソッドの型は「引数の型 → 戻りの型」 |
| ② Ruby/RBS | 何でもメソッド送信。オープンクラスで全部は表に書けない |
| ③ Rigor 実装の問題 | `(クラス, メソッド)→型` を表で引き、未知は `Dynamic` に degrade。引数判定は手書き（Part 7 で accepts に格上げ） |

**続編／後の Part に送ったもの**：

- 手書き表 → **RBS** からの本物の引き（Part 8）。
- 継承チェーン・モジュール mixin をたどったメソッド解決、`method_missing`、オープンクラスの
  本格対応（続編）。
- 引数判定の三値化（`accepts`）と robustness（Part 7）。
- 実物の dispatch 5 段カスケード（定数畳み込み → shape → RBS → in-source → fallback）の全貌は
  付録 a3。

---

## 2-7. 発展：定数畳み込み（畳めれば畳む）

> これは本筋から外した**発展ノート**です。Part 1 では `1 + 2` を `Integer` に「丸める」とだけ
> しましたが、ここではその丸めの**手前**で一手間かけて、畳める所は畳む話を重ねます。教える
> コード（2-2〜2-4 の表引き）はそのままに、`+` の解決に*ひとさじ*足すとどうなるかを見ます。

Part 1 で `1 + 2` を `Integer` に**丸めて**きました。でも `1` も `2` も*既知の値*です ― なら
**実際に足して `Const[3]` に畳める**はず。「値そのもの」をもう一段保てれば、`annotate` の精度が
上がります（実 Rigor の `Constant<3>` リテラル精度の縮図）。

やることは「両オペランドが既知値の `Const` なら計算する、ただし*大きくなりすぎたら*丸める」だけ。
`+` の解決に*ひとさじ*足すと、こうなります（丸める前に一度だけ畳みを試す）：

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
  「いつ畳むのをやめるか」が、実 Rigor では**正規化・推論予算**という大きなテーマになります
  （深掘りは付録 a1）。
- **誤検知ゼロ**：畳み込みは*精度を足すだけ*。`Const[3]` も `Integer` の所に通るので、新しい
  診断は一切増えません（`1 + "x"` のように畳めない式は、これまで通り丸めて元の挙動のまま）。

そしてここが本筋への回収です ― 実際の `chibirigor` では、この畳み込みは `+` の特別扱いではなく、
メソッドの**表**（この章の `Dispatch`）側に置いてあります。だから表を引くどの演算でも効き、
`x = 1; 1 + x` のように*変数が既知の `Const` を運んでいれば*、それも `2` に畳めます。手元の
`exe/chibirigor` で `1 + 2` が `3` と出るのは、畳み込みが Dispatch 側にいるからです。

> **`Const` と refinement carrier**：`Const[42]` は「値が `42` である」という超精密な型です。
> 実 Rigor はここからさらに踏み込んで、`unless n > 0` を通った後は `positive-int`（正の整数）の
> ような「述語を満たす値の集合」という型（**refinement carrier**）を持ちます。`Const` が
> 「ピンポイントの値」、refinement carrier は「条件を満たす値の範囲」 ― 精度の方向が異なります
> （詳しくは付録 a1、用語集にも一覧）。

---

## 演習

1. `Integer#*` を `METHODS` 表に足し、`check("2 * 3")` が空配列になることを確かめよ。
2. `1.to_s(2)`（引数過剰）を `check` し、出るメッセージを読め。アリティ判定は `dispatch` の
   どの分岐か。
3. 表に無いメソッド呼び出しが「黙って通る」例を 3 つ作り、なぜ脅かさないのかを説明せよ。

---

**次章予告（Part 3）**：ローカル変数と文。`x = 1` で型を覚え、`x` を読めるようにします。
ここで「型環境＝Scope」が登場します。
