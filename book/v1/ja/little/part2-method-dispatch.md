---
title: Part 2 ― メソッド送信とディスパッチ
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

**全部、レシーバ（受け手）にメッセージを送っている**だけです。ここで**レシーバ**という言葉を
押さえます。多くの言語は `length("ab")` のように関数を*単独で*呼びますが、Ruby は違います ―
`"ab".length` のように、必ず**誰か（レシーバ）に対して**メソッドを呼びます。`1 + 2` も裏では
`1.+(2)`、つまり「`1` というレシーバに、`+` というメッセージを、引数 `2` を添えて送る」。引数の
ない `"ab".length` ですら `"ab"` がレシーバです。Ruby に*レシーバのない裸の関数*はほぼ無く、
`foo` と書けばそれも暗黙の `self` への `self.foo` です（この性質は Part 3 でもう一度効きます）。

これは型付けに直結します。同じ `+` でも、レシーバが `Integer` なら「整数＋整数→整数」、`String`
なら「文字列＋文字列→文字列」 ― **意味はレシーバで変わります**。だから「このメソッドは何を
返すか」はメソッド名だけでは決まらず、**レシーバの型とセット**で初めて決まる。「式の型を求める」の
大半が、結局「**このレシーバのこのメソッドは、何を返すか**」を知ることに尽きるのは、このため
です。Part 1 の `+` 専用コードを捨てて、ここを一般化します。

---

## 2-2. 手書きのディスパッチ表

`1.+(2)` のように「レシーバにメッセージを送る」と、Ruby は実行時に「そのレシーバにとって `+` とは
どの実装か」を選びます。この**送られたメッセージから実際のメソッドを選ぶ仕組み**を
**ディスパッチ（dispatch、振り分け）**と呼びます ― 章題の後半はこれです。私たちがやるのはその
*型版*です ― Ruby が*実行時*にメソッド本体を選ぶのに対し、私たちは*型チェック時*に「そのメソッドは
何を返すか（戻り型）」を選びます。`1.+(2)` を実際に走らせる代わりに、「`(Integer, +)` は何を返すか」を
**表から選ぶ**。だから、手書きの**ディスパッチ表**を用意します。

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
  Rigor はこの表が RBS）。本書ではいまは「素朴な表引き」に留めます（実物はもっと凝った解決を
  しますが、その全貌は 2-6 のまとめで案内します）。

---

## 2-3. 表に委ねる

ディスパッチの流れはこうです ― レシーバとメソッド名で表を引き、見つかれば引数を確かめて
戻り型を返す、見つからなければ黙って `untyped` を返す：

```text
  1 + "x"
    │ レシーバの型 = Integer、メソッド = :+、引数の型 = [String]
    ▼
  METHODS[[:Integer, :+]] ─ 見つかる ─→ 引数を accepts で確認 ─ 合わない ─→ 診断
    │                                                        └ 合う ─→ 戻り型 Integer
    └─ 見つからない（未知メソッド）─→ untyped（脅かさない）
```

つまり出口は 3 つです ― **表に無ければ**黙って `untyped`（脅かさない）、**有って引数が合えば**
戻り型、**有って引数が合わなければ**診断です。どの道も出発点は同じ「レシーバの型とメソッド名で表を
引けたか」です。

![図 2-1　メソッド送信のディスパッチ](../figures/svg/little-2-1.svg)
> ▼ 図 2-1　メソッド送信のディスパッチ

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

`dispatch` の中身です。表が見つかったら、引数の**数**と**型**を確かめます：

```ruby
def dispatch(receiver_type, name, arg_types, node, diagnostics)
  signature = METHODS[[class_of(receiver_type), name]]
  return Type::Dynamic.new unless signature # 知らないメソッド → 脅かさない（2-5）

  if arg_types.size != signature[:params].size
    diagnostics << Chibirigor.diagnostic(
      node, "wrong number of arguments for #{name} (#{signature[:params].size} expected, #{arg_types.size} given)"
    )
    return signature[:returns]
  end

  signature[:params].zip(arg_types).each do |param, arg|
    next if matches?(param, arg)

    diagnostics << Chibirigor.diagnostic(node, "expected #{param} but got #{arg}")
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
check('"a" + 1')        # ["expected String but got 1"]
check('"ab".length(1)') # ["wrong number of arguments for length (0 expected, 1 given)"]
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

<!-- run: examples/part2.rb -->
```text
1: String
1: Integer
expected Integer but got "x"
```

---

## 2-6. この章のまとめ

足したものは、`Dispatch` モジュール（`METHODS` 表・`class_of`・`matches?`・`dispatch`）です。
`type_of` 側はむしろ*短くなりました*（`+` 専用コードが消え、表に委ねるだけ）。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 3 章 / TAPL 9 章） | メソッドの型は「引数の型 → 戻りの型」 |
| ② Ruby/RBS | 何でもメソッド送信。オープンクラスで全部は表に書けない |
| ③ Rigor 実装の問題 | `(クラス, メソッド)→型` を表で引き、未知は `Dynamic` に degrade。引数判定は手書き（Part 7 で accepts に格上げ） |

素朴な一段の表引きで止めたのは、易しさのためです。ここから先は「その表をどう太らせ、どう
正しく引くか」という*深さ*の話 ― いまは行き先だけ置いておきます。

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
if recv.is_a?(Type::Const) && arg.is_a?(Type::Const)
  result = recv.value + arg.value
  return Type::Const[result] if result.abs <= 1_000_000   # 予算内 → 畳む
end
return Type::Nominal[:Integer]                              # 畳めない → 丸める
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

- **拡大（widening）**：際限なく大きな `Const` を抱えないよう、閾値を超えたら丸める。この
  「いつ畳むのをやめるか」を実 Rigor がどう体系立てるかは、後編で深掘りします。
- **誤検知ゼロ**：畳み込みは*精度を足すだけ*。`Const[3]` も `Integer` の所に通るので、新しい
  診断は一切増えません（`1 + "x"` のように畳めない式は、これまで通り丸めて元の挙動のまま）。

そしてここが本筋への回収です ― 実際の `chibirigor` では、この畳み込みは `+` の特別扱いではなく、
メソッドの**表**（この章の `Dispatch`）側に置いてあります。だから表を引くどの演算でも効き、
`x = 1; 1 + x` のように*変数が既知の `Const` を運んでいれば*、それも `2` に畳めます。手元の
`exe/chibirigor` で `1 + 2` が `3` と出るのは、畳み込みが Dispatch 側にいるからです。

---

## 演習

1. `Integer#*` を `METHODS` 表に足し、`check("2 * 3")` が空配列になることを確かめよ。
2. `1.to_s(2)`（引数過剰）を `check` し、出るメッセージを読め。アリティ判定は `dispatch` の
   どの分岐か。
3. 表に無いメソッド呼び出しが「黙って通る」例を 3 つ作り、なぜ脅かさないのかを説明せよ。

---

**次章予告（Part 3）**：ローカル変数と文。`x = 1` で型を覚え、`x` を読めるようにします。
ここで「型環境＝Scope」が登場します。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part2/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part2/lib)
