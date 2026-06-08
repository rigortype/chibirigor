# CURRENT_WORKS — 次の作業候補（レジューム・ブックマーク）

> 次の担当（人／サブエージェント）が**ここから再開**するための作業台帳。統治の軸は
> [`../AGENTS.md`](../AGENTS.md)。本書は意図的な*簡略版*だが Rigor についての*記述*は実態と
> 一致させること（AGENTS.md §「Rigor を真実の源として参照する」）。
> 一過性のメモ ― 大きな区切りで更新する。

## 現状（2026-06-09 更新）

### ★本線：v1 清書版 `v1/ja/`（draft の外・リポジトリ直下）
原稿 `draft/` を**二巻維持・大胆再構成**で再編した清書ツリー。**ここが今後の編集対象**。
- 方針＝[`../v1/ja/_reorg-proposal.md`](../v1/ja/_reorg-proposal.md)、移植の正典（相互参照リマップ・抜き差し）＝[`../v1/ja/_migration-guide.md`](../v1/ja/_migration-guide.md)。
- **前編 `little/` Part 0–9**：Part1 純化／旧 Part4 を Union(4)・ナローイング(5)に分割／旧 Part7+8 を RBS と型シグネチャ(8)に統合／baseline・特別な型総括を Part9 へ集約。
- **後編 `seasoned/` Part 1–8**：章順を新順序（1,2,旧6→3,4,旧3→5,旧5→6,7,8）へ組み替え（構造→推論/フロー→理論の頂点→橋）／gradual の2規律と停止の工学を Part7 集約／Part1↔Part7 ブックエンド。
- 横断トピックを**付録 a1–a4**（特別な型／ナローイング・パターン集／道具／参考書・ADR）へ括り出し。README・両巻 README・用語集を v1 番号で整備。
- **検証済み**：内部リンク全 69 解決・後編ドリフトチェッカー緑（8章/5examples）。
- **6レンズ査読1巡＋バックログ整理 済み**（2026-06-08〜09。編集者・型理論・フィデリティ・校閲・Java/Ruby読者）。所見＝`v1/ja/_*-review.md`。型理論・フィデリティ両レンズが「公開水準」と評価。

