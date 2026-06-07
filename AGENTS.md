# AGENTS.md — chibirigor

chibirigor は **Rigor の教育用・最小版**です。実物の
[Rigor](https://github.com/)（Ruby 向けの漸進的型チェッカー）のアーキテクチャの「最小版」を、
ステップバイステップで作っていく教材リポジトリです。

## 最重要：Rigor と混同しない

- **ここは Rigor 本体ではありません。** Rigor の実装仕様 ― 豊富な型キャリア群、推論エンジン、
  RBS 連携、ADR、正規化規則、FactStore の 6 バケツ、HKT/`App`、性能最適化など ― を
  **持ち込まないでください。**
- chibirigor は*意図的に簡略化したモデル*です。Rigor の挙動を「正解」として chibirigor の
  コードを“直さ”ないでください。逆に、chibirigor の簡略モデルを Rigor の仕様と
  **取り違えないでください**。
- `docs/` に Rigor との対応レポートがありますが、それは「学習用の地図」であって移植指示では
  ありません。
- **「持ち込まない」と「参照しない」は別**：Rigor の*実装の複雑さ*は持ち込まない。しかし本文が
  「実 Rigor ではこうする（③ の段／「Rigor の中では」「実 Rigor では」）」と**事実を述べる箇所は、
  必ず実 Rigor で裏が取れていること**。簡略化は自由、事実の誤りはダメ。

## Rigor を「真実の源」として参照する（乖離防止）

chibirigor セッションは **chibirigor リポジトリから起動**し、この `AGENTS.md` が統治の主体です。
実 Rigor は **読み取り専用の参照先**として、次のチェックアウトを絶対パスで参照します（**編集禁止**）：

- **Rigor チェックアウト**：`/Users/megurine/repo/ruby/rigor`（read-only。chibirigor セッションから
  Rigor のファイルを変更しない）。

**乖離防止の規律**（本書は意図的な*簡略版*だが、Rigor についての*記述*は実態と一致させる）：

1. **「Rigor だと…／Rigor の中では…／実 Rigor では…」を書く・直すときは、その場で裏を取る。**
   トピック別の一次情報（すべて Rigor リポジトリ内）：
   - 思想・他ツール比較：`docs/handbook/`（特に `appendix-typeprof.md`／`appendix-steep.md` 等）
   - 型の意味論：`docs/type-specification/`（value-lattice / special-types / relations-and-certainty /
     control-flow-analysis / robustness-principle / normalization / rbs-erasure …）
   - エンジン内部の契約：`docs/internal-spec/`
   - 設計判断・なぜそうか：`docs/adr/`（`docs/adr/README.md` が索引）
   - 実コード：`lib/rigor/`
2. **意図的な簡略化は乖離ではない。** 各章の「続編に送ったもの」と `docs/` 設計ドラフトに記録済みの
   簡略化（局所 Scope vs FactStore、HKT 非実装、引数は untyped 等）は*正しい縮約*。**事故的な
   不正確さ**（実 Rigor が X なのに本書が ¬X と断言）だけを直す。両者を `docs/` の対応レポートで
   区別する。
3. **マイルストーンで「フィデリティ・チェック」**：本文の Rigor 主張を拾い、上記一次情報と突き合わせる
   サブエージェントを 1 本回し、ズレを `_*-review.md` に記録 → 軸を保って選択適用（過去に
   mametter レビューが `appendix-typeprof.md` を引いて TypeProf 不在を突いたのが好例）。
4. **対応レポート（`docs/20260607-type-systems-distilled-rigor-mapping.md` 等）を“台帳”として維持**：
   「本書のこの記述 ↔ Rigor のこの仕様/ADR/コード」を引けるようにしておく。

## 設計の鉄則（正は `docs/` の設計ドラフト）

- **とても易しく。** 1 step ＝ 難しいこと 1 つ。複数の難所を同時に持ち込まない。
- **誤検知を出さない**（"never frighten working code"）。わからない所は `untyped`（`Dynamic`）に
  逃がす。
- **拒まない**：Ruby が構文エラーにしないコードは受理する（パーサに Prism を使うので解釈できる
  範囲はさらに広い）。ただし「型が付く＝動く」は保証しない。
- **高度な内容は続編へ**：双方向型付けの形式化・変性・再帰型・本物の型推論・健全性理論は
  「The Seasoned chibirigor」に送る（本編に持ち込まない）。

## アーキテクチャ（最小版）

`lib/chibirigor/` … `type.rb`（型キャリア）/ `type_of.rb`（式→型の合成）/ `scope.rb` /
`evaluator.rb`（文を縫う）/ `dispatch.rb`（メソッド送信）/ `rbs.rb`（ミニ RBS 読み込み）/
`accepts.rb`（三値受理判定）/ `narrowing.rb` / `checker.rb` / `annotator.rb`、
`exe/chibirigor`（CLI）。各 Part が 1 ファイルを足す/育てる。

## 実行・テスト

```console
$ ruby exe/chibirigor check FILE      # 型診断
$ ruby exe/chibirigor annotate FILE   # 推論型の表示
$ ruby test/test_part1.rb             # 依存ゼロの plain Ruby テスト
```

Ruby 3.4 以降（Prism 同梱）。テストフレームワークは使わない。

## フォーマッタ注意

環境の rubocop autocorrect が `examples/*.rb` の「わざとおかしいコード」（未使用変数・型
エラーを含む例）を削除/書き換えます。対策：

- サンプルは rubocop-clean に保つ。
- 挙動の検証は `test/` の**文字列ソース**で行う（整形の影響外）。
- リテラル列の `annotate` デモは `/dev/stdin` パイプで見せる。

## 本文・ドキュメント

- **本文（チュートリアル）**：`draft/` 以下。二巻構成で前編 = **The Little chibirigor**
  （`draft/little/ja/`、ファーストドラフト・全 10 章 Part 0〜9）、後編 = **The Seasoned
  chibirigor**（`draft/seasoned/ja/`、着手・Part 1 ドラフト）。
- **設計・副読本対応**：`docs/`。
- いずれも Rigor リポジトリには置かない（分離を維持するため）。

## 用語・表記（ドキュメントサイト準拠）

本文の用語・表記は、配信先のドキュメントサイト
（`rigor.typedduck.fail`、最終的に `/chibirigor/` 配下へ submodule 統合）の表記に合わせる。
主な統制語（サイトの実表記に一致させること）。**注意：この節は誤った表記を「誤」列に*例として*
含むので、このファイルに一括置換（`perl -i s/旧語/新語/`）をかけてはいけない（誤列が壊れる）。**

| 概念 | 採用（正） | 誤（使わない） |
|---|---|---|
| プロジェクト名 | **Rigor**（大文字 R）／実行ファイルは `rigor` | 文中で `rigor`（固有名詞として） |
| 型を表すデータ | **キャリア** | 「カ」＋「リア」（誤った転写） |
| 型チェッカー／チェックする | **型チェッカー**／**型チェック** | 『チェッカー』『チェック』の漢字表記（旧称） |
| 型推論（の働き・エンジン） | **型推論**／**型推論エンジン** | （`chibirigor` 自身の呼称としての「型推論器」は可） |
| 絞り込み | **ナローイング**（動詞は「絞り込む」） | ― |
| 漸進的型付け | **漸進的型付け（gradual typing）**／文中の英語は `gradual` 可 | 「漸進的（gradual）型付け」（語を割らない） |
| 未知の型 | `untyped`／`Dynamic[Top]` | ― |
| 誤検知・原則 | **誤検知**／**ロバストネス原則** | ― |
| 構造的契約 | **RBS interface**／**構造的インターフェース**（Java 的誤読回避のため初出で明示） | 単に interface |
| 格子の両端 | `Top`／`Bot` | ― |
| 参照 | RBS（Ruby Signature）／Prism／Ruby | ― |

- 日本語の約物は全角（。、「」『』（））。コード／識別子はバッククォート。カタカナは
  借用テクニカル語（キャリア・ナローイング等）、漢字かなは native 概念（型・値・推論・絞り込む）。
- `chibirigor` 独自の最小キャリアは `Const[1]` 等のコード表記のままでよい（実装の都合）。本物の
  Rigor の表示を引くときは `Constant<3>` / `int<min, max>` / `Dynamic[Top]` のサイト記法に従う。
- **サイト統合の準備（将来）**：Astro + Starlight。各章 md には `title` / `description` /
  `sidebar.order` の frontmatter が要る（その際 `# 【ドラフト】…` の H1 は frontmatter `title` へ移す）。
  本文確定時にまとめて付与する。

## 次の作業はここから（レジューム）

次の作業候補・優先度・着手スライスは [`docs/CURRENT_WORKS.md`](docs/CURRENT_WORKS.md) に台帳化。
大きな区切りで更新する。

## レビューレンズのカタログ

§「Rigor を真実の源として参照する」のフィデリティ・チェックは**数あるレンズの一つ**。本書は
**複数の独立レビューレンズ**で磨いてきた（各結果は `draft/_*-review.md`、軸を保って選択適用）：

- **初学者の再現性**（最強の検証）：型理論ゼロ・Ruby 中級の読者を演じるサブエージェントが
  **本文だけ**から chibirigor を再実装し、リファレンス挙動で採点する。**答えは隠す**
  （`lib/`・`test/`・`docs/` を読ませない）。前編は 3 名が **34/34** を通過＝「推測ほぼゼロで
  本文だけから動くものを作れる」水準の証拠。新章・大改訂のたびに 1 本回すと劣化を防げる。
- **型理論エキスパート** … 形式的正確さ（`rigor/docs/` で裏取り）。
- **技術書編集者** … 構成・学習設計・刊行完成度。
- **ドメイン著者** … 例：mametter（TypeProf 作者＋『しくみ』著者）― ツール設計差と引用の公正さ。
- **日本語校正・校閲** … 表現の質のみ。定着技術用語（例外を投げる等）には踏み込ませない。

新レンズも同型：①読ませる範囲を絞る（答えを隠す）②重大度を付けさせる ③`_*-review.md` に記録
④軸を保って選択適用。

## 清書フォーマット（前編 Part 1〜9 で確定。後編もこれに揃える）

- H1 は `# The Little/Seasoned chibirigor Part N ― <タイトル>`（**`【試し書き】`/`【ドラフト】`
  マーカーは除去**）。
- 章は「**この章のゴール → 本文（三題噺）→ まとめ表 → `## 演習` → 次章予告**」で閉じる。
- 章末の `> 検証メモ`（執筆者向け足場）は刊行時に除去（git 履歴に残る）。読者向けの**参考書メモ**
  （『しくみ』/TAPL）は残す。
- **framing の不可逆点（後退させない）**：chibirigor は「**推論を土台にした検査器**」。やるのは
  *式から型を組み立てる合成*であって、*呼び出し元から引数を逆算する*ことはしない（それは
  TypeProf の仕事＝本書でも明示済み）。「推論器 vs チェッカー」「check は副産物」とは言わない
  （`draft/_mametter-review.md` 反映点）。

## 並行セッションと引き継ぎ

- 前編担当と後編担当が**同一リポジトリで並走**しうる。コミットは**自分が触ったファイルを明示 add**
  （`git add <path>`、`-A` 禁止）。相手の未コミット WIP を巻き込まない。
- **前編には前編担当、後編には後編担当**が触れる。明白な ERROR の越境修正のみ例外（1 ファイルだけ
  直して相手に通知）。
- セッション間の郵便受け：`draft/_handoff-*.md`／`draft/_*-review.md`。
