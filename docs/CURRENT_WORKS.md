# CURRENT_WORKS — 次の作業候補（レジューム・ブックマーク）

> 次の担当（人／サブエージェント）が**ここから再開**するための作業台帳。統治の軸は
> [`../AGENTS.md`](../AGENTS.md)。本書は意図的な*簡略版*だが Rigor についての*記述*は実態と
> 一致させること（AGENTS.md §「Rigor を真実の源として参照する」）。
> 一過性のメモ ― 大きな区切りで更新する。

## 現状（2026-06-13（lib 先行 2 機能の本文反映）更新）

### 2026-06-11〜13 の進捗（台帳追記）
- **フルサイクル査読（真→伝→読→整・10 レンズ）反映済み**（947db6c）。型名前空間の逆流ブロッカー解消
  （`Type::` 正規形に統一）。残課題＝`book/v1/ja/_handoff-state.md` の著者裁量バックログ。
- **lib に 2 機能追加→本文反映済み（2026-06-13）**：
  - **Union ディスパッチ**（51f299b：レシーバ分配＋畳み込み段の直積展開）→ 前編 part4「4-1x」発展ノート。
  - **trace コマンド**（19c182d：推論イベントのコマ送り再生・`tracer.rb`）→ 付録 a3「a3-3b/a3-3bx」。
    実 Rigor の `rigor trace` は manual で実在確認済み。
- **整/読バックログ一部消化**：三題噺フル定義を Part 0 へ一本化／chibivue 比喩の回収（書評 M2）／
  **Rigor 来歴ボックスを Part 0 に追加（書評 M1・著者一次記述）**。
- **辛口 A-2／A-1 消化（2026-06-13）**：Part 0 を「本編で中心に作るのは 2 つ（`check`／`annotate`）」と
  限定し、`--explain`/`--unreachable` は付録の opt-in 発展と明示。**`type-of` は著者判断で完全廃止**
  （実装 `type_at.rb`・`exe type-of`・`test_type_at.rb`、本文の極小版 a3-2x をすべて除去）。付録 a3-2 は
  実 Rigor の道具紹介として残置。A-1（Part 5）は L3 反映（947db6c）で既に opt-in 明記済み。
- **整バックログ消化（2026-06-13）**：
  - **用語集を二層化**（前編で出会う語／後編・Rigor 固有の語）。`Difference` 項の渋滞を語源＋a2-6 ポインタへ圧縮。
  - **演習解答導線（軽量案）**：Part0 §0.5 に答え合わせ方針を一度だけ明記、各章末「この章の実装」リンクを
    「（演習の答え合わせにも）」へ拡張し参照実装との突き合わせ導線に兼用。
- **再現性レンズ再走（2026-06-13）**：再現役 2 名が本文だけから独立再実装。**核の再現性は実質 41/41＝
  大改訂による劣化なし**（`type-of` 廃止・Part0 限定句・用語集二層化とも詰まりを生まず）。生スコア
  38/41 の外し 3 項目は両名一致の定数畳み込みで、本文は「丸める」と明記＝Part2§2-7 発展ノート＝
  ハーネス側の過剰採点（本の欠陥でない）。唯一の共通 FRICTION（Part4 `node.subsequent` の ElseNode 非明示）に
  散文ノートを適用。所見＝`book/v1/ja/.reviews/_reproducibility-review.md`。
- **図版バックログ完了（2026-06-13）**：Claude Design 作成の図 4-1（then/else→union）・6-1（open shape）・
  7-1（accepts 三値＋Union 畳み込み）を `figures/svg/little-{4,6,7}-1.svg` に追加、各章（§4-1・§6-3・§7-3a 後）へ配線。
  仕様書＝`book/v1/ja/figures/_drafts/README.md`。XML 整形式・CSS 変数既知・ラベル本文一致・色の意味づけ 3 図一貫を確認。
  → 前編の図は 0/2/4/5/6/7 をカバー（概念ヤマの偏在解消）。デザイナーによる本清書は将来別途。
