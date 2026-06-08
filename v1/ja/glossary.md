# 用語集

本文で「実はこれ◯◯と呼ばれます」と後出しした用語を、引けるようにまとめます。
（前編＝The Little、後編＝The Seasoned。初出の章を併記。）

## 型と値

- **型キャリア（type carrier）**〔前編 P1〕… 型を表す Ruby オブジェクト。`Const`/`Nominal`/
  `Dynamic`/`Union`/`HashShape`/`Tuple` など。
- **`Const`（リテラル型）**〔前編 P1〕… 「その値そのもの」を表す型。例：`Const[1]`。
- **`Nominal`（名前的型）**〔前編 P1〕… 名前付きクラスを表す型。例：`Nominal[:Integer]`。
- **`Dynamic`／untyped**〔前編 P1〕… 「型を見失った」印。gradual の要。他言語対応表は前編 P1 コラム参照。
- **`Union`（合併型）**〔前編 P4〕… 「`A` か `B` のどちらか」。例：`Integer | String`。
- **`HashShape`（レコード型）**〔前編 P5〕… キーごとの型を覚えるハッシュの型。Hack の `shape(...)` を起点とし PHPStan/Psalm を経て Rigor に至る設計（前編 P5 コラム参照）。
- **`partial_of[T]`**〔Rigor 固有〕… `T` と同じキー構造を持つが、各キーの値型は変えない「部分ハッシュ」型。`partial_of[{name: String, age: Integer}]` は `{name: String}` や `{age: Integer}` などを含む。重要な点は**値型を `nil` に広げない**こと ― 値は「省略されうる（キーが無くてもよい）」だけで、「あれば確実に `T` の値型を持つ」。`Partial<T>` に相当する TypeScript との違いは、TS の `Partial<T>` が全キーを `T | undefined` にするのに対し、Rigor の `partial_of[T]` は省略されたキーには触れず存在するキーの値型を保ちます。
- **丸め／正規化（normalization）**〔前編 P1〕… 細かい型（`Const[3]`）を大ざっぐな型
  （`Integer`）に戻すこと。TAPL 12 章。
- **`Difference` 型**〔Rigor 内部〕… 「`A` から `B` を除いた値の集合」を表す型キャリア。
  `non-empty-string` は内部的に `String - ""` として実装される（`String` の値の集合から
  空文字列 `""` を差し引いた集合）。名前は付いていても、実体は**集合差（set difference）**。
  union（合併）・intersection（交差）と並ぶ集合論的型演算の一つ。chibirigor では扱わないが、
  refinement carrier の「なぜその名か」の答えはここにある。
- **refinement carrier（細粒度キャリア）**〔後編 P5 / Rigor 固有〕… 「空でない・正の値・
  リテラル由来」といった*述語で絞り込まれた型*。`Nominal` のサブクラスではなく、フロー事実から
  自動的に生まれる。`unless s.empty?` を通った後の `s` は `non-empty-string` になる。
  「値そのもの」の `Const[42]` とは別概念 ― `Const` は特定の値、refinement carrier は
  *述語を満たす値の集合*。

  Rigor の主な組み込み refinement carrier と、PHPStan の対応語彙：

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
  | `int<m, n>` | `int<m, n>` | 範囲指定の整数（例：`int<1, 9>`） |
  | `non-empty-array` | `non-empty-array<T>` | 要素が 1 つ以上の配列 |
  | `non-empty-hash` | ― | キーが 1 つ以上のハッシュ |
  | `lowercase-string` | ― | ASCII 小文字のみの文字列 |
  | `uppercase-string` | ― | ASCII 大文字のみの文字列 |

  PHPStan との語彙の対応は意図的で、「同じ述語を異なる言語チェッカーが同じ名前で表現する」
  ことで学習コストを下げる設計です（後編 P5 §5-1 参照）。

