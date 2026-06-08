---
date: 2026-06-09
lens: reproducibility
target: v1/ja/little/（Part 0–9）
reviewer: subagent（sonnet, 型理論ゼロ・Ruby 中級者ペルソナ）
score: 39/39
---

# 再現性レンズ査読 — v1/ja/little/

## 採点結果

**SCORE: 39/39（全項目パス）**

採点ハーネス：`/tmp/run_grade.rb`（シェイプ非依存 39 項目）  
再現実装：`/tmp/chibirigor-repro-v1/lib/`

## 章別所見

| Part | 明瞭度 | 推測・補足 |
|---|---|---|
| 0 | ✅ 明瞭 | 概念のみ。実装不要 |
| 1 | ✅ 明瞭 | `Const`/`Nominal`/`Dynamic`、`type_of`、`check`、`annotate`、`diagnostic` — すべて本文から直接実装可能 |
| 2 | ✅ 明瞭 | `Type::` 名前空間への移動・`Dispatch` モジュール・`METHODS` 表。`matches?` は Part 7 で `accepts` に差し替える旨も先行提示 |
| 3 | ✅ 明瞭 | 不変 `Scope`・`eval_statement` の `[type, scope]` パターン — 丁寧に説明されている |
| 4 | ✅ 明瞭 | `Union` 型と `union()` helper（flatten/dedup/unwrap）・`IfNode` 処理 |
| 5 | ⚠️ 軽微な推測 1 件 | `nil?` の else 枝（`remove_nil`）は明示。**`is_a?` の else 枝（`remove_class`）は類推で実装**。本文では真の枝の絞り込みのみ示す — 対称実装を「演習」相当として推測 |
| 6 | ✅ 明瞭 | `HashShape`・`Tuple`・`read_index` — コードスニペット付きで明確 |
| 7 | ⚠️ 軽微な推測 1 件 | `:yes/:no/:maybe` の actual-is-Union（weakest）は本文明示。**expected-is-Union（strongest）は演習のヒントから推測** |
| 8 | ✅ 明瞭 | `Rbs.load`・`CORE` 定数・`DefNode` 処理・`method_signature` — 実装ステップが追いやすい |
| 9 | ✅ 明瞭 | `untyped` が union を支配するコード変更（1 行）・`baseline` の差し引きロジック — 本文通りに実装可能 |

## 推測箇所（2 件・軽微）

### G1（Part 5）— `is_a?` の else 枝の実装

**箇所**：`narrow_type` で `is_a?` の `truthy=false` 時（else 枝）の型縮小。  
**本文**：真の枝で `Type::Nominal[klass]` に絞ることは明示。偽の枝は「保守的に触らない」と述べており、`remove_class` は言及なし。  
**推測**：真の枝の対称として `remove_class` を実装（Union から当該クラスを除く）。  
**影響**：採点では問われていない（保守的に「触らない」ほうが正解でも採点には影響なし）。  
**判定**：本文は意図通り（FP 安全のため else 枝は触らない方針）。推測は本文と矛盾しないが、**明示してもよい**（「偽の枝はスコープをそのまま返す」と 1 文加えると推測不要）。

### G2（Part 7）— expected-is-Union の `accepts`

**箇所**：`Accepts.call` で expected が `Union` のとき（strongest ルール）。  
**本文**：actual が `Union` のとき（weakest ルール）は本文で実装。expected が `Union` の場合は「演習」のヒントで示唆のみ。  
**推測**：演習ヒントから "strongest" ルールを補完実装。  
**影響**：採点の P7-1（union-of-ints no FP）が通る理由の一部がこの実装に依存している可能性がある。  
**判定**：本文に追加説明の余地あり。「expected が Union のとき、どれか 1 つに合えば `:yes`」を本文の実装コードに明示するか、演習を「実装してみよ」に格上げするとよい。

## 総評

**前編 v1/ja/little/ は「型知識ゼロ・Ruby 中級者」が本文だけで再実装できる水準に達しています。**  
39/39 の全項目パスは、説明の密度・実装ステップの順序・FP 安全ケースの提示がすべて機能していることを示します。

特に良かった点：
- Part 3 の「スコープを縫う」パターン（`[type, scope]` の返し方）が丁寧
- Part 5 の `possible?` ガード（dead-branch の FP 回避）が図なしでも伝わる
- Part 9 の `untyped` 支配が「1 行の追加」として提示されており迷いなし

推奨する修正：軽微なので次マイルストーンの任意適用として記録（MUST ではない）：

| ID | Part | 重大度 | 内容 |
|---|---|---|---|
| R-1 | 5 | nitpick | `is_a?` の偽の枝の方針（「スコープをそのまま返す」）を 1 文明示 |
| R-2 | 7 | nitpick | expected-is-Union の `:yes` ルールを本文のコードに明示（演習ではなく実装例として） |