- **L4 整レイヤー再走・反映（2026-06-13）**：編集者＋校閲を並列。**刊行を妨げる構成欠陥ゼロ**。
  反映＝校閲ERROR「differ 置換」(part8×3)を平易日本語へ／part0 §0.4 係り受け・来歴の敬体統一／
  README 目次の a3 を trace 反映＋type-of を実Rigor専用と明示／Part9 末から a3 trace へ戻りポインタ。
  所見全文＝`.reviews/_editorial-review.md`・`_copyedit-review.md`。
- **type-of をユーザー向けに見せない＋dump_type を導入（2026-06-13・著者方針）**：正確には「引退」ではなく、
  `rigor type-of` は **hover/エディタ/MCP 等ツールが使う低レベル API**であり、**ユーザー/学習者には
  コマンドとして見せない**のが正しい位置づけ。よって本書の公開テキストから CLI コマンド `type-of`
  （ハイフン）を除去（a3-2 を「Rigor の型表示」に改題、Part1 コラム・README・a3-4 早見表を差替）。
  - **ユーザー向けの型確認手段**＝① `annotate`（ファイル全体）② **`dump_type(式)` を書いて `check`**
    （`include Rigor::Testing`／実行時は値をそのまま返す／`check` が `:info` で `dump_type: <型>` を印字。
    PHPStan の `dumpType()` 相当。本書 §a3-1 `--explain` と同じ `:info` 機構）。a3-2 に追記済み。
  - 中核**関数** `type_of`（アンダースコア）は本書の心臓につき維持。フィデリティ：`dump_type` は実在
    （`rigor/lib/rigor/testing.rb`・`analysis/check_rules.rb` で裏取り済み）。type-of も内部 API として残存
    （消える話ではない）。Rigor 側 manual がユーザー露出を絞れば本書と完全一致。
- **残る任意バックログ（nitpick・著者裁量）**：三題噺②ラベルの本文⇔表ゆれ（Ruby だと/Ruby・RBS）、
  part1 初出での型名前空間予告（技術判断含み・著者留保）、図キャプションのコードスパン体裁。
  デザイナーによる図の本清書は将来別途。

### ★本線：v1 清書版 `book/v1/ja/`（draft の外・リポジトリ直下）
原稿 `draft/` を**二巻維持・大胆再構成**で再編した清書ツリー。**ここが今後の編集対象**。
- 方針＝[`../book/v1/ja/_reorg-proposal.md`](../book/v1/ja/_reorg-proposal.md)、移植の正典（相互参照リマップ・抜き差し）＝[`../book/v1/ja/_migration-guide.md`](../book/v1/ja/_migration-guide.md)。
- **前編 `little/` Part 0–9**：Part1 純化／旧 Part4 を Union(4)・ナローイング(5)に分割／旧 Part7+8 を RBS と型シグネチャ(8)に統合／baseline・特別な型総括を Part9 へ集約。
- **後編 `seasoned/` Part 1–8**：章順を新順序（1,2,旧6→3,4,旧3→5,旧5→6,7,8）へ組み替え（構造→推論/フロー→理論の頂点→橋）／gradual の2規律と停止の工学を Part7 集約／Part1↔Part7 ブックエンド。
- 横断トピックを**付録 a1–a4**（特別な型／ナローイング・パターン集／道具／参考書・ADR）へ括り出し。README・両巻 README・用語集を v1 番号で整備。
- **検証済み**：内部リンク全 69 解決・後編ドリフトチェッカー緑（8章/5examples）。
- **6レンズ査読1巡＋バックログ整理 済み**（2026-06-08〜09。編集者・型理論・フィデリティ・校閲・Java/Ruby読者）。所見＝`book/v1/ja/_*-review.md`。型理論・フィデリティ両レンズが「公開水準」と評価。
- **図版 8 点配置済み**（2026-06-09）：`book/v1/ja/figures/svg/`（日本語依存・v1 完結）。
- **`examples/` 単体完結**（2026-06-09）：`book/v1/ja/little/examples/dist/` に段別 lib を同梱。ディレクトリをコピペするだけで動作する。

