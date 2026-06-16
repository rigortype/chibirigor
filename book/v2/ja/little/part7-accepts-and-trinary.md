---
title: Part 7　受理判定と「たぶん」
description: "「この型を渡して合う？」を三値（`:yes`／`:no`／`:maybe`）で判定する `accepts` を作る。"
sidebar:
  order: 8
---

# The Little chibirigor Part 7　受理判定と「たぶん」

この章のゴールは、「この型、ここに渡して合う？」を判定する`accepts`を作ることです。
ただし答えは「はい」か「いいえ」の二択ではなく、`:yes`／`:no`／`:maybe`の三択にします。
この「`:maybe`」が、Rigorがやさしくいられる最大の理由です。

> 『しくみ』では7章「部分型付け」（TAPL 15章「部分型付け」）が対応します。
> あの本は「合う／合わない」を`true`/`false`の二択で答えました。
> 私たちはそこに一つ足します。

---

## 7-1. 「合う？」という新しい問い

これまで`type_of`は、式から型を上向きに求めるだけでした（`1` → `Const[1]`）。
でもエラーを見つけるには、別の問いが要ります。

> このメソッドは`Integer`を欲しがっている。あなたが渡したのは`String`だ。これは合う？

これが**受理判定**です。
『しくみ』 7章ではこれを`subtype(a, b)`（aはbの部分型か）として作り、答えは`true`/`false`の二択でした。

ところがRubyでこれを二択にすると、すぐ困ります。
`type_of`は、わからない式に対して`Dynamic`（untyped）を返すのでした（Part 1でそう決めた）。
では：

> `Integer`を欲しがっている所に、`untyped`が来た。これは合う？

`true`と答えれば、本当はバグでも見逃すかもしれません。
`false`と答えれば、ちゃんと動くコードを怒ってしまうかもしれません。
どちらも嘘です。
本当の答えは「わからない（たぶん）」です。

だから`accepts`の答えは三択にします。`:yes`／`:no`／`:maybe`です。

---

## 7-2. まず白黒がつく所（確実に合うか、確実に合わないか）

「合う」とは何か。むずかしく考えず、「渡した値が、欲しがっている型の*箱*に入るか」と思ってください。
`Integer`という箱には`1`も`2`も入ります。
`String`の`"x"`は入りません。
ただし`Const[1]`と`Nominal[:Integer]`をそのまま比べても一致しません。
値そのものの型とクラスの型は別物だからです。
だから比べる前に、値の型をクラスに丸めてそろえます（`widen`。Part 1の「丸め」がここでも効きます）。

```ruby
# Const[1] のような「値そのもの」は、いったんクラスに丸めてから比べる
def widen(t) = t.is_a?(Type::Const) ? Type::Nominal[t.value.class.name.to_sym] : t

def accepts(expected, actual)
  widen(expected) == widen(actual) ? :yes : :no
end
```

```ruby
accepts(Type::Nominal[:Integer], Type::Const[1])      # => :yes  （1 は Integer の箱に入る）
accepts(Type::Nominal[:Integer], Type::Const["x"])    # => :no   （"x" は入らない）
```

この「箱に入るか」が、『しくみ』 7章の部分型付けそのものです（用語は覚えなくて大丈夫。*小さい箱は大きい箱に入る*、とだけ）。
『しくみ』は箱の大小を丁寧に階段状に定義しましたが、私たちはまず「クラスが同じか」という一番素朴な所から始めます。

> [!NOTE]
> **「部分型」は継承だけではありません**
>
> 「部分型」は継承（クラスを継いだから箱に入る）だけではありません。
> Part 6の「キーが*多い*ハッシュは、キーが*少ない*ハッシュの部分型」（`{name:, age:}`は`{name:}`の部分型）には継承関係が一切なく、構造（持っているキー）が揃っていれば部分型と*形*だけで決まります。
> これを**構造的部分型（structural subtyping）**と呼びます。
> **Rigor／chibirigorは、継承で決まる部分型も構造で決まる部分型も「箱に入るか」という一つの判定にまとめて扱います。**
> `accepts`はその入口です（Javaの継承／GoやTypeScriptの構造的な型との対応は付録[a5-2](../appendix/a5-other-languages.md)へ。形式の扱いは後編Part 2で）。

この章の三つの視点（パースペクティブ）：

- **① 型理論**：値が期待の型に*入る*かが部分型付けです（『しくみ』 7章）。
- **② Rubyだと**：`1`は`Integer`、`"x"`は`String`。クラスで大まかに判定できます。
- **③ Rigorだと**：`Const[1]`のような細かい型は、比べる前にクラスに丸めてから判定します（Part 1の「丸め」がここでも効きます）。

