---
title: The Little chibirigor Part 7 ― 受理判定と「たぶん」
description: "「この型を渡して合う？」を三値（`:yes`／`:no`／`:maybe`）で判定する `accepts` を作る。"
sidebar:
  order: 8
---

# The Little chibirigor Part 7 ― 受理判定と「たぶん」

この章のゴール：**「この型、ここに渡して合う？」を判定する `accepts` を作る。** ただし
答えは `はい`／`いいえ` の二択ではなく、**`はい`／`いいえ`／`たぶん`** の三択にします。
この「たぶん」が、Rigor がやさしくいられる最大の理由です。

> 『しくみ』では 7 章「部分型付け」（TAPL 15 章「部分型付け」）が対応します。あの本は「合う／合わない」を
> `true`/`false` の二択で答えました。私たちはそこに一つ足します。

---

## 7-1. 「合う？」という新しい問い

これまで `type_of` は、式から型を*上向きに*求めるだけでした（`1` → `Const[1]`）。でも
エラーを見つけるには、別の問いが要ります：

> このメソッドは `Integer` を欲しがっている。あなたが渡したのは `String`。── **合う？**

これが**受理判定**です。『しくみ』 7 章ではこれを `subtype(a, b)`（a は b の部分型か）として作り、
答えは `true`/`false` の二択でした。

ところが Ruby でこれを二択にすると、すぐ困ります。`type_of` は、わからない式に対して
`Dynamic`（untyped）を返すのでした（Part 1 でそう決めた）。では：

> `Integer` を欲しがっている所に、`untyped` が来た。── 合う？

`true` と答えれば、本当はバグでも見逃すかもしれない。`false` と答えれば、**ちゃんと動く
コードを怒ってしまう**かもしれない。どちらも嘘です。本当の答えは **「わからない（たぶん）」**。

だから `accepts` の答えは三択にします：`:yes`／`:no`／`:maybe`。

---

## 7-2. まず白黒がつく所 ― 確実に合う・確実に合わない

「合う」とは何か。むずかしく考えず、**「渡した値が、欲しがっている型の*箱*に入るか」** と
思ってください。`Integer` という箱には `1` も `2` も入る。`String` の `"x"` は入らない。
ただし `Const[1]` と `Nominal[:Integer]` をそのまま比べても一致しません ― 値そのものの型と
クラスの型は別物だからです。だから比べる前に、値の型を*クラスに丸めて*そろえます（`widen`、
Part 1 の「丸め」がここでも効く）。

```ruby
# Const[1] のような「値そのもの」は、いったんクラスに丸めてから比べる
def widen(t) = t.is_a?(Const) ? Nominal[t.value.class.name.to_sym] : t

def accepts(expected, actual)
  widen(expected) == widen(actual) ? :yes : :no
end
```

```ruby
accepts(Nominal[:Integer], Const[1])      # => :yes  （1 は Integer の箱に入る）
accepts(Nominal[:Integer], Const["x"])    # => :no   （"x" は入らない）
```

この「箱に入るか」が、『しくみ』 7 章の**部分型付け**そのものです（用語は覚えなくて大丈夫。
*小さい箱は大きい箱に入る*、とだけ）。『しくみ』は箱の大小を丁寧に階段状に定義しましたが、私たちは
まず「クラスが同じか」という一番素朴な所から始めます。

> **コラム：「部分型」は継承だけではありません**
>
> 「部分型」は継承（クラスを継いだから箱に入る）だけではありません。Part 6 の「キーが*多い*
> ハッシュは、キーが*少ない*ハッシュの部分型」（`{name:, age:}` は `{name:}` の部分型）には
> 継承関係が一切なく、**構造（持っているキー）が揃っていれば部分型**と*形*だけで決まります ―
> これを**構造的部分型（structural subtyping）**と呼びます。**Rigor／chibirigor は、継承で決まる
> 部分型も構造で決まる部分型も『箱に入るか』という一つの判定にまとめて扱う**。`accepts` はその
> 入口です。（Java の継承／Go・TypeScript の構造的な型との対応は付録
> [a5-2](../appendix/a5-other-languages.md) へ。形式の扱いは後編 Part 2 で。）

- **① 型理論**：値が期待の型に*入る*か＝部分型付け（『しくみ』 7 章）。
- **② Ruby だと**：`1` は `Integer`、`"x"` は `String`。クラスで大まかに判定できる。
- **③ Rigor だと**：`Const[1]` のような細かい型は、比べる前に*クラスに丸めて*から判定する
  （Part 1 の「丸め」がここでも効く）。

