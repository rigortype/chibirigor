# CURRENT_WORKS — 次の作業候補（レジューム・ブックマーク）

> 次の担当（人／サブエージェント）が**ここから再開**するための作業台帳。統治の軸は
> [`../AGENTS.md`](../AGENTS.md)。本書は意図的な*簡略版*だが Rigor についての*記述*は実態と
> 一致させること（AGENTS.md §「Rigor を真実の源として参照する」）。
> 一過性のメモ ― 大きな区切りで更新する。

## 現状（ship-readiness）

- **前編 = The Little chibirigor**（`draft/little/ja/`、Part 0〜9）：**刊行ドラフトとして高完成度**。
  - 5 つの独立レビューを通過：初学者の再現性（本文だけで再実装→リファレンス挙動 **34/34 を 3 名**）、
    型理論エキスパート（ERROR ゼロ、参照 1 件修正）、技術書編集者、mametter（TypeProf 作者＋
    『しくみ』著者）、日本語校正・校閲。指摘は軸を保って選択適用済み。
  - 整備済み：二巻共通まえがき（`draft/preface.md`）・用語集（`draft/glossary.md`）・Part 0 俯瞰表・
    **全章の `## 演習`**・ASCII 図 3 枚（0-1/2-1/4-1）・清書フォーマット（`【試し書き】`/検証メモを除去、
    各章「まとめ→演習→次章予告」で完結）・TypeProf の明示・「推論を土台にした検査器」への framing 修正。
  - 実装：`lib/chibirigor/`（Part 1〜9）＋ `test/test_part1〜9.rb` がグリーン。CLI 動作。
- **後編 = The Seasoned chibirigor**（`draft/seasoned/ja/`、Part 1〜8）：**ドラフト済み**（別セッション主進行）。
  - `examples/` に実行可能スケッチ（subtype / unification / mu_typeeq / fact_invalidation / subst）。
  - 巻の性格づけ（「作る」より「読み解く」）を README で明言。

## レビュー資産（バックログ。未適用の指摘を引ける）

| ファイル | レンズ | 残っている主な未適用項目 |
|---|---|---|
| [`../draft/_editorial-review.md`](../draft/_editorial-review.md) | 技術書編集者（両巻） | 図の本清書・後編の演習/スケッチ・局所過積載整理 |
| [`../draft/seasoned/ja/_expert-review-findings.md`](../draft/seasoned/ja/_expert-review-findings.md) | 型理論家（後編） | MISLEADING/REF/INCONSIST 群（下記 B1） |
| [`../draft/_mametter-review.md`](../draft/_mametter-review.md) | TypeProf 作者＋『しくみ』著者 | 後編 Part 3 での TypeProf 対比・map 表の本体組み込み |
| [`../draft/_copyedit-review.md`](../draft/_copyedit-review.md) | 日本語校閲 | 太字過密・誇張反復・「いよいよ」・「differ 置換」（下記 A1） |
| [`../draft/seasoned/ja/_handoff-notes-for-little.md`](../draft/seasoned/ja/_handoff-notes-for-little.md) | 後編→前編 | 章対応の参照番号統一・ドリフトチェッカ前編展開 |

## 次の作業候補（トラック別・優先度つき）

### Track A — 前編 仕上げ（低リスク・前編担当）
- **A1（小）** 校閲の留保適用（`_copyedit-review.md`）：太字を **1 段落 1 強調**へ間引く／誇張形容詞
  （「決定的に違う」「正反対」「いちばん大事」）を章ごと一度ずつへ／「いよいよ」を半減／
  **「differ 置換」**を初出注記 or 平易化（要：技術定訳か確認）／「合言葉」vs「標語」を統一。
- **A2（中・デザイナー）** 図の本清書：ASCII の図 0-1/2-1/4-1 を正式図版へ。必要なら Part 6 三値の図を追加。
- **A3（小・要判断）** ✅ 局所整理完了：Part 4 の `possible?` を blockquote 実装メモへ移動／Part 6 に `### 6-3a. Union が引数に来るとき` 独立小節を追加。
- **A4（任意）** map §12 の章対応表を前編付録/Part 0 へ組み込む検討（mametter 推奨）。