### ★新インフラ：段スナップショット `impls/`（2026-06-09 試作）
chibivue [`book/impls`](https://github.com/chibivue-land/chibivue/tree/main/book/impls) 風に各 Part の到達状態を完全ツリーで展開。**手コピーでなく単一ソース→生成物→検証ゲート**（詳細 `impls/README.md`、方針 `ROADMAP.md`「本のインフラ」）。
- `impls/steps/partN/`（源：変わったファイルだけ＋`test_stage.rb`）→ `tools/gen_impls.rb` が前方 compose → `impls/dist/partN/`（生成物・手編集禁止）。
- `make impls` / `impls-verify`（段テスト）/ `impls-check`（手編集検出）。`make all` 同梱。
- **Part 1–9 すべて充填済み**（2026-06-09）。`make impls-verify` で全段グリーン。最終段 dist/part9 は lib/ と機能的に収束。

### lib 実装 `lib/chibirigor/`（Part 1–9・完成形）
- `test/test_part1〜9.rb` 緑・CLI 動作。ROADMAP フェーズ1–2 実装済み、**フェーズ3＝ジェネリクスのみ未着手**。

### draft（原稿・legacy 素材）
- `draft/little`・`draft/seasoned`・`draft/preface.md`・`draft/glossary.md` は **v1 の移植元**。
  v1 安定まで残置（撤去判断は V4）。`draft/_*-review.md` は draft 期の査読（現行は `v1/ja/_*-review.md`）。

## 次セッションの作業候補（優先度つき）

### Track V — v1 仕上げ（本線）
- **V1（中）本文↔スナップショット連携**：v1 章のコードブロックを `<!-- include: ../../impls/dist/partN/...#region -->` で**スナップショットから直接引く**。本文↔スナップショット↔段テストが一源化しドリフトが原理的に消える。**前編 examples ドリフト配線の再設計もここで**（v1 Part1 は「丸めて Integer」、畳み込み例は実 lib で `3`＝意図的乖離。段の lib に寄せて「章の挙動＝段の lib」で検証）。
- **V2（中・デザイナー）図版の本清書**：前編3図（0-1/2-1/4-1 相当）＋後編5図（双方向 ⇒/⇐・部分型格子・変性 S-Arrow・FactStore 6 バケツ・μ 畳/展開）。編集レンズ E 級・後編 B3。
- **V3（中）再現性レンズを v1 で1巡**：型知識ゼロ読者が `v1/ja/little` だけで再実装→34項目採点（`lib/`・`impls/`・`test/` は隠す）。大改訂後の本丸検証で今回未実施。
- **V4（小・要判断）旧 `draft/` 撤去**：v1 全章が安定したら `draft/little`・`draft/seasoned` を撤去判断。
- **V5（任意・大）英語版 `v1/en/`**：TAPL を共通参照に（『しくみ』は日本語のみ）。

### Track I — 段スナップショット拡張
- **I1 ✅ 完了（2026-06-09）**：Part3–9 の steps 充填・全段 test_stage.rb 緑・dist/part9 = lib/ に機能収束。
- **I2（小）CI ゲート**：`make impls-check` を GitHub Actions に（dist 手編集を弾く）。
- **I3（小）V1 と一体**：本文 include 連携（上記 V1）。

### Track L — lib 機能（ROADMAP フェーズ3）
- **L1（L・bounded）ジェネリクス／要素型**：後編 `unification.rb` を本実装へ昇格。**5a 要素型の読み → 5b ブロック仮引数へ ⇐ → 5c 戻り多相**（各段出荷可能・予算を脅かしたら止める）。詳細 `ROADMAP.md` §5。本のクライマックス（generics が lib で一本につながる）。

### Track S — サイト統合
- **S1（中）Astro + Starlight**：v1 章の frontmatter（`title`/`description`/`sidebar.order`）採番を確認し、配信サイト（`rigor.typedduck.fail` の `/chibirigor/`）へ submodule 統合。

## レビュー資産（v1・未適用の指摘を引ける）

| ファイル | レンズ | 主な未適用（任意・軸を保って選択適用） |
|---|---|---|
| [`../v1/ja/_editorial-review.md`](../v1/ja/_editorial-review.md) | 技術書編集者 | 図版（V2）／後編 README↔§1-6 地図の二重（意図的残置） |
| [`../v1/ja/_expert-review.md`](../v1/ja/_expert-review.md) | 型理論 | ERROR/REF は反映済み。nitpick 数件（occurs check 脚注 等） |
| [`../v1/ja/_fidelity-review.md`](../v1/ja/_fidelity-review.md) | フィデリティ | 2 件反映済み。次マイルストーンで再チェック |
| [`../v1/ja/_copyedit-review.md`](../v1/ja/_copyedit-review.md) | 日本語校閲 | 巻呼称「続編/後編」のゆれ（定型見出しとして残置中）／統制語（robustness/きれい 等）の寄せ |
| [`../v1/ja/_java-reader-review.md`](../v1/ja/_java-reader-review.md) | Java 中堅読者 | 高一致 FRICTION は反映済み。残 FRICTION 数件（P7 Union 引数の限定明示 等） |
| [`../v1/ja/_ruby-reader-review.md`](../v1/ja/_ruby-reader-review.md) | Ruby ジュニア読者 | 高一致 FRICTION は反映済み。残 FRICTION 数件（Part3 不変 Scope の動機 等） |

## 着手の入口（推奨スライス）

1. **V1＋I3（本文↔スナップショット連携）** → ドリフトを原理的に消す。続けて **I1（Part3–9 steps）** で最終段を `lib/` に収束。
2. または **V3（再現性レンズ）** で大改訂後の本丸検証（再現性は最強の品質ゲート・今回未実施）。
3. lib を進めるなら **L1（generics 5a から）**。
4. 大きな節目で **フィデリティ再チェック**（`v1/ja/_fidelity-review.md` を更新）。

## 注意（運用）

- **v1 が本線**。`draft/` は移植元（v1 安定まで残置）。`impls/dist/` は**生成物（手編集禁止）** ― 直すのは `impls/steps/` で、`make impls` 後に `make impls-check` で同期確認。
- コミットは **`git add <個別ファイル>`**（`-A` 禁止。生成物 `impls/dist` を意図せず巻き込まない）。
- セッション間の引き継ぎは本ファイル ＋ `v1/ja/_*-review.md`（査読台帳）を“郵便受け”に。
- 用語・表記は `AGENTS.md` の統制語（キャリア／型チェッカー／ナローイング 等）。**Rigor についての記述は実態と一致**（フィデリティ）。
