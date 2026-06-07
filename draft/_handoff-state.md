# chibirigor 作業状態と引き継ぎ（← 次セッションは最初にここを読む）

二巻のオンライン技術書 `chibirigor`（最小の Ruby 型推論ベース検査器を作りながら本物の Rigor を
学ぶ）。コンテキスト逼迫時の再開用に、状態・進め方・落とし穴を一枚に集約。詳細は各
`_*-review.md` / `README.md` / `docs/` の design draft へ。

## 現在の状態（2026-06-08 時点）
- **前編 The Little chibirigor**（`draft/little/ja/` Part 0–9）：実装（`lib/` ＋ `test/test_part*.rb`
  9 本緑）＋本文＋まえがき・用語集・演習（全章）・ASCII 図 3 枚・清書フォーマット（足場剥離済み）。
  **5 レンズ査読を通過**し、刊行ドラフト水準。
- **後編 The Seasoned chibirigor**（`draft/seasoned/ja/` Part 1–8 ＋ `examples/`）：ドラフト済み。
  動く設計スケッチ 5 本＋ドリフトチェッカー。
- 共通：`draft/preface.md`（二巻まえがき）、`draft/glossary.md`（用語集）。
- 実装：`lib/chibirigor/`（Prism、Ruby 3.4、依存ゼロ）。

## レビュー方法論（再利用可能な QA レシピ）
独立コンテキストのサブエージェントで **5 つのレンズ**を回し、各結果を `_*-review.md` に記録、
**軸（やさしい／gradual／脅かさない）を保って選択適用**（全部は反映しない）：
1. **再現性** … 型知識ゼロの読者役が本文だけで再実装し、挙動を採点（前編 34/34 ×3 名で実証）。
2. **型理論エキスパート** … 形式的正確さ（`_expert-review-findings.md`）。
3. **技術書編集者** … 構成・刊行完成度（`_editorial-review.md`）。
4. **ドメイン著者 mametter** … 『しくみ』著者＋TypeProf 作者の視点（`_mametter-review.md`）。
5. **日本語校正・校閲** … 言語の質（慣用句誤用・AI 調・表記ゆれ）。

## バックログ（未適用 / 進行中）
- **校正**：「腑に落とす／体に入れる／実はこれ〜」等の言語修正 ＝ 進行中（一部適用済み）。残りを一巡。
- **編集者 Top7**：図の本清書（デザイナー領域）／後編側のスケッチ追加／後編の性格づけは適用済み。
- **mametter**：前編はほぼ反映済み（推論器→「推論を土台にした検査器」、TypeProf 明示 ×3）。
- **専門家（後編）**：`_expert-review-findings.md` の MISLEADING/REF（TAPL 29 章併記、FactStore
  バケツ名の spec 照合、Part 1「合成は失敗しない」の限定 等）。
- 後編セッションへの依頼文は既出（前回の指示文＋上記レビュー 2 本のパス）。

## 進め方（writing recipe）
- 章単位：プローズ → （前編は）**実 Prism/Ruby で検証＋ `test_part*.rb`** → 査読レンズ → 選択適用
  → 論理単位ごとに commit。
- 5 レンズ査読は **マイルストーンで**（per-edit ではなく区切りで）。
- 軸を最上位に。**すべてのフィードバックは反映しない**。

## 所有とコーディネーション（fork 時の鉄則）
- **ディレクトリで所有を分ける**：前編 = `draft/little`、後編 = `draft/seasoned`。
- 共有ファイル（`preface`/`glossary`/`_*-review`）と**横断パス（用語統一・校正）は単一オーナー**が、
  相手の対象巻が安定しているときに実施。**同一ファイルの同時編集を避ける**（今回の最大の摩擦源）。
- commit は必ず **`git add <個別ファイル>`**。`-A` 禁止（相手の WIP を巻き込む）。
- 非同期連絡は `_handoff*` / `_*-review` の**コミット済みノート**で（低摩擦・永続）。

## 検証コマンド
- `ruby test/test_part1.rb` 〜 `9`（前編 lib 挙動、依存ゼロ）。
- `ruby draft/seasoned/ja/examples/<sketch>.rb`（後編の動くスケッチ）。
- `ruby draft/seasoned/ja/examples/check_docs.rb`（本文↔コードのドリフト検出）。
- Ruby 実行は `nix --extra-experimental-features 'nix-command flakes' develop --command ruby …`、
  **`cd` しない**（flake 探索が壊れる）。

## 既知の落とし穴
- 並行コミットで HEAD がレースする → `git add <file>` で自分のファイルだけ commit。
- rubocop autocorrect が `examples/*.rb` の「わざとおかしいコード」を壊す → 挙動検証は `test/` の
  **文字列ソース**で。
- 前編コードは `module Chibirigor`／`Chibirigor::Type::*`／診断は `{line:, message:}`／annotate は
  `{line:, type:}`。本文と lib はこの形で一致させる。