### ★新インフラ：段スナップショット `impls/`（2026-06-09 試作）
chibivue [`book/impls`](https://github.com/chibivue-land/chibivue/tree/main/book/impls) 風に各 Part の到達状態を完全ツリーで展開。**手コピーでなく単一ソース→生成物→検証ゲート**（詳細 `impls/README.md`、方針 `ROADMAP.md`「本のインフラ」）。
- `impls/steps/partN/`（源：変わったファイルだけ＋`test_stage.rb`）→ `tools/gen_impls.rb` が前方 compose → `impls/dist/partN/`（生成物・手編集禁止）。
- `make impls` / `impls-verify`（段テスト）/ `impls-check`（手編集検出）。`make all` 同梱。
- **Part 1–9 すべて充填済み**（2026-06-09）。`make impls-verify` で全段グリーン。最終段 dist/part9 は lib/ と機能的に収束。

### lib 実装 `lib/chibirigor/`（Part 1–9・完成形）
- `test/test_part1〜9.rb` 緑・CLI 動作。ROADMAP フェーズ1–2 実装済み。
- **2026-06-10：Rigor 由来 5 機能を追加**（計画＝[`../docs/20260610-v1-rigor-features-plan.md`](../docs/20260610-v1-rigor-features-plan.md)）。
  すべて発展ノート方式で本文へ重ねた。`make test`/`drift`/`impls-verify` 緑。
  1. **type-of 位置クエリ**（`type_at.rb`／`exe type-of FILE:LINE:COL`／`test_type_at.rb`）→ 付録 a3-2x。
  2. **check --explain**（fail-soft 地図。`dispatch.rb`＋`checker.rb`／`test_explain.rb`）→ 付録 a3-1x・前編 P9。
  3. **到達不能アーム診断**（ADR-47 縮小版・opt-in `check --unreachable`。`narrowing.rb`＋`type_of.rb`／
     `test_unreachable.rb`）→ 付録 a1-3x・a5-5。健全性のため「閉じた既知型＋互いに素な葉クラス」限定。
  4. **ジェネリクス 5a（要素型の読み）**（`type_of.rb` の `element_read`／`test_generics.rb`）→ 後編 P3「3-6x」。
  5. **non-empty-array ブリッジ**（本文のみ。`Tuple`＝事実上の非空配列）→ 付録 a2-6x。
- **2026-06-10（続き）：5b＋5c も実装**（`type_of_block`／新カリア `Type::Generic`）。
  `map { |x| ... }` の `x` を要素型に束縛＋本体を型チェック、`map`→`Array[本体型]`。直接代入で実装
  （単一化は未知型変数の一般ケース用に概念のまま）。本文＝後編 Part 5「5-6x」・Part 3「3-6x」更新。
  → **generics が lib で一本につながった（本のクライマックス到達）**。
- 補足：`Makefile` の `test` ターゲットを `test/test_*.rb` glob に変更（従来 `test_return_type_check.rb` が未実行だった）。

### draft（原稿・legacy 素材）
- `draft/little`・`draft/seasoned`・`draft/preface.md`・`draft/glossary.md` は **v1 の移植元**。
  v1 安定まで残置（撤去判断は V4）。`draft/_*-review.md` は draft 期の査読（現行は `book/v1/ja/_*-review.md`）。

## 次セッションの作業候補（優先度つき）

### Track V — v1 仕上げ（本線）
- **V1（中）本文↔スナップショット連携**：基盤は 2026-06-09 に初期実装済み。`book/v1/ja/little/examples/` に段別 example（`dist/partN/lib` ロード）を整備、`check_docs.rb` で `<!-- run: -->` / `<!-- include: -->` を照合。**Part1 annotate デモ（`3: Integer`）を配線済み**。残課題：Part3–9 章の出力ブロックを `run:` / `include:` で配線（章の散文コードブロックは教材的に簡略のため `include:` 非対象）。
- **V2 ✅ 暫定完了（2026-06-09）図版配置**：8点を `book/v1/ja/figures/svg/` に配置済み。デザイナーによる本清書は別途。
- **V3 ✅ 完了（2026-06-09）再現性レンズを v1 で1巡**：型知識ゼロ読者が `book/v1/ja/little` だけで再実装→**39/39 満点**（採点項目を 34→39 に増強）。推測 2 件（G1: Part5 is_a? 偽の枝、G2: Part7 expected-is-Union ルール）いずれも nitpick 相当。所見 `book/v1/ja/_reproducibility-review.md`。
- **V4（小・要判断）旧 `draft/` 撤去**：保留中。v1 安定を待って判断。
- **V5（任意・大）英語版 `book/v1/en/`**：TAPL を共通参照に（『しくみ』は日本語のみ）。

### Track I — 段スナップショット拡張
- **I1 ✅ 完了（2026-06-09）**：Part3–9 の steps 充填・全段 test_stage.rb 緑・dist/part9 = lib/ に機能収束。
- **I2 ✅ 完了（2026-06-09）**：`.github/workflows/ci.yml`（`make test` + `make impls-check`）を追加。
- **I3（小）V1 と一体**：本文 include 連携（上記 V1）。

### Track L — lib 機能（ROADMAP フェーズ3）
- **L1（L・bounded）ジェネリクス／要素型 ✅ 5a/5b/5c 実装済み（2026-06-10）**：
  `element_read`（読み）＋`type_of_block`（押し下げ・`map`→`Array[Elem]`）。generics が lib で一本に
  つながった＝**本のクライマックス到達**。残る発展：要素が未知型変数のときの本格単一化
  （`unification.rb` 昇格）・bounded 量化・変性 ― demand 次第（当面は概念のまま）。

### Track S — サイト統合
- **S1 移譲済み**：Astro + Starlight 統合は `/Users/megurine/repo/site/rigor.typedduck.fail` へ移譲。

## レビュー資産（v1・未適用の指摘を引ける）

| ファイル | レンズ | 主な未適用（任意・軸を保って選択適用） |
|---|---|---|
| [`../book/v1/ja/_reproducibility-review.md`](../book/v1/ja/_reproducibility-review.md) | 再現性 | G1(Part5 is_a? 偽枝 1文追加)・G2(Part7 expected-Union ルール明示) いずれも nitpick ― **適用済み**（2026-06-09） |
| [`../book/v1/ja/_editorial-review.md`](../book/v1/ja/_editorial-review.md) | 技術書編集者 | 図版（V2）／後編 README↔§1-6 地図の二重（意図的残置） |
| [`../book/v1/ja/_expert-review.md`](../book/v1/ja/_expert-review.md) | 型理論 | ERROR/REF は反映済み。nitpick 数件（occurs check 脚注 等） |
| [`../book/v1/ja/_fidelity-review.md`](../book/v1/ja/_fidelity-review.md) | フィデリティ | 2 件反映済み。次マイルストーンで再チェック |
| [`../book/v1/ja/_copyedit-review.md`](../book/v1/ja/_copyedit-review.md) | 日本語校閲 | 巻呼称「続編/後編」のゆれ（定型見出しとして残置中）／統制語（robustness/きれい 等）の寄せ |
| [`../book/v1/ja/_java-reader-review.md`](../book/v1/ja/_java-reader-review.md) | Java 中堅読者 | 高一致 FRICTION は反映済み。残 FRICTION 数件（P7 Union 引数の限定明示 等） |
| [`../book/v1/ja/_ruby-reader-review.md`](../book/v1/ja/_ruby-reader-review.md) | Ruby ジュニア読者 | 高一致 FRICTION は反映済み。残 FRICTION 数件（Part3 不変 Scope の動機 等） |

## 着手の入口（推奨スライス）

1. **V1 続き（本文↔スナップショット連携の完成）** → Part3–9 章の出力ブロックに `run:` を配線。各 `partN.rb` に region を足して `<!-- include: -->` も順次。
2. または **V3（再現性レンズ）** で大改訂後の本丸検証（再現性は最強の品質ゲート・今回未実施）。
3. lib を進めるなら **L1（generics 5a から）**。
4. 大きな節目で **フィデリティ再チェック**（`book/v1/ja/_fidelity-review.md` を更新）。

## 注意（運用）

- **v1 が本線**。`draft/` は移植元（v1 安定まで残置）。`impls/dist/` は**生成物（手編集禁止）** ― 直すのは `impls/steps/` で、`make impls` 後に `make impls-check` で同期確認。
- コミットは **`git add <個別ファイル>`**（`-A` 禁止。生成物 `impls/dist` を意図せず巻き込まない）。
- セッション間の引き継ぎは本ファイル ＋ `book/v1/ja/_*-review.md`（査読台帳）を“郵便受け”に。
- 用語・表記は `AGENTS.md` の統制語（キャリア／型チェッカー／ナローイング 等）。**Rigor についての記述は実態と一致**（フィデリティ）。