---

## 7-3. 「たぶん」が出るとき（untypedが混ざる）

さて、ここからが本題です。
`Dynamic`（untyped）が絡んだら、白黒つけません。**`:maybe`を返します。**

```ruby
def accepts(expected, actual)
  return :maybe if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)   # ★追加
  widen(expected) == widen(actual) ? :yes : :no
end
```

```ruby
accepts(Type::Nominal[:Integer], Type::Dynamic.new)   # => :maybe  （わからないものは、わからない）
```

たった1行ですが、これがRigorを「動的言語にやさしい型チェッカー」にしている大黒柱の一本です。
`untyped`は「型を見失った」という印です。
見失ったものについて「合う」とも「合わない」とも言い張らない。
それが誠実というものです。

- **① 型理論**：未知（untyped）が混ざると、判定は片側に倒せません。
- **② Rubyだと**：型注釈が無いコードは普通です。`foo.bar`の戻りなんて誰にもわかりません。
- **③ Rigorだと**：わからない所は`:maybe`にします。これが次の節で「脅かさない」に直結します。

---

### 7-3a. Unionが引数に来るとき（一番弱い答えを採る）

Union（「`Integer`か`String`のどちらか」のような型。Part 4、5で作りました）が渡されることもあります。
渡す側（actual）がUnionのときは、実際にどのメンバの値が来るか分からないので、全メンバが通って初めて安全です。
だからメンバを全部`accepts`にかけて、一番弱い答えを採るだけです。
一個でも`:no`なら`:no`、`:no`が無くても`:maybe`があれば`:maybe`、全部`:yes`なら`:yes`です。

```ruby
def accepts(expected, actual)
  return :maybe if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)

  if actual.is_a?(Type::Union)   # 全メンバが通って初めて :yes（一番弱い答え）
    results = actual.members.map { |m| accepts(expected, m) }
    return :no    if results.include?(:no)
    return :maybe if results.include?(:maybe)
    return :yes
  end

  if expected.is_a?(Type::Union)  # どれか一つに合えば :yes（一番強い答え）
    results = expected.members.map { |m| accepts(m, actual) }
    return :yes   if results.include?(:yes)
    return :maybe if results.include?(:maybe)
    return :no
  end

  widen(expected) == widen(actual) ? :yes : :no
end
```

`expected`がUnionの場合（`accepts(Integer | String, Integer)`など）は一番強い答えを採ります。
どれか一つに合えば`:yes`です。
「`Integer`か`String`のどちらかを期待」していて、渡されたのが`Integer`なら問題ない、という直感です。

これで`1 + (c ? 1 : 2)`のようにUnionが引数に来ても、中身が全部`Integer`なら`:yes`で黙ります（誤検知しない）。
`Integer | String`のように`String`が混じれば`:no`で報告します。

> [!NOTE]
> （`(c ? 1 : 2)`のような括弧つきの式は、Prismでは`ParenthesesNode`という別ノードになります。
> 中身を1つ評価して返すよう、`type_of`に1行足しておきましょう：
> `when Prism::ParenthesesNode then type_of_body(node.body, scope, diagnostics)`。
> これを忘れると括弧つきの式が`untyped`に落ちてしまいます。）

- **① 型理論**：Unionのメンバ全員が通って初めて`:yes`です（一番弱い結論を採る、union-subtyping）。
- **② Rubyだと**：`c ? 1 : 2`は`Integer | Integer`だから問題ありません。`c ? 1 : "a"`はUnionになり、`Integer`を期待する式に渡すと`:no`になります。
- **③ Rigorだと**：Unionへの照合でも「分からなければ`:maybe`」です。全員`:yes`のときだけ怒れます。

![図7-1　acceptsの三値判定（untypedは :maybe、Unionは一番弱い答え）](../figures/svg/little-7-1.svg)
> ▼ 図7-1　`accepts`の三値判定（untypedは`:maybe`、Unionは一番弱い答え）

---

## 7-4. 「たぶん」は罰しない（この章の山場）

`accepts`を`check`で使います。
エラーを報告する場所は、「欲しい型が決まっている所」だけです。
Part 2で作った手書きディスパッチ表を思い出してください。
`Integer#+`は「`Integer`を1つ欲しい」と書いてありました。
そこが「欲しい型が決まっている所」です。