---

## 7-3. 「たぶん」が出るとき ― untyped が混ざる

さて、ここからが本題です。`Dynamic`（untyped）が絡んだら、白黒つけません。**`:maybe` を返します。**

```ruby
def accepts(expected, actual)
  return :maybe if expected.is_a?(Dynamic) || actual.is_a?(Dynamic)   # ★追加
  widen(expected) == widen(actual) ? :yes : :no
end
```

```ruby
accepts(Nominal[:Integer], Dynamic.new)   # => :maybe  （わからないものは、わからない）
```

たった 1 行ですが、これが Rigor を「動的言語にやさしい型チェッカー」にしている大黒柱の一本です。
`untyped` は「型を見失った」という印。見失ったものについて「合う」とも「合わない」とも
言い張らない ── それが誠実というものです。

- **① 型理論**：未知（untyped）が混ざると、判定は*片側に倒せない*。
- **② Ruby だと**：型注釈が無いコードは普通。`foo.bar` の戻りなんて誰にもわからない。
- **③ Rigor だと**：わからない所は `:maybe`。これが次の節で「脅かさない」に直結する。

---

### 7-3a. Union が引数に来るとき ― 一番弱い答えを採る

Union（「`Integer` か `String` のどちらか」のような型。Part 4・5 で作りました）が渡される
こともあります。そのときは **メンバを全部 `accepts` にかけて、一番弱い答えを採る**だけ ―
一個でも `:no` なら `:no`、`:no` が無くても `:maybe` があれば `:maybe`、全部 `:yes` なら `:yes`：

```ruby
def accepts(expected, actual)
  return :maybe if expected.is_a?(Dynamic) || actual.is_a?(Dynamic)

  if actual.is_a?(Union)   # 全メンバが通って初めて :yes（一番弱い答え）
    results = actual.members.map { |m| accepts(expected, m) }
    return :no    if results.include?(:no)
    return :maybe if results.include?(:maybe)
    return :yes
  end

  if expected.is_a?(Union)  # どれか一つに合えば :yes（一番強い答え）
    results = expected.members.map { |m| accepts(m, actual) }
    return :yes   if results.include?(:yes)
    return :maybe if results.include?(:maybe)
    return :no
  end

  widen(expected) == widen(actual) ? :yes : :no
end
```

`expected` が Union の場合（`accepts(Integer | String, Integer)` など）は**一番強い答え**を採ります ― どれか一つに合えば `:yes`。「`Integer` か `String` のどちらかを期待」していて、渡されたのが `Integer` なら問題ない、という直感です。

これで `1 + (c ? 1 : 2)` のように **Union が引数に来ても**、中身が全部 `Integer` なら `:yes`＝
黙る（誤検知しない）。`Integer | String` のように `String` が混じれば `:no` で報告します。

> （`(c ? 1 : 2)` のような **括弧つきの式**は、Prism では `ParenthesesNode` という別ノードに
> なります。中身を 1 つ評価して返すよう、`type_of` に 1 行足しておきましょう：
> `when Prism::ParenthesesNode then type_of_body(node.body, scope, diagnostics)`。これを忘れると
> 括弧つきの式が `untyped` に落ちてしまいます。）

- **① 型理論**：Union のメンバ全員が通って初めて `:yes`（一番弱い結論を採る＝union-subtyping）。
- **② Ruby だと**：`c ? 1 : 2` は `Integer | Integer` だから問題ない。`c ? 1 : "a"` は Union になり
  `Integer` を期待する式に渡すと `:no`。
- **③ Rigor だと**：Union への照合でも「分からなければ `:maybe`」。全員 `:yes` のときだけ怒れる。

---

## 7-4. 「たぶん」は罰しない ― この章の山場

`accepts` を `check` で使います。エラーを報告する場所は、**「欲しい型が決まっている所」** だけ。
Part 2 で作った手書きディスパッチ表を思い出してください。`Integer#+` は「`Integer` を 1 つ
欲しい」と書いてありました。**そこが「欲しい型が決まっている所」** です。

```ruby
# Part 2 の表（再掲）。param に「欲しい型」が書いてある
METHODS = {
  [:Integer, :+] => { params: [Nominal[:Integer]], returns: Nominal[:Integer] },
  # ...
}

def check_call(recv_type, name, arg_types, diagnostics)
  sig = METHODS[[class_of(recv_type), name]]
  return Dynamic.new unless sig          # 知らないメソッド → 脅かさない（Part 2 の方針）

  sig[:params].zip(arg_types).each do |want, got|
    case accepts(want, got)
    when :no
      diagnostics << "#{want} が必要ですが #{got} が渡されました"
    when :maybe, :yes
      # 何もしない ← ここが全て
    end
  end
  sig[:returns]
end
```

