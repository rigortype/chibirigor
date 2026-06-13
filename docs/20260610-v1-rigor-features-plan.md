# v1 への Rigor 由来機能追加 ― 構成計画（2026-06-10）

本文（`book/v1/ja/`）のレベルをもう一段引き上げるため、実 Rigor の機構の**最小版**を
`lib/` に足し、その効果を**発展ノート方式**（`AGENTS.md`）で本文へ重ねる。統治の軸は
[`../AGENTS.md`](../AGENTS.md)／[`ROADMAP.md`](ROADMAP.md)。台帳は [`CURRENT_WORKS.md`](CURRENT_WORKS.md)。

## 狙い（なぜこれで「本文レベル」が上がるか）

付録・後編には「**本書：仕組みなし／概念だけ**」と書いた箇所が複数ある（a3-1/a3-2 の
「本書：仕組みなし」、後編 Part 3/5/6 の概念スケッチ）。そこに対応する**極小だが動く実装**を
足すと、その記述が「本書にも極小版があり、実物はそれを拡大した形」に**格上げ**できる。
机上説明 → 動く worked example への昇格が、本文の密度を上げる。

## 不変条件（全スライス共通・予算保護）

- **1 スライス＝難所 1 つ**。独立に出荷可能。予算を脅かしたら止めて概念のまま戻す。
- **依存ゼロ**（Prism のみ）。テストフレームワーク不使用。
- **誤検知ゼロ最優先**：分からなければ `untyped`／`:maybe` に倒す。新しい推論・診断は
  「期待が `untyped` なら照合しない」「閉じた既知型でだけ主張する」を守る。
- **本文反映は発展ノート方式**：章の本筋コードは据え置き、章末 prose ＋ 別建て抜粋で重ねる。
- 各スライスは lib に**独立テスト**（`test/test_<feature>.rb`、plain Ruby・文字列ソース）を足す。
  Phase 1–2 の先例＝`test/test_plugin.rb`／`test/test_return_type_check.rb`。
- 用語・表記はサイト準拠（`AGENTS.md` 統制語）。Rigor についての**記述は実態と一致**（フィデリティ）。

## スライスと順序（risk-adjusted・効果順）

| # | 機能 | Rigor 対応 | 労力 | 本文の格上げ先 | FP |
|---|---|---|---|---|---|
| 1 | ~~`type-of` 位置クエリ~~ **（2026-06-13 廃止）** | `rigor type-of FILE:LINE:COL` | S | 付録 a3-2・前編 Part 1 コラム | 0（表示のみ） |
| 2 | `check --explain`（fail-soft 地図） | `rigor check --explain` | S〜M | 付録 a3-1・前編 Part 9 | 0（:info のみ） |
| 3 | 到達不能アーム診断 | ADR-47（`flow.unreachable-clause`） | S〜M | 付録 a1-3・a5-5・後編 Part 7 | 0（封筒を厳格に） |
| 4 | ジェネリクス 5a（要素型の読み） | 軽量 HKT（ADR-20）／`unification.rb` | M | 後編 Part 3/5/6・ROADMAP L1 | 0（degrade 死守） |
| 5（任意） | リファインメント表示 | Difference/Refined/IntegerRange（ADR-3） | M | 付録 a2-6 | 0（表示のみ） |

着手順は 1 → 2 → 3 → 4、5 は予算が残れば。1・2 は付録 a3 を直接格上げするパイプライン確認も兼ねる。
4 が本のクライマックス（generics が lib で一本につながる）。

## 各スライスの成果物

### 1. `type-of` 位置クエリ ― **廃止（2026-06-13）**
- 「2 つの機能だけ（`check`／`annotate`）」の看板と衝突し読者を混乱させるため、実装
  （`type_at.rb`・`exe type-of`・`test_type_at.rb`）と本文の極小版（旧 a3-2x）を**完全廃止**。
- 付録 a3-2 は**実 Rigor の `type-of`** の紹介として残す（位置指定＋内部精密型/RBS 境界型の 2 段）。
  chibirigor 側は「極小版は設けない・推論型は `annotate` で見る」と明記。前編 Part 1 のコラムも
  実 Rigor の道具紹介として整合（変更不要）。

