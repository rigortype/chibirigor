# v1/ja 移植ガイド（内部メモ・各移植担当はまず読む）

> 本書（v1）は [`_reorg-proposal.md`](_reorg-proposal.md) の合意済み方針で、`draft/` の原稿を
> `v1/ja/` へ**二巻維持・大胆再構成**で移す。本ファイルは**正典**：相互参照の番号・ファイル名・
> 清書フォーマット・章ごとの抜き差し仕様を一元化する。原稿は**読み取り専用の素材**。

## 0. 不可侵ルール（AGENTS.md より）

- **1 章 1 難所**。派生トピックは「発展ノート」か「付録」へ逃がす。本筋を細らせない。
- **三題噺（① 型理論 ↔ ② Ruby/RBS ↔ ③ Rigor 実装）**の枠組みを保つ。
- **framing**：chibirigor は「**推論を土台にした型チェッカー**」。「推論器 vs チェッカー」「check は
  副産物」とは言わない。式から型を**合成**する（呼び出し元から引数を逆算しない＝それは TypeProf）。
- **誤検知を出さない／拒まない**の思想を曲げない。
- **教えるコードは原稿の最小版のまま据え置く**（churn させない）。機能は発展ノートで重ねる。
- **Rigor についての事実記述は実態と一致**（簡略化は自由、事実誤認はダメ）。原稿の記述を維持。
- **章の閉じ方**：「（導入）→ 本文 → まとめ表 → `## 演習` → 次章予告」。各章末の `> 検証メモ`
  （執筆者向け足場）が残っていれば**除去**。読者向け参考書メモは残す。

## 1. 統制語（表記を揃える）

キャリア／ナローイング（動詞は絞り込む）／型チェッカー・型チェック／型推論（エンジン）／
`untyped`・`Dynamic[Top]`／`Top`・`Bot`／誤検知・ロバストネス原則／RBS interface・構造的
インターフェース／漸進的型付け（gradual typing）。約物は全角。コード/識別子はバッククォート。

## 2. 正典：新ファイル名と番号

### 前編 `little/`
| 新 | ファイル | 旧対応 |
|---|---|---|
| 0 | `part0-introduction.md` | 旧 little P0 |
| 1 | `part1-literals-and-arithmetic.md` | 旧 P1（縮約） |
| 2 | `part2-method-dispatch.md` | 旧 P2 ＋ 旧 P1 の 1-4c |
| 3 | `part3-scope-and-statements.md` | 旧 P3 |
| 4 | `part4-union.md` | 旧 P4 の前半（Union） |
| 5 | `part5-narrowing.md` | 旧 P4 の後半（ナローイング） |
| 6 | `part6-hash-and-tuple.md` | 旧 P5 |
| 7 | `part7-accepts-and-trinary.md` | 旧 P6 |
| 8 | `part8-rbs-and-signatures.md` | 旧 P7 ＋ 旧 P8 |
| 9 | `part9-gradual-philosophy.md` | 旧 P9 |

### 後編 `seasoned/`
| 新 | ファイル | 旧対応 |
|---|---|---|
| 1 | `part1-bidirectional-typing.md` | 旧 seasoned P1 |
| 2 | `part2-subtyping-and-variance.md` | 旧 P2 |
| 3 | `part3-generics-and-substitution.md` | 旧 P6 |
| 4 | `part4-recursive-types.md` | 旧 P4 |
| 5 | `part5-real-inference.md` | 旧 P3 |
| 6 | `part6-fact-store.md` | 旧 P5 |
| 7 | `part7-soundness.md` | 旧 P7 |
| 8 | `part8-toward-rigor.md` | 旧 P8 |

## 3. 相互参照リマップ（**必ず適用**）

原稿の「Part N」「前編 Part N」「後編 Part N」を新番号へ。

**前編内の参照**（前編→前編）:
`P0→0, P1→1, P2→2, P3→3, P4→4(Union文脈)/5(絞り込み文脈), P5→6, P6→7, P7→8, P8→8, P9→9`

**後編内の参照**（後編→後編）:
`P1→1, P2→2, P3→5, P4→4, P5→6, P6→3, P7→7, P8→8`

**巻をまたぐ参照**:
- 前編→後編：旧「後編 Part 3（推論）」→「後編 Part 5」、旧「後編 Part 5（FactStore）」→
  「後編 Part 6」、旧「後編 Part 6（ジェネリクス）」→「後編 Part 3」。Part 1/2/4/7/8 は不変。
- 後編→前編：上の前編内リマップに同じ（旧 P5→6, P6→7, P7→8, P8→8）。

> 迷ったら*テーマ*で引く：accepts/三値＝前編 7、RBS/sig＝前編 8、推論＝後編 5、
> FactStore＝後編 6、ジェネリクス＝後編 3、再帰型＝後編 4。

## 4. frontmatter（各章先頭）

```yaml
---
title: The Little/Seasoned chibirigor Part N ― <タイトル>
description: <一文>
sidebar:
  order: <前編は 1..10 を Part0..9 に、後編は 11..18 を Part1..8 に>
---
```

