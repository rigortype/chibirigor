# CURRENT_WORKS — 次の作業候補（レジューム・ブックマーク）

> 次の担当（人／サブエージェント）が**ここから再開**するための作業台帳。統治の軸は
> [`../AGENTS.md`](../AGENTS.md)。本書は意図的な*簡略版*だが Rigor についての*記述*は実態と
> 一致させること（AGENTS.md §「Rigor を真実の源として参照する」）。
> 一過性のメモ ― 大きな区切りで更新する。

## 現状（ship-readiness）

- **v1 清書版 = `v1/ja/`（draft の外）**：原稿 `draft/` を**二巻維持・大胆再構成**で再編した清書ツリー（2026-06-08 着手）。
  - 方針・対応表は [`../v1/ja/_reorg-proposal.md`](../v1/ja/_reorg-proposal.md)、移植の正典（相互参照リマップ・抜き差し）は
    [`../v1/ja/_migration-guide.md`](../v1/ja/_migration-guide.md)。
  - **前編 little/ Part 0–9**：Part1 を純化、旧 Part4 を Union(4)/ナローイング(5)に分割、旧 Part7+8 を RBS と型シグネチャ(8)に統合、
    baseline と特別な型総括を Part9 へ集約。**後編 seasoned/ Part 1–8**：章順を新順序（1,2,旧6→3,4,旧3→5,旧5→6,7,8）へ組み替え、
    gradual の2規律と停止の工学を Part7 集約、Part1↔Part7 ブックエンド。
  - 横断トピックを**付録 a1–a4**（特別な型／ナローイング・パターン集／道具／参考書・ADR）へ括り出し。README・両巻 README・用語集を v1 番号で整備。
  - 検証：内部リンク全 69 解決、後編ドリフトチェッカー緑（8章/5examples）。
  - **6レンズ査読1巡 実施済み（2026-06-08）**：編集者・型理論・フィデリティ・校閲・Java読者・Ruby読者。
    各所見は `v1/ja/_*-review.md`。必要・軸保持の修正のみ反映（残骸タグ除去・後編Part2予告修正・腑に落とす・
    refinement carrier 二層注記・読者高一致の一文×5・用語集初出タグ）。
    **バックログ整理 実施済み（2026-06-09）**：C1（Part5 演習1 をナローイング設問へ差し替え重複解消）・
    C4（PHPStan 表を a2-6 に一元化・用語集はポインタ）・C5（Part6 冒頭の反復圧縮）・「地に足」比喩 5 箇所を平易化。
    C3（Part4/7 fuel/三値表）は移植時に Part4→Part7 委譲済み・三値表は HKT 固有のため対象外と判断。
    **残（任意）**：前編3図の本清書（デザイナー領域）／後編README↔§1-6 地図の二重（役割が異なるため意図的に残置）。
  - **未了**：再現性レンズ（教えるコード不変のため今回省略）／前編 examples ドリフト配線は要再設計
    （Part1「丸めて Integer」と畳み込み例の意図的乖離のため）。
  - 旧 `draft/little`・`draft/seasoned` は v1 安定まで残置（撤去は別判断）。
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
- **A4（任意）** ✅ 『しくみ』/TAPL 対応早見表を Part 0 §0.4 末尾に組み込み済み（2026-06-08）。

### Track B — 後編 拡充（主トラック・別セッション）
- **B1（中）** ✅ 専門査読の反映済み（2026-06-08）：P1「合成は失敗しない」→ chibirigor 固有限定、
  P2「基底は不変(variance)」→「非自明な部分型なし」コメント修正、P4 §4-4 余帰納 `seen` 簡約性注記、
  P6 §6-2 シャドーイング 2 段階明示、P2 §2-5 三値表実装注、P4/P8 TAPL 29 章参照、
  P5 FactStore `dynamic_origin` の位置づけ脚注、P3 TAPL 22 章 let 多相注記、Ω 単純型不能説明 ―
  いずれも現行テキストで対応済みを確認。nitpick（巻名表記揺れ・k≥0 明記）も解決済み。
- **B2（中）** 各章末に `## 演習`（**前編フォーマットをミラー**。後編は証明・トレース・規則導出系）。
- **B3（中・デザイナー）** 図 5 枚：双方向 `⇒`/`⇐`・部分型格子・変性 S-Arrow・FactStore 6 バケツ・μ 畳/展開。
- **B4（小）** ✅ TypeProf 対比を Part 3 §3-4a に追加済み（2026-06-08）：whole-program 抽象解釈 vs
  local+catalog の対比表・mametter の指摘を反映。出典：`rigor/docs/handbook/appendix-typeprof.md`。
- **B5（小）** ✅ 健全性の回収：後編 Part 7 §7-4 に 4 箇所×progress/preservation 対応表あり。前編 Part 9 §9-2
  との相互参照も両側に存在（確認・補強 2026-06-08）。
- **B6（小）** ✅ 最終章 Part 8 §8-2・§8-3 に具体 ADR 番号付き参照表あり（2026-06-08 補強）。
  ADR-0・ADR-4 の入口案内を §8-3 冒頭に追加済み。
- **B7（小）** ✅ スケッチ整合確認済み（2026-06-08）：前編・後編とも `check_docs.rb` がグリーン。

### Track C — 横断・インフラ
- **C1** ✅ ドリフトチェックを前編に展開済み。`draft/little/ja/examples/check_docs.rb`（examples 3 本）＋ `Makefile`（`make` で両巻チェック）。`<!-- run: -->` ディレクティブ対応済み、`<!-- include: #region -->` も有効（パス汎用化済み）。Part1 の annotate 出力ドリフト（Integer→3）を検知・修正済み。
- **C2** ✅ サイト統合準備（Astro + Starlight）：前編 10 章・後編 8 章に frontmatter 追加済み（`title`/`description`/`sidebar.order`、後編は `draft: true`）。H1 はそのまま残存。
- **C3** 用語集（`draft/glossary.md`）の維持：後編で形式用語を導入したら初出章つきで追記。
- **C5** ✅ appendix・handbook コンテンツ盛り込み完了（2026-06-08）：
  - appendix 12 ファイルから★★★（3 件）・★★（3 件）・★（3 件）を抽出し各章に挿入。
  - handbook ch.01–12 から 16 トピックを抽出し優先順で挿入完了：
    A（Difference 型）・G（再代入でナローイングリセット）・F（Union サイズ予算→強制 wide）・
    B（dispatch 5 段カスケード）・P（filter_map が Tuple を wide）・
    I（`rigor check --explain`）・M（`rigor type-of file:line:col`）・
    D（エスケープブロックでナローイング消滅）・E（`&&` チェーンで事実積み上げ）・
    H（正規表現名前付きキャプチャ→ String）・C（HKT 条件型も三値）・
    J（`param:` 2 効果）・K（`partial_of[T]` 値型を nil に広げない）・
    L（`JSON.parse symbolize_names: true` → HKT 型引数）・
    N（ivar = 全代入の union）・O（`assert: self is T` 手動事実挿入）・
    Q（Sorbet `T.assert_type!` と gradual consistency）。
- **C4（任意）** 英語版 `draft/little/en/` の起こし（TAPL を共通参照に。『しくみ』は日本語のみ）。

### Track D — フィデリティ維持（AGENTS.md §「Rigor を真実の源に」）
- **D1** ✅ フィデリティ・チェック実施済み（2026-06-08）：`draft/_fidelity-review.md` に記録。
  重大ズレ 3 件を修正済み（baseline 照合キー・Nominal 型引数変性・推論予算実装状況）。
  次のマイルストーンで再実施。

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