### 2. `check --explain` ― S〜M・FP 0
- **lib**：未知ディスパッチで `Dynamic` に倒れた地点を `{line, column, reason}` で**provenance 収集**。
  `check(source, explain: true)` のとき、それらを `:info`（`severity:`）診断として返す。
  既定（`explain:false`）は挙動不変。
- **CLI**：`exe/chibirigor check --explain FILE`。`:info` 行を「ここで型を見失いました」で描く。
- **test**：`test/test_explain.rb`（未知呼び出しに info が出る／既定では出ない）。
- **本文**：付録 a3-1 の表「本書：仕組みなし（黙るだけ）」に**発展ノート**で「極小版あり」を重ね、
  `Dynamic` マーカー回収の縮図として前編 Part 9 から戻りポインタ。

### 3. 到達不能アーム診断 ― S〜M・FP 0（封筒厳格）
- **lib**：`Type::Bot` を導入（`to_s = "bot"`）。`if` の枝でナローイングが**証明可能に空**
  （subject が閉じた既知型で、その枝の条件が成立し得ない）になるとき、その枝を**到達不能**として
  `:info` 報告し、枝は型付けしない（FP 回避）。
  - **封筒（厳格）**：subject が `Dynamic` を含む／`Union` に `Dynamic` を含む／ループ・ブロック内
    なら**報告しない**（実 Rigor の FP envelope の縮図）。`raise` 経路の `bot` 合流も同様に静観。
- **test**：`test/test_unreachable.rb`（証明可能な dead 枝に info／gradual な枝は無診断）。
- **本文**：付録 a1-3（`bot`）・a5-5（unreachable vs missing arm）に**発展ノート**、後編 Part 7
  の概念記述を「lib で動く」に格上げ。

### 4. ジェネリクス 5a（要素型の読み）― M・FP 0
- **lib**：`Type::Nominal` に `type_args`（既定 `[]`）。`Array[Elem]`/`Hash[K,V]` を持ち、
  `arr.first`/`arr.last`/非リテラル添字 `arr[i]` が `Elem` を返す。配列リテラルの要素型は
  要素型の `union`。**生レシーバ／未束縛型変数は `Dynamic` に degrade**（前編「埋まらねば untyped」死守）。
  単一化は 5a では不要（要素抽出のみ）。`examples/unification.rb` 昇格は 5b 以降。
- **test**：`test/test_generics.rb`（`[1,2].first → Integer`、`[].first → untyped`、生 `Dynamic` 受信→ `untyped`）。
- **本文**：後編 Part 3/5/6 の対応スケッチを「lib に昇格済み」に更新。前編への新章追加は手応えを見て判断。
- ROADMAP §5 / CURRENT_WORKS Track L（L1）に進捗反映。

### 5（任意）リファインメント表示 ― M・FP 0
- 予算が残れば。`Tuple`（非空配列）由来の `non-empty-array` 相当を**表示だけ**で見せ、付録 a2-6 を格上げ。
  危なければ概念のまま後送り（ROADMAP 非目標の範囲を侵さない）。

## 検証ゲート（各スライス出荷時）

1. `ruby test/test_<feature>.rb` 緑、かつ `make test`（全 part）緑。
2. FP ゼロ例を最低 1 つテスト化（「動くコードを脅かさない」）。
3. `make drift`（本文ドリフト）緑、`make impls-verify` 緑。
4. 本文（付録/後編）・`draft/glossary.md`／`book/v1/ja/glossary.md`・CURRENT_WORKS を更新。
5. 大きな節目でフィデリティ再チェック（`_fidelity-review.md`）。

## 段スナップショットの扱い

新機能は**発展ノート方式**で章の本筋コードを変えないため、`impls/steps` の各段は据え置く。
機能は `lib/`（＝実物の道具）と独立テストに置き、`impls/dist` とは機能的に分岐してよい
（`make impls-check` は dist の手編集検出であって lib との一致は要求しない）。