- **`literal-string`**〔Rigor 固有〕… 文字列リテラルおよびリテラル同士の演算から
  *のみ*構成された文字列を表す refinement carrier。「ユーザー入力が混入していない」ことを
  型レベルで証明できるため、SQL インジェクション・XSS 対策の審査に使われる（Python の
  `LiteralString`（PEP 675）と同じ役割）。Rigor では文字列補間 `"#{a}#{b}"` で両辺が
  `literal-string` なら結果も `literal-string` として伝播する。

## 推論と型チェック

- **合成（synthesize, `⇒`）**〔後編 P1〕… 式から型を上向きに**求める**こと。前編の `type_of`。
- **照合（check, `⇐`）**〔後編 P1〕… 期待型に対して式が**合うか確かめる**こと。前編の `accepts`。
- **双方向型付け（bidirectional typing）**〔後編 P1〕… 合成と照合の 2 方向に分ける枠組み。
- **三値（trinary）`:yes`/`:no`/`:maybe`**〔前編 P6〕… 受理判定の答え。`:maybe` は罰しない。
- **ナローイング（narrowing／絞り込み）**〔前編 P4〕… 条件分岐の枝ごとに変数の型を狭めること。
- **ディスパッチ（dispatch）**〔前編 P2〕… レシーバとメソッド名から戻り型を引くこと。
- **型再構築（type reconstruction）／HM**〔後編 P3〕… 注釈から型を復元する推論。TAPL 22 章。
- **単一化（unification）**〔後編 P3〕… 2 つの型を等しくする型代入を求める操作。型再構築の中核。
- **erasure（境界での型の落とし込み）**〔前編 P1 コラム／後編で本式〕… Rigor 内部の精密な型
  （`Constant<"FOO">` など）を、RBS で表せる粗い型（`String`）に落とすこと。「境界で精度を捨てて
  外向けの型に合わせる」操作。**Java ジェネリクスの「型消去（type erasure）」とは別物**
  ― あちらは実行時に型引数 `<String>` を消す話、こちらは静的な型の精度を境界で丸める話。
  `sig-gen`（シグネチャ生成）の中で使われる。

## 部分型と多相

- **部分型（subtype, `<:`）**〔前編 P6／後編 P2〕… 「値が期待の型の箱に入る」関係。TAPL 15 章。
- **包摂（subsumption）**〔後編 P1〕… 合成した `S` が `S <: T` なら期待型 `T` に照合できる、の規則。
- **変性（variance）**〔後編 P2〕… 構築子の引数位置での部分型の向き。返り共変・引数反変。
- **アルゴリズム的部分型付け**〔後編 P2〕… 宣言的な `<:` 規則を、型の形ごとに規則 1 つの決定
  手続きに組み直すこと。前編の `accepts` がこれ。TAPL 16 章。
- **カインド（kind）**〔後編 P4〕… 「型の型」。`App[F, A]` のような型適用の正しさの根拠。TAPL 29 章。
- **gradual consistency（整合）**〔後編 P2〕… `untyped` が絡むときの対称・非推移な関係。`<:` とは別。
- **型代入（substitution）／System F**〔後編 P6〕… 型変数に型を入れる操作。TAPL 23 章。
- **再帰型（μ型）／余帰納（coinduction）**〔後編 P4〕… 自分を参照する型と、その等価判定。TAPL 20–21 章。
- **HKT（高階型）**〔後編 P4〕… 型を取って型を返す型。`App[F, A]`。TAPL 29 章。

## 設計思想

- **gradual typing（漸進的型付け）**〔前編 P9〕… 型の付く所と付かない所を混在させる型付け。
- **never frighten working code（動くコードを脅かさない）**〔前編 P0〕… 誤検知を最上位に避ける規律。
- **robustness principle（Postel の法則）**〔前編 P6／後編 P1〕… 返りは厳密・引数は寛容。
- **健全性（soundness）＝進行＋保存**〔後編 P7〕… 型の付いたプログラムは未定義動作に陥らない、の保証。TAPL 8 章 §8.3。
- **FactStore**〔前編 P4（素朴版）／後編 P5（完全版）〕… フロー感応な「事実」の置き場。
