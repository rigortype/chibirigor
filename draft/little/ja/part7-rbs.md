# 【試し書き】The Little chibirigor Part 7 ― RBS ひとさじ

> Part 2 の手書きディスパッチ表を、「別ファイルに書いた型」＝ RBS から生成する表に差し替える章。
> 実装は `lib/chibirigor/rbs.rb` に反映済み。コードは実 Prism/Ruby で動作確認済み。

この章のゴール：**手書きの `METHODS` 表を、RBS テキストから読み込んだ表に差し替える。**
差し替えても診断は 1 つも変わらない ― それが「正しく差し替えられた」証拠です。

> 『しくみ』9 章「ジェネリクス」（TAPL 23 章「全称型／System F」）の*型代入*が遠い親戚ですが、ここで本当に
> 出会うのは Ruby/RBS 固有の世界観 ―「**型はコードではなく、別ファイル（.rbs）に書く**」です。

---

## 7-1. 型は「別ファイル」に書く ― RBS

ここまで、メソッドの型は Ruby のコード（`METHODS` 表）に直接書いてきました。でも Ruby 本来の
やり方は違います。Ruby のコードには型注釈を**書きません**。代わりに、型は **RBS** という
*別ファイル*（`.rbs`）に書きます：

```rbs
class Integer
  def +: (Integer) -> Integer
  def to_s: () -> String
end
```

これが Ruby/RBS の世界観です。「コードは型のことを知らない。型は外から与える」。Rigor は
この RBS を**正**として読み、その上にさらに精度を足していく（RBS のスーパーセット）。

- **① 型理論**：宣言された型を引いて使う（『しくみ』 9 章の型代入の遠縁）。
- **② Ruby だと**：コードに型注釈は無い。型は `.rbs` に別書き。
- **③ Rigor だと**：RBS を真実の源として読む。手書き表は、その RBS の*ミニ版*だった。

---

## 7-2. ごく小さな RBS を読む

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

## 7-3. 手書き表を RBS 由来に差し替える

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

内容が手書き表と同じなので、差し替えても **診断は 1 つも変わりません**。Part 1〜6 のテストが
全て緑のまま、というのがその証拠（＝ふるまいを変えずに土台だけ入れ替える、安全なリファクタ）。

```console
$ ruby test/test_part1.rb  # … 緑
$ ruby test/test_part6.rb  # … 緑（表の出どころが変わっただけ）
```

- **① 型理論**：型の出どころを宣言（RBS）に一元化。
- **② Ruby だと**：`.rbs` が型の単一の源。
- **③ Rigor だと**：手書き表 → RBS 由来へ。ふるまい不変（differ 置換）。

---

## 7-4. この章のまとめ

足したもの：`Rbs.load`（ごく小さな RBS リーダー）と `Rbs::CORE`。`Dispatch::METHODS` の
*出どころ*だけが変わり、ふるまいは変わりませんでした。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 9 章 / TAPL 23 章） | 宣言された型を引いて使う（型代入の遠縁） |
| ② Ruby/RBS | 型はコードに書かず、別ファイル `.rbs` に書く |
| ③ Rigor 実装の問題 | RBS を真実の源に。手書き表 → RBS 由来へ differ 置換 |

**続編／後の Part に送ったもの**：

- 本物の `rbs` gem を使った完全な RBS 読み込み（union 型・optional・ブロック・ジェネリクス）。
- **型変数の置換**（`Array[Elem]` に `String` を入れて `Array[String]`）。『しくみ』 9 章の `subst` の
  正道版。
- 継承チェーン・モジュール mixin をたどったメソッド解決、`erasure`（型を RBS に書き戻す）。

**次章予告（Part 8）**：`annotate` を仕上げます。推論したシグネチャを RBS 風に出力し、
**`untyped` がどこに出るか＝推論が型を見失った場所**を可視化します（`sig-gen` の芽）。

---

> **検証メモ**
> - 複雑さ予算：新規は `Rbs.load`（正規表現 2 本）と `CORE` 文字列のみ。本物の rbs gem は
>   入れない（依存ゼロ維持・環境の native ext 非互換も回避）。○
> - differ 置換の証拠：Part 1〜6 のテストが全て緑＝表の出どころを変えてもふるまい不変。○
> - 三題噺：③「手書き → RBS 由来」が②（型は別ファイル）の素直な実体化として出た。○
> - 設計との差：当初は本物の rbs gem を想定していたが、環境の native 拡張が Ruby 4.0 と非互換
>   だったため自前ミニリーダーに切替。むしろ chibivue 流（最小版を自分で作る）に合致。
