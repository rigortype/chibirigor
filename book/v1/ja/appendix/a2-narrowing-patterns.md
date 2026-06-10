---
title: 付録 a2 ― ナローイング・パターン集
description: 後編 Part 6（完全な FactStore）の本筋から括り出した、個別の絞り込みパターンを「いつ事実が生まれ、いつ消えるか」で一覧する参照付録。
sidebar:
  order: 22
---

# 付録 a2 ― ナローイング・パターン集

> **この付録は後編向けの予習です。前編だけを読む方は飛ばして構いません**（前編は付録 a2 に
> 依存せず完結します）。
>
> **本筋への戻りポインタ**：この付録は後編 [Part 6 ― 完全な FactStore](../seasoned/part6-fact-store.md)
> の本筋（6 バケツ・stability・join）から括り出した、個別のナローイング・パターン集です。
> Part 6 を読んでいて「この絞り込みは具体的にどう動くのか」と思ったら、ここを引いてください。
> 各パターンを **いつ事実が生まれ／いつ消えるか** で簡潔にまとめます。前提となる素朴な
> 絞り込み（`narrow`／`if x.nil?`／`is_a?`／再代入リセット）は前編
> [Part 5 ― ナローイング](../little/part5-narrowing.md)、用語は
> [用語集](../glossary.md) を参照。

FactStore は型環境を **フロー感応な事実の集合** へ一般化したものです（後編 Part 6）。
ここで言う「事実」は、その地点で成り立っている命題 ―「`x` は nil でない」「`arr` は空でない」
など ― で、対象の種類により 6 つのバケツ（`local_binding`／`captured_local`／`object_content`／
`global_storage`／`relational`／`dynamic_origin`）に分けて持ちます。本付録は、その事実が
**どう積まれ・どう消えるか** の個別パターンを 1 箇所に集めたものです。

---

## a2-1. `&&` で事実が積み上がり、`||` で削れる

`&&` 演算子は左から右へ **逐次評価** されるため、FactStore の事実も左から順に積み上がります。

```ruby
if x.is_a?(Integer) && x > 0
  # ここでは local_binding に 2 つの事実が積まれている
  #   1. x is_a? Integer   （is_a? ナローイング）
  #   2. x > 0             （比較述語）
  # 合成されると x : positive-int と読める
end
```

左側の `is_a?(Integer)` が通過した時点で `x` の型が `Integer` に絞られ、その状態で右側の
`x > 0` が評価されます。「`Integer` かつ `> 0`」が積み重なるので、Rigor はこれを
**`positive-int` リファインメント**（→ a2-6）として扱えます。

逆に `||` チェーンは「どちらか一方が成立した場合」なので、合流点で join（共通事実のみ残す。
後編 Part 6 の join）が走り、片方にしかない事実は消えます。

| 演算子 | 事実への作用 | いつ生まれる／消えるか |
|---|---|---|
| `&&` | *足す* | 左から逐次に積む。左が通った状態で右を評価 |
| `\|\|` | *削る* | 合流で join。両辺で共通の事実だけ残る |

`&&` が足し、`||` が削る ― これが FactStore が左右を対称に扱わない理由です。

---

## a2-2. 正規表現の名前付きキャプチャがマッチ後に String を生む

Ruby の `=~` と名前付きキャプチャ（`(?<name>...)`）は、**マッチ成功時にローカル変数へ
`String` を束縛する** という、他の言語にほぼ無い独自の挙動を持ちます。

```ruby
if /(?<year>\d{4})-(?<month>\d{2})/ =~ str
  # year, month が String として束縛されている
  year.upcase   # OK（year は String）
end
```

Rigor はこれを **名前付きキャプチャ・ナローイング** として認識します。`if` ブロック内で
`year` と `month` の `local_binding` に `String` 事実を追加します。

- **事実が生まれる**：`=~` の左辺が正規表現リテラルかつ名前付きキャプチャを含むとき、
  マッチ成功側（`if` ブロック内）で、キャプチャ名と同名のローカルに `String` 事実が入る。
- **事実が消える（届かない）**：マッチ失敗なら束縛は `nil`。したがって `if` の外では
  どちらも `String | nil` のまま。

Prism では `=~` の左辺が `RegexpNode` かつ名前付きキャプチャを含む場合、Rigor は捕獲グループ名を
読み出して FactStore に直接事実を挿入します。`is_a?` の型述語や `nil?` の nil ガードと同じ
仕組みですが、**変数名が正規表現の本文から来る** 点が特殊です。

| パターン | ナローイング対象 | 追加される事実 |
|---|---|---|
| `is_a?(String)` | 左辺の変数 | `String` |
| `nil?` 否定 | 左辺の変数 | `non-nil` |
| `=~` 名前付きキャプチャ | キャプチャ名の変数 | `String` |

---

## a2-3. エスケープするブロックで事実が消える

`each` や `map` のブロックは即時呼び出しなので、ナローイングの事実はブロック終了後まで
ほぼ保持できます。問題は **エスケープするブロック** ― ブロックが呼び出し元の外へ「脱出」する
場合です。