見てのとおり、**怒るのは `:no` のときだけ**。`:yes` はもちろん、**`:maybe` でも黙ります**。

```ruby
check("1 + 2")        # 文句なし（:yes）
check('1 + "x"')      # ["Integer が必要ですが \"x\" が渡されました"]（:no）
check("1 + foo.bar")  # 文句なし！ foo.bar は untyped → :maybe → 黙る
```

<!-- run: examples/part7.rb -->
```text
Integer | Integer: OK（エラーなし）
Integer が必要ですが 1 | "a" が渡されました
```

ここで、この本で一番大事な一文を置きます。専門用語は要りません：

> **エラーは「欲しい型が決まっている所」でしか出ない。そして untyped が混ざれば `:maybe`、
> `:maybe` は罰しない。だから ── 型のわからない、けれど動いているコードが、怒られることは
> ない。**

『しくみ』 7 章にも、よく似た問題意識のコラムがありました。「健全な型システムが*良いプログラムを
弾く*のは誤検知だ」と。『しくみ』は誤検知を*減らす*方向で工夫しました。Rigor はもう一歩進めて、
**「わからない＝たぶん＝黙る」を仕組みの真ん中に置く**ことで、誤検知をそもそも生みにくく
しています。

そして実は ── Part 1・2 で `+` のときに書いた場当たり的な `integerish?` チェック、あれは
**この `accepts` の手書き版**でした。いまそれを正式な仕組みに置き換えたわけです。

---

## 7-5. 返りは厳しめ、引数は緩め（小さなコラム）

最後に小さな観察を一つ。いま私たちは、

- **引数**を見るときは `accepts` で*緩く*判定しました（`:maybe` を通す）。
- 一方 **戻り型**は `type_of` で*きっちり*求めています（`Integer#+` なら `Integer` と断言）。

この非対称 ── **「返すものは厳しく・正確に、受け取るものは緩く・寛容に」** ── は偶然では
なく、Rigor がわざと守っている作法です。受け取りを厳しくすると、呼ぶ側が無駄な型変換を
書かされて窮屈になる。返しを緩くすると、その値を使う側が損をする。だから逆にする。[^postel]

[^postel]: この「返りは厳密・引数は寛容」という作法には名前も付いていますが、覚えなくて
構いません。なぜこの非対称が正しいのか ― 型理論やオブジェクト指向の置換可能性と、別々の
出発点から同じ規則に至る ― は、後編 Part 2 で正面から確認します。

---

## 7-6. この章のまとめ

足したもの：関数 `accepts`（`:yes`/`:no`/`:maybe` を返す）と、`check` がそれを使って
**`:no` のときだけ怒る**仕組み。コードは `accepts` 本体 4 行＋Union のおまけ数行＋`check` の
差し替えだけ。新しい型キャリアは増えていません。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 7 章 / TAPL 15 章） | 値が期待の型に入るか＝部分型付け |
| ② Ruby/RBS | ダックタイピング、型注釈の無いコード、untyped が普通に混ざる |
| ③ Rigor 実装の問題 | 二択だと誤検知 or 見逃しになる → **三値（`:maybe`）で「わからない」を表に出し、罰しない** |

**続編に送ったもの**（ここで深掘りすると易しさが壊れるため）：

- 箱の大小をきちんと階段状に定義する**本物の部分型**（『しくみ』 7 章の width/depth）。
- **変性**（関数を渡すとき、引数の向きだけ逆になる「反変」）。『しくみ』 7 章の山場。
- `type_of`（求める）と `accepts`（確かめる）の 2 方向、まとめて**双方向型付け**と呼ぶ話。

## 演習

1. `accepts(Nominal[:Integer], Union[[Const[1], Const["x"]]])` が `:no` になることを確かめよ
   （一番弱い答え）。
2. *expected* が Union のとき（`accepts(Integer | String, Integer)`）の挙動を上記の実装で確かめよ。`:yes` を返すのはなぜか、`:no` とはならない理由とあわせて説明せよ。
3. `:maybe` が出るのに `check` が黙る例を 1 つ作り、「`:maybe` は罰しない」を確かめよ。

---

**次章予告（Part 8）**：手書きの `METHODS` 表を、本物の **RBS** から引くように差し替えます。
「型は別ファイルに書く」という Ruby/RBS の世界観に初めて触れます。
