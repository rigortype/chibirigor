# フィデリティ・チェック（D1）― 2026-06-08 実施

> chibirigor 本文の「Rigor だと…」記述と実際の Rigor ソース・ADR を突き合わせた独立レビュー。
> Rigor 参照：`docs/adr/`（ADR 0〜51）・`docs/type-specification/`・`docs/internal-spec/`・`lib/rigor/`。

---

## 重大なズレ（事実と明確に違う）

### Z1 ― baseline の照合キー（前編 Part 9）

**本文の記述（checker.rb コメント）**：「行とメッセージで照合し、列は含めない」

**実際（ADR-22 WD1）**：Rigor の baseline キーはデフォルトで `(file, rule, count)` であり
行番号はキーに含まれない。オプトインで `(file, rule, message, count)` も選択可。
「行番号を含まないことで編集に強くする」という論旨は正しいが、chibirigor の実装（`line + message`）
を Rigor 本体と同一視する記述があれば誤解を招く。

**対応方針**：前編 Part 9 の「Rigor だと」三題噺の記述を「chibirigor の簡略版では行番号＋メッセージ、
実 Rigor のデフォルトはルール ID で照合する（行番号は含めない）」と修正する。

**状態**: ✅ 適用済み（2026-06-08）

---

### Z2 ― Nominal 型引数の変性（後編 Part 2）

**本文の記述**：「Rigor は Nominal の型引数を『読み共変・書き反変・両方で不変』として扱います」

**実際（`lib/rigor/inference/acceptance.rb` 行 394–438）**：
現実装は型引数を一律共変で処理。宣言サイト変性（declared variance）は Slice 5+ の未実装課題。
「構造的インターフェースの attr_reader/writer/accessor = 共変/反変/不変」の記述は正しいが、
それは Nominal の型引数の話ではない。

**対応方針**：Part 2 の該当箇所を「実装上は型引数を共変に処理し、宣言サイト変性は設計済みだが
未実装（Slice 5 以降の課題）」と限定する。

**状態**: ✅ 適用済み（2026-06-08）

---

### Z3 ― 推論予算の実装状況（後編 Part 7）

**本文の記述**：「HKT 還元 fuel（Part 4）、推論予算（inference budget）」と並置している

**実際（ADR-41、Status: Proposed）**：ADR-41 冒頭に
"Nothing here is implemented yet; the work is sequenced as Layer 1 and Layer 2"と明記。
設定キーは未パース・打ち切り動作は未配線。HKT fuel（ADR-20、実装済み）とは別物。

**対応方針**：Part 7 の記述を「HKT fuel は実装済み（ADR-20）、broader な推論予算は
ADR-41 として設計済みだが**現時点では未実装**」と区別する。

**状態**: ✅ 適用済み（2026-06-08）

---

## 軽微なズレ / ニュアンス差

### N1 ― 「RBS = 真実の源」の限定（前編 Part 7）
Rigor のディスパッチは多層で RBS より高位ティアが先に適用される。「RBS は主要な事実源の一つ」
という表現に強化すると正確。教育的単純化として許容範囲。→ 対応不要（注記のみ）

### N2 ― "never frighten working code" を「合言葉」と呼ぶこと（前編 Part 0）
Rigor 公式の公式定義名ではないが精神的に正確。ADR-26 で "project's first value" として確認済み。
→ 対応不要

### N3 ― `Const[1]` を比較前に「クラスに丸める」（前編 Part 6）
実際には Constant 同士の直接照合など丸めない経路もある。教育的単純化として許容範囲。→ 対応不要

### N4 ― robustness principle の表現（後編 Part 1）
「返すものは厳密・受け取るものは寛容」は精神的に正確。ADR-5 の適用ルールはより具体的。→ 対応不要

### N5 ― `DataClass`/`DataInstance` の `Struct` 範囲（前編 Part 5）
ADR-48 の現行実装は `Data.define` が中心。`Struct` の完全折りたたみは残課題。
→ 「続編に送ったもの」の記述なので許容範囲

### N6 ― ADR-47 の引用範囲（前編 Part 4）
ADR-47 は「到達不能枝の診断」に特化。「`case`/`in` 絞り込み全般」の参照先としては不完全。
→ 記述を「到達しない枝の検出（ADR-47）と絞り込みの制御フロー分析」に限定する旨をコメント

---

## 確認不能

### U1 ― ブロック捕獲時の事実無効化の詳細規則（前編 Part 3）
`captured_local` バケツの存在は確認できるが、無効化タイミングの詳細仕様が未文書化の可能性。
→ 「実 Rigor の FactStore の機微」という書き方は許容範囲。具体規則には踏み込まない。

---

## 問題なし（主要項目）

| ファイル | 記述 | 照合先 |
|---|---|---|
| 前編 Part 1 | `Constant<1>` 型（値そのもの） | ADR-3 WD |
| 前編 Part 3 | Scope は不変設計（本物 Rigor も同じ） | `lib/rigor/scope.rb` |
| 前編 Part 4 | `Dynamic` は post-guard narrowing しない | `lib/rigor/inference/narrowing.rb` |
| 前編 Part 5 | HashShape は open（少なくとも） | `lib/rigor/type/hash_shape.rb` |
| 後編 Part 1 | `ExpressionTyper`（合成 ⇒）・`accepts`（照合 ⇐） | `lib/rigor/inference/expression_typer.rb` |
| 後編 Part 4 | HKT は defunctionalised encoding（ADR-20） | ADR-20 確認済み |
| 後編 Part 5 | FactStore 6 バケツの名称 | `docs/internal-spec/inference-engine.md` |
| 後編 Part 7 | HKT fuel は実装済み（デフォルト 64 ステップ） | ADR-20 Slice 2a |
| 後編 Part 8 | ADR 番号参照表（全 21 番号） | 全 ADR ファイル存在確認済み |
| 後編 Part 6 | Rigor は RBS のスーパーセット | `docs/types.md`・ADR-1 |