```ruby
if x.is_a?(Integer)
  # ここで x の local_binding に "is Integer" が入る
  Thread.new { x.some_integer_method }   # ← x を捕獲して別スレッドへ
end
# Thread がいつ走るかは不明 → x の narrowing を保持し続けるのは危険
```

`Thread.new` に渡したブロックは *任意のタイミング* で動きます。その時点で `x` が再代入されて
いたり、すでに別の型になっている可能性を排除できません。

- **事実が消える**：FactStore はこの「エスケープ」を検知すると、そのブロックが捕獲した変数
  すべての `captured_local` 事実を **保守的に無効化** します。
- **対象パターン**：`Thread.new`・`define_method`・`Proc.new`／`Fiber.new` など、
  「ブロックをオブジェクトとして保存する／後で呼ぶ」パターン。

「即時呼び出しか」「後で呼ばれるか」の判定は、Rigor が RBS のシグネチャアノテーション
（`&block` が `Proc` か `yield` か、等）から推定します。判断できない場合はエスケープと見なし、
**迷ったら消す**（緩める側に倒す）方針を採ります。

> なお、ブロックが **外側のローカルを書き換える** 場合（`x = nil; [1,2,3].each { |i| x = i }`）も、
> その `captured_local` 事実は無効化されます ― ブロックが `x` に再代入し得るなら、ナローイングの
> 事実は危ういからです。これは後編 Part 6 のクロージャ捕獲の本筋で扱います。

---

## a2-4. ivar の型は「すべての代入の union」

`object_content` バケツには ivar（インスタンス変数）の型が入ります。Rigor はクラス内の
`@x` への代入を **すべて収集** し、その型の union を `@x` の型とします。

```ruby
class Foo
  def initialize
    @x = 1          # Const[1]
  end

  def reset
    @x = nil        # Const[nil]
  end

  def use
    @x              # => Integer | nil （すべての代入の union）
  end
end
```

`@x` に書く場所が `initialize` だけなら `Integer`、`reset` が加わると `Integer | nil` に
なります。「どこかで `nil` が代入され得るなら、どこで読んでも `nil` を含む」 ― これは
保守的ですが、ivar の可視性（どのメソッドから書けるか）がファイルを跨ぐと完全には追えない
ため、**すべての可視な代入の union** が安全な近似です。

- **事実が生まれる**：クラス内の `@x` への各代入から、`object_content` バケツに型が集まり、
  読み出し点ではその union になる。
- **事実が消える**：`@x` を `nil` で初期化してすぐ設定するパターンでは、読み出し前に
  `@x.nil?` で分岐して `non-nil` 事実を足すのが定石（`nil?` ガード）。

---

## a2-5. 再代入でナローイングがリセットされる

前編 Part 5 の「再代入リセット」の一般化です。変数への **再代入** は、それ以前にその変数へ
積んだ事実をすべてリセットします。

```ruby
x = find_user        # x : User | nil
if x
  # x : User （non-nil 事実が積まれた）
  x = find_other     # ★ 再代入 ― ここで x の local_binding 事実は全て消える
  # x : User | nil （find_other の戻り型に戻り、絞り込みの記憶は無い）
end
```

事実は「変数名」ではなく「そのスコープ位置で確定した事実」に結びついています。`x = something_else`
を書いた瞬間、`x` に関する narrowing の記憶は全て消え、新しい右辺の型から再出発します。

- **事実が消える**：`x = …` は `x` の `local_binding` 事実を消す（後編 Part 6 の stability で言う
  「再代入による無効化」）。
- バケツごとに無効化のタイミングが違う点に注意：再代入は `local_binding` を、メソッド呼び出し
  （`obj.mutate!`）は `obj` の `object_content` を疑う、というように対象を絞って消します。

---

## a2-6. refinement carrier はなぜ Difference 型（集合差）か

`non-empty-string`・`positive-int`・`literal-string` のような、**述語で絞り込まれた型** を
Rigor は **refinement carrier（細粒度キャリア）** と呼びます。`unless s.empty?` を通った後の
`s` は `non-empty-string` になる、というように、フロー事実から自動的に生まれます
（後編 Part 6 で `payload` が運ぶ値）。

これは前編 Part 1 の `Const[42]`（「値が 42」という超精密な型）とは別概念です。`Const` は
*特定の 1 つの値*、refinement carrier は *述語を満たす値の集合* を表します。

**なぜ「集合差（Difference 型）」なのか。** `non-empty-string` は内部的に `String - ""`、
つまり「`String` の値の集合から空文字列 `""` を差し引いた集合」として実装されます
（用語集「`Difference` 型」）。「空でない」とは「全文字列の集合から、空文字列という値を
*除いた* 残り」に他なりません。名前は付いていても、実体は union（合併）・intersection（交差）と
並ぶ集合論的型演算の一つ ― **集合差（set difference）** です。chibirigor 本体では扱いませんが、
この種の carrier の「なぜその名か」の答えはここにあります。