H1 は `# The Little/Seasoned chibirigor Part N ― <タイトル>`（`【ドラフト】`/`【試し書き】`
マーカーは除去）。後編の旧 frontmatter にある `draft: true` は v1 では外す。

## 5. 横断トピックの行き先（重複は畳む）

| トピック | v1 での扱い |
|---|---|
| 特別な型 3 種（`untyped`/`void`/`never`、`Top`/`Bot`） | 各章は初出 1 行のみ → 付録 `a1`。前編 9 に総括 box、深掘りは後編 2(Bot/never)・1(void) |
| TypeProf 比較 | 前編：原則＝Part 0、具体（引数は後編 5）＝Part 8 の 2 回のみ。後編：対比＝Part 5 に一本化 |
| baseline | 前編 Part 9 に本体集約。Part 1 は診断フォーマットのみ（列照合/ADR-22 は出さない） |
| gradual（consistency/guarantee） | 後編 Part 7 に集約。Part 2 は 1 行ポインタ |
| 余帰納 vs fuel/予算 | 後編 Part 7 に集約。Part 4 は HKT 別解のみ（発展ノート） |
| dispatch 5 段カスケード | 付録 `a3`。前編 Part 2 は素朴な表引きに留め 1 行ポインタ |
| refinement carrier / ナローイング個別パターン | 付録 `a2`。後編 Part 6 は本筋（6 バケツ＋stability＋join）に純化 |

各章は、付録へ送ったトピックに**1 行ポインタ**（例：「詳しくは付録 a1」）を残す。

## 6. 章ごとの抜き差し（要点。詳細は proposal §2・§3）

### 前編
- **0**：早見表は付録 a4 へ寄せ 1 つに。TypeProf 原則はここで一度。
- **1**：`Const`/`type_of`/`check`/`annotate` に純化。1-1 の untyped 軸 A/B 長考→付録 a1、
  1-4c 定数畳み込み→Part 2、1-4b の baseline 詳細→Part 9（診断の列/キャレットは残す）。
  「丸める」で一貫（畳み込みは Part 2 で覆す形にしない）。
- **2**：旧 P2 ＋ 旧 1-4c を**発展ノート**として同居（「実際は Dispatch 側に置く」を回収）。
  旧 2-2 の 5 段カスケードは外し付録 a3 へ（1 行ポインタ）。
- **3**：旧 P3 をほぼ移植（最もスリムな良章）。
- **4**：旧 P4 の Union 導入に純化（4-1 ＋ Union 道具）。ボトム型コラム→付録 a1。
- **5**：旧 P4 の後半（ナローイング）＝ `narrow`／偽は false・nil／is_a? dead branch／
  2 つの掟／再代入リセット。ぬるぽコラムは 1 つ残してよい。
- **6**：旧 P5（hash/tuple）。系譜コラムは残す。部分型の予告は「Part 7 で」。
- **7**：旧 P6（accepts/三値）をほぼ移植。Postel コラムは残す。
- **8**：旧 P7（RBS 一元化）→ 旧 P8（def から戻り型合成 → RBS 風 sig）を**一続き**に統合。
  旧 8-1 の void コラムは Part 9 の特別な型総括へ。8-3 で「引数推論は後編 Part 5」を一度。
- **9**：旧 P9 ＋ **baseline 本体**（Part 1 から集約）＋「特別な型 3 種」総括 box。
  「わざと見逃す 4 箇所」は後編 Part 7 で健全性として再論、と予告。

### 後編
- **1**：旧 S1。1-7-a 発展ノート（check(rbs:)）は移設し「コードを書かない章」のトーンを保つ
  （param コラムも整理）。
- **2**：旧 S2。変性に純化。gradual consistency(2-5)→Part 7 へ、ここは 1 行ポインタ。
- **3**：旧 S6（ジェネリクス）を**前出し**。後続の推論(Part 5)がこれを後方参照。examples=`subst.rb`。
- **4**：旧 S4（再帰型 μ・余帰納）。HKT/fuel は発展ノート。予算は Part 7 へ。examples=`mu_typeeq.rb`。
- **5**：旧 S3（推論）。型変数は Part 3 を後方参照。TypeProf 対比はここに一本化。examples=`unification.rb`。
- **6**：旧 S5（FactStore）。本筋（6 バケツ＋stability＋join）に純化。コラム 5 本→付録 a2。
  5-3/5-4 のエスケープ重複は 1 つに。examples=`fact_invalidation.rb`。
- **7**：旧 S7。gradual（consistency＋guarantee）と「余帰納 vs 予算」をここに集約。Part 1 とブックエンド。
- **8**：旧 S8。ほぼ移植（8-2-a 発展ノート保持）。ADR 参照表は維持。

## 7. examples / 検証

- 後編 examples は `v1/ja/seasoned/examples/` に複製済み（`subtype`/`unification`/`mu_typeeq`/
  `fact_invalidation`/`subst`/`check_docs`）。後編本文の `<!-- run: -->`/`<!-- include: -->`
  ディレクティブのパスは v1 構成に合わせて直す（移植担当が本文内の相対パスを更新）。
- 教えるコードの**挙動**は原稿と同一に保つ（lib は不変）。出力の数値・型表示を変えない。