### Track B — 後編 拡充（主トラック・別セッション）
- **B1（中）** 専門査読の反映（`_expert-review-findings.md`）：P1「合成は失敗しない」を chibirigor 固有と
  限定／P2「不変」用語→「非自明な部分型なし」／P4 余帰納スケッチの簡約性を一言／P6 シャドーイングの
  「適用 vs 代入」段階を明示／P2 §2-5 三値表に実装注／**P4・P8 に TAPL 29 章（型演算子・kind）**／
  P5 FactStore のバケツ名を実 Rigor の spec と照合（`docs/internal-spec/` 等）。
- **B2（中）** 各章末に `## 演習`（**前編フォーマットをミラー**。後編は証明・トレース・規則導出系）。
- **B3（中・デザイナー）** 図 5 枚：双方向 `⇒`/`⇐`・部分型格子・変性 S-Arrow・FactStore 6 バケツ・μ 畳/展開。
- **B4（小）** TypeProf 対比を Part 3（本物の型推論）で明示：whole-program 抽象解釈で呼び出し元から
  引数を逆算する TypeProf と、ローカル＋カタログの Rigor の差（mametter）。出典：`rigor/docs/handbook/appendix-typeprof.md`。
- **B5（小）** 健全性の回収：Part 7 で「前編 Part 9 の*わざと見逃す 4 箇所*を progress 放棄として言い直す」と
  明示接続（前編側は前振り済み）。
- **B6（小）** 最終章 Part 8：プラグイン/キャッシュ/LSP/CI の駆け足列挙を「実 Rigor のどの ADR から
  読むか」の具体へ（`rigor/docs/adr/README.md` を参照）。
- **B7（小）** スケッチ整合：`examples/check_docs.rb` で本文 run 出力と一致を保つ。

### Track C — 横断・インフラ
- **C1** ✅ ドリフトチェックを前編に展開済み。`draft/little/ja/examples/check_docs.rb`（examples 3 本）＋ `Makefile`（`make` で両巻チェック）。`<!-- run: -->` ディレクティブ対応済み、`<!-- include: #region -->` も有効（パス汎用化済み）。Part1 の annotate 出力ドリフト（Integer→3）を検知・修正済み。
- **C2** ✅ サイト統合準備（Astro + Starlight）：前編 10 章・後編 8 章に frontmatter 追加済み（`title`/`description`/`sidebar.order`、後編は `draft: true`）。H1 はそのまま残存。
- **C3** 用語集（`draft/glossary.md`）の維持：後編で形式用語を導入したら初出章つきで追記。
- **C4（任意）** 英語版 `draft/little/en/` の起こし（TAPL を共通参照に。『しくみ』は日本語のみ）。

### Track D — フィデリティ維持（AGENTS.md §「Rigor を真実の源に」）
- **D1** マイルストーンごとに**フィデリティ・チェック**のサブエージェントを 1 本：本文の「Rigor だと…」
  主張を拾い、`rigor/docs/`（handbook / type-specification / internal-spec / adr）・`lib/rigor/` と突き合わせ、
  ズレを `draft/_*-review.md` に記録 → 軸を保って選択適用。

## 着手の入口（推奨スライス）

1. **後編を別セッションで主進行**（Track B）：指示文は前編担当が用意済み（`_editorial-review.md` ＋
   `_expert-review-findings.md` を読ませる）。まず **B1（査読反映）→ B3（図）→ B2（演習）**。
2. **並行して前編担当**は軽量タスク：**A1（校閲留保の適用）✅** と **C2（frontmatter 準備）✅** は完了。次は **A3（局所整理）** または **C1（ドリフトチェック前編展開）**。
3. 大きな節目で **D1（フィデリティ・チェック）**。

## 注意（運用）

- **並行セッション**：前編担当と後編担当が同一リポジトリで並走中。コミットは **自分が触ったファイルを
  明示 add**（`git add <path>`、`-A` 禁止）し、相手の未コミット WIP を巻き込まない。
- セッション間の引き継ぎは `draft/_handoff-*.md` ／ `draft/_*-review.md` を“郵便受け”に。
- **前編には前編担当、後編には後編担当**が触れる（衝突回避）。明白な ERROR の越境修正のみ例外で、
  その場合は 1 ファイルだけ直して相手に通知。