```ruby
# Part 2 の表（再掲）。param に「欲しい型」が書いてある
METHODS = {
  [:Integer, :+] => { params: [Type::Nominal[:Integer]], returns: Type::Nominal[:Integer] },
  # ...
}

def check_call(recv_type, name, arg_types, diagnostics)
  sig = METHODS[[class_of(recv_type), name]]
  return Type::Dynamic.new unless sig          # 知らないメソッド → 脅かさない（Part 2 の方針）

  sig[:params].zip(arg_types).each do |want, got|
    case accepts(want, got)
    when :no
      diagnostics << "expected #{want} but got #{got}"
    when :maybe, :yes
      # 何もしない ← ここが全て
    end
  end
  sig[:returns]
end
```

見てのとおり、怒るのは`:no`のときだけです。
`:yes`はもちろん、**`:maybe`でも黙ります**。

```ruby
check("1 + 2")        # 文句なし（:yes）
check('1 + "x"')      # ["expected Integer but got \"x\""]（:no）
check("1 + foo.bar")  # 文句なし！ foo.bar は untyped → :maybe → 黙る
```

<!-- run: examples/part7.rb -->
```text
Integer | Integer: OK (no errors)
expected Integer but got 1 | "a"
```

ここで、この本で一番大事な一文を置きます。専門用語は要りません。

> [!IMPORTANT]
> エラーは「欲しい型が決まっている所」でしか出ない。
> そしてuntypedが混ざれば`:maybe`、`:maybe`は罰しない。
> だから、型のわからない、けれど動いているコードが、怒られることはない。

『しくみ』 7章にも、よく似た問題意識のコラムがありました。
「健全な型システムが*良いプログラムを弾く*のは誤検知だ」と。
『しくみ』は誤検知を*減らす*方向で工夫しました。
Rigorはもう一歩進めて、「わからない＝たぶん＝黙る」を仕組みの真ん中に置くことで、誤検知をそもそも生みにくくしています。

そして実は、Part 1、2で`+`のときに書いた場当たり的な`integerish?`チェック、あれはこの`accepts`の手書き版でした。
いまそれを正式な仕組みに置き換えたわけです。

---

## 7-5. 返りは厳しめ、引数は緩め（小さなコラム）

最後に小さな観察を一つ。いま私たちは、

- **引数**を見るときは`accepts`で緩く判定しました（`:maybe`を通す）。
- 一方、**戻り型**は`type_of`できっちり求めています（`Integer#+`なら`Integer`と断言）。

この非対称、「返すものは厳しく正確に、受け取るものは緩く寛容に」は偶然ではなく、Rigorがわざと守っている作法です。
受け取りを厳しくすると、呼ぶ側が無駄な型変換を書かされて窮屈になります。
返しを緩くすると、その値を使う側が損をします。
だから逆にするのです。[^postel]

[^postel]: この「返りは厳密、引数は寛容」という作法には名前も付いていますが、覚えなくて構いません。なぜこの非対称が正しいのか（型理論やオブジェクト指向の置換可能性と、別々の出発点から同じ規則に至ること）は、後編Part 2で確認します。

---

## 7-6. この章のまとめ

足したものは、関数`accepts`（`:yes`/`:no`/`:maybe`を返す）と、`check`がそれを使って`:no`のときだけ怒る仕組みです。
コードは`accepts`本体4行＋Unionのおまけ数行＋`check`の差し替えだけです。
新しい型キャリアは増えていません。

この章の三つの視点：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 7 章 / TAPL 15 章） | 値が期待の型に入るか＝部分型付け |
| ② Ruby/RBS | ダックタイピング、型注釈の無いコード、untyped が普通に混ざる |
| ③ Rigor 実装の問題 | 二択だと誤検知 or 見逃しになる → **三値（`:maybe`）で「わからない」を表に出し、罰しない** |

**続編に送ったもの**（ここで扱うと易しさが壊れるため）：

- 箱の大小をきちんと階段状に定義する本物の部分型（『しくみ』 7章のwidth/depth）。
- **変性**（関数を渡すとき、引数の向きだけ逆になる「反変」）。『しくみ』 7章の山場。
- `type_of`（求める）と`accepts`（確かめる）の2方向をまとめて「双方向型付け」と呼ぶ話。

## 演習

1. `accepts(Nominal[:Integer], Union[[Const[1], Const["x"]]])`が`:no`になることを確かめよ（一番弱い答え）。
2. *expected*がUnionのとき（`accepts(Integer | String, Integer)`）の挙動を上記の実装で確かめよ。`:yes`を返すのはなぜか、`:no`とはならない理由とあわせて説明せよ。
3. `:maybe`が出るのに`check`が黙る例を1つ作り、「`:maybe`は罰しない」を確かめよ。

---

**次章予告（Part 8）**：手書きの`METHODS`表を、本物の**RBS**から引くように差し替えます。
「型は別ファイルに書く」というRuby/RBSの世界観に初めて触れます。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part7/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part7/lib)