> **ただし refinement carrier すべてが集合差ではありません。** 実 Rigor は二層構成（ADR-3）で、
> `non-empty-string` のような**点除去**だけが `Difference`。`lowercase-string`/`numeric-string` の
> ような**述語部分集合**は別キャリア `Refined`、`Integer[1..10]` のような**範囲整数**は
> `IntegerRange` で表します（値の表記は `Integer[1..10]`。`int<min,max>` はディレクティブ語彙や内部表示で使う）。
> 下表の carrier はこの 3 種が混ざっています。

- **事実が生まれる**：`unless s.empty?`／`if n > 0`／`&&` チェーン（a2-1）など、述語ガードを
  通過した枝で、その変数の `payload` がより精密な refinement carrier になる。
- **事実が消える**：再代入（a2-5）・エスケープ（a2-3）など、対象の事実を無効化する操作で
  元の粗い型（`String`・`Integer`）に戻る。

### PHPStan 語彙対応表

Rigor の主な組み込み refinement carrier と、PHP のチェッカー PHPStan の対応語彙です。同じ述語を
異なる言語チェッカーが同じ名前で表現することで学習コストを下げる、意図的な命名対応です
（用語集「refinement carrier」より再掲）。

| Rigor | PHPStan | 意味 |
|---|---|---|
| `non-empty-string` | `non-empty-string` | 空でない文字列 |
| `numeric-string` | `numeric-string` | 数値に変換できる文字列（`"42"` 等） |
| `literal-string` | `literal-string` | ソースコードリテラルのみから構成された文字列 |
| `non-empty-literal-string` | ― | 上 2 つの交差 |
| `positive-int` | `positive-int` | 0 より大きい整数 |
| `negative-int` | `negative-int` | 0 より小さい整数 |
| `non-zero-int` | `non-zero-int` | 0 でない整数 |
| `non-negative-int` | `non-negative-int` | 0 以上の整数 |
| `Integer[1..9]`（`IntegerRange`） | `int<m, n>` | 範囲指定の整数（例：`Integer[1..9]`） |
| `non-empty-array` | `non-empty-array<T>` | 要素が 1 つ以上の配列 |
| `non-empty-hash` | ― | キーが 1 つ以上のハッシュ |
| `lowercase-string` | `lowercase-string` | ASCII 小文字のみの文字列 |
| `uppercase-string` | ― | ASCII 大文字のみの文字列 |

### a2-6x. 発展：chibirigor の `Tuple` は事実上の `non-empty-array`

上表の `non-empty-array`（要素が 1 つ以上の配列）は、chibirigor にも**構造として**現れています ―
新しいキャリアを足さずに、です。前編 Part 5 の `Tuple`（位置ごとに型を覚える配列）は、要素が
1 つでもあれば「空でない」ことが**形から確定**しているからです。generics 5a の要素型の読み
（後編 Part 3「3-6x」）と合わせると、こう出ます：

```console
$ printf '[1, 2].first\n[].first\n' | ruby exe/chibirigor annotate /dev/stdin
1: Integer
2: untyped
```

`[1,2].first` は `Integer`（**nil を含まない**）。一般の `Array[Elem]#first` なら「要素が無いかも
しれない」ので `Elem | nil` になるところを、chibirigor は `Tuple`（＝空でないと分かっている形）
からの読みなので `nil` を混ぜません ― これは実 Rigor の `non-empty-array` リファインメントが
`first` を `Elem`（非 nil）に絞るのと**効きは同じ**です（出自は違う ― 後述）。逆に空配列 `[]` は
要素も非空性も不明なので
`first` は `untyped`（埋まらねば untyped）。

ただし chibirigor のこれは**専用キャリアではなく `Tuple` の副産物**です。実 Rigor は
`unless arr.empty?` のようなフロー事実から `non-empty-array` carrier を*生成*し、再代入や
エスケープで*消し*ます（上の「事実が生まれる／消える」）。chibirigor の `Tuple` は
リテラルの形から静的に空でなさを持つだけ ― 述語ガードで生まれる動的な refinement carrier は
続編に送ります。

---

## まとめ

| パターン | 事実が生まれる | 事実が消える |
|---|---|---|
| `&&` / `\|\|`（a2-1） | `&&` が左から逐次に積む | `\|\|` は合流の join で共通だけ残す |
| 正規表現キャプチャ（a2-2） | マッチ成功側でキャプチャ名に `String` | マッチ失敗側／ブロック外は `String \| nil` |
| エスケープブロック（a2-3） | 即時呼びブロックは事実を保持 | エスケープ検知で `captured_local` を無効化 |
| ivar union（a2-4） | 全代入の union を `object_content` に | `nil?` ガードで `non-nil` を足すまで `nil` を含む |
| 再代入リセット（a2-5） | ガード通過で `local_binding` に事実 | `x = …` で `x` の事実を全消去 |
| refinement carrier（a2-6） | 述語ガード通過で精密な `payload` | 再代入・エスケープで粗い型に戻る |

いずれのパターンも、後編 Part 6 の芯 ―「**迷ったら消す**（緩める側に倒す）」「narrowing は
事実を *足す* だけ」「FactStore は不変・フロー感応」― の上で動いています。本筋は
[後編 Part 6](../seasoned/part6-fact-store.md) に戻って確認してください。
