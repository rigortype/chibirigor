---
name: chibirigor-review
description: Run the multi-lens review battery on the chibirigor book draft. Spawns independent-context reviewer subagents — reproducibility (a no-type-theory reader implements from the prose and is graded), type-theory expert, technical-book editor, domain author (mametter = 『しくみ』 author + TypeProf dev), Japanese copyeditor, Rigor-fidelity, and a Java mid-level-engineer reader (no type-theory background, vague on generics, flags explanatory leaps from a static-typed-language reader's seat) — records each lens to a draft/_<lens>-review.md note, then synthesizes only the necessary, axis-preserving fixes. Use when asked to "review chibirigor", "run the review lenses", "查読して/校閲して", validate a chapter or volume before a milestone, or check the book's faithfulness to real Rigor. Not for tiny single edits.
---

# chibirigor-review — 多観点レビュー・バッテリー

chibirigor（最小の Ruby 型推論ベース検査器を作りながら本物の Rigor を学ぶ二巻本）の本文を、
**独立コンテキストのサブエージェント**で複数の専門レンズから査読し、各結果をノートに記録して、
**軸を保った必要な修正だけ**を選択適用するための手順を凍結したスキル。

このスキルは「品質ゲートの方法論」そのものです。個々の編集ではなく、**章/巻の節目**で回す。

## いつ使う / 使わない

- **使う**：章や巻を一区切りまで書いた／刊行前点検／「Rigor の実態と乖離していないか」確認／
  ユーザーが「査読して」「校閲して」「review chibirigor」。
- **使わない**：一文の手直しや明白な typo。レンズ起動のオーバーヘッドに見合わない。

## 7 つのレンズ

各レンズは**独立した新規コンテキストのサブエージェント**（Agent ツール、`general-purpose`）として
起動する。互いに依存しないので**並列で投げてよい**。`opus` を既定（再現性・Java 読者レンズは `sonnet` 可）。

| # | レンズ | 主眼 | 出力ノート |
|---|---|---|---|
| 1 | 再現性（reproducibility） | 型知識ゼロの読者が本文だけで再実装できるか＋挙動採点 | （実装は `/tmp`、所見は本文へ） |
| 2 | 型理論エキスパート | 形式的・技術的正確さ | `draft/_expert-review-findings.md` |
| 3 | 技術書編集者 | 構成・学習設計・刊行完成度 | `draft/_editorial-review.md` |
| 4 | ドメイン著者（mametter） | 概念の位置づけ・自著引用の公正さ・TypeProf 対比 | `draft/_mametter-review.md` |
| 5 | 日本語校正・校閲 | 言語の質（慣用句誤用・AI 調・表記ゆれ） | `draft/_copyedit-review.md` |
| 6 | Rigor フィデリティ | 「実 Rigor では…」の記述が実態と一致するか | `draft/_fidelity-review.md` |
| 7 | Java 中堅読者（説明の飛躍） | 型理論ゼロ・静的型付け経験ありの読者に飛躍がないか | `draft/_java-reader-review.md` |

### 共通の約束（全レンズに必ず渡す）
- **本書の軸を壊さない**：やさしい前編／形式的な後編、gradual・"never frighten working code"、
  TAPL と『しくみ』は任意の副読本。**TAPL 並みの厳密さを足せ、ではない**。
- **重大度を付ける**（ERROR / MISLEADING / 表記 / nitpick 等）。瑣末は末尾に少数。
- **読む対象**：前編 `draft/little/ja/part0〜part9` ＋ `README.md`、共通 `draft/preface.md`・
  `draft/glossary.md`。後編 `draft/seasoned/ja/` は依頼に応じて。`_*.md`（内部メモ）は読まない。
- 出力は**最後のメッセージに、観点ごとの表（該当箇所の引用 / 問題 / 修正案）＋総評**。

### レンズ 1：再現性（reader-reproduction）
- ペルソナ：**型理論の知識ゼロ・Ruby 中級者**。本文だけ（`draft/little/ja/`）を Part 0 から順に読み、
  `/tmp/chibirigor-repro-<id>/` に実装。**`lib/`・`test/`・`exe/`・`examples/`・`docs/` は開かない**
  （答えなので実験が無効化する）。
- Ruby 実行：`nix --extra-experimental-features 'nix-command flakes' develop --command ruby -I /tmp/chibirigor-repro-<id>/lib …`（`cd` しない）。
- 返すもの：章ごと（clarity・推測した所・本文の期待出力との食い違い）＋公開 API の形。
- **採点**：返ってきた実装に対し、シェイプ非依存の挙動採点器を回す（下記「採点ハーネス」）。
  目標は「推測ほぼゼロで本文だけから再実装でき、挙動が一致」。複数名（2〜3）回すと共通の詰まり
  ＝本物の穴が出る。

### レンズ 2：型理論エキスパート
- ペルソナ：TAPL を教えられる水準（gradual／双方向／変性／再帰型／HM／System F／健全性／HKT）。
- 事実確認のため `lib/chibirigor/`・`docs/`・Rigor チェックアウトを参照してよい。
- 探す：形式的な ERROR、条件付きでしか正しくない MISLEADING、TAPL/『しくみ』参照番号の誤り、
  内部/実装との不整合。**honest な簡略化は咎めない**。

### レンズ 3：技術書編集者
- ペルソナ：中級技術書を多数担当したプロ編集者（**技術者ではない**）。構成・学習曲線・章配分・
  語り・読者適合・刊行完成度（まえがき/前提/環境/演習/図/索引/まとめ）を見る。
- 技術的正確さと再現性は別レンズ済みと伝え、**編集観点に集中**させる。【試し書き】/検証メモは
  足場と理解した上で「最終稿でどう整理すべきか」を述べさせる。

### レンズ 4：ドメイン著者（mametter）
- ペルソナ：**遠藤侑介**＝『型システムのしくみ』著者（TAPL 訳者の一人）＋**TypeProf 開発者**。
- 前提に **`/Users/megurine/repo/ruby/rigor/docs/handbook/appendix-typeprof.md`**（TypeProf vs Rigor の
  対比）を必ず渡す。自著 PDF
  `/Users/megurine/Dropbox/EBook/ラムダノート/型システムのしくみ―TypeScriptで実装しながら学ぶ型とプログラミング言語.pdf`
  は引用の公正さ検証用（任意）。
- 突く：「推論器」という語法、引数の call-site 推論の不在、TypeProf 不在、自著引用の公正さ。
  興味を引く点も率直に。**辛口可**。

### レンズ 5：日本語校正・校閲
- ペルソナ：**日本語に堪能なプロの校正・校閲者（技術者ではない）**。日本語の適否だけ。
- 較正例（必ず渡す）：「実はこれ、+ に限りません」＝AI 調で軽すぎる／「体に入れる」＝意味不明な
  比喩／「腑に落とす」＝自動詞の誤用。
- **やさしくカジュアルな文体は壊さない**（堅くしない）。**定着した技術表現**（「例外を投げる」
  「untyped に倒す」「diff を取る」等）は**指摘しない**。技術用語の当否には踏み込まず、迷えば留保。

### レンズ 6：Rigor フィデリティ（乖離防止）
- ペルソナ：Rigor の実装と本書の両方を読み、**「実 Rigor では…／Rigor の中では…」という*事実
  記述*が実態と一致するか**だけを検証する査読者。
- 読む：本書の「③ Rigor 実装の問題」「Rigor の中では」段 ＋ Rigor チェックアウト
  `/Users/megurine/repo/ruby/rigor`（read-only）の一次情報：
  - `docs/handbook/`（appendix 群）／`docs/type-specification/`／`docs/internal-spec/`／
    `docs/adr/`（`docs/adr/README.md` が索引）／`lib/rigor/`。
- **意図的な簡略化は乖離ではない**（各章「続編に送ったもの」＋設計ドラフトに記録済み）。直すのは
  *事故的な不正確さ*（実 Rigor が X なのに本書が ¬X と断言）だけ。両者を区別して報告。
- Rigor リポジトリは**絶対に編集しない**。

### レンズ 7：Java 中堅読者（説明の飛躍検出）
- ペルソナ：**Java で普通に機能開発できる中堅エンジニア**。本書の最大ボリュームの読者層
  ＝「静的型付け言語の実務経験はあるが、型理論は未学習」の代理人。具体的な前提：
  - ジェネリクス（`List<String>`）は**なんとなく使える**が、**「型消去（type erasure）」という語を知らない**。
  - `NullPointerException`（通称「ぬるぽ」）は日常的に踏むが、**それを型システムの言葉で説明できない**
    （null 安全・Option 型・健全性の欠陥…という接続が頭に無い）。
  - 部分型／変性／健全性／gradual／双方向／単一化…といった**型理論の専門語はすべて初見**。
    継承・インターフェース・`instanceof`・キャスト・オーバーロードといった **Java の足場は持っている**。
- 主眼：**説明の飛躍**を炙り出す。本文が（a）専門用語を定義せず使う、（b）「自明」として論理段を
  飛ばす、（c）Java 読者の直感（継承＝部分型のつもり、null は実行時の話、ジェネリクスは型消去…）
  との**ズレを橋渡ししていない**箇所を、引っかかった瞬間に率直に申告する。
- やり方：本文だけ（`draft/little/ja/` を Part 0 から順に。後編 `draft/seasoned/ja/` は依頼があれば
  *軽く*）を読む。`lib/`・`test/`・`examples/` は開かない。**実装はしない**（再現性レンズとの違い）。
  各章で「ここで詰まった／前提が飛んだ／Java の常識と食い違って混乱した」を**読者の生の声**で記録し、
  「彼の既知（ジェネリクス・NPE・継承・キャスト・null）から橋が架かっているか」で判定する。
- 探す：未定義の専門語、Java 的直感との未接続、暗黙の前提、たとえ話の不発。**「分からない」と言うのが
  仕事**で、彼の無知は資産。ただし**やさしい前編が対象**であることを前提に、後編（形式的な巻）の
  難しさは*設計どおり*と理解し、「前編の中での飛躍」と「後編は元々難しい」を区別して報告する。
- 注意：再現性レンズ（型理論ゼロだが*実装して*採点）とは別物。こちらは **「読んで理解が繋がるか・
  飛躍がないか」専門**で、コードは書かない。重大度（詰まって先に進めない／混乱したが推測で進めた／
  nitpick）を付ける。出力ノート `draft/_java-reader-review.md`。

## 採点ハーネス（レンズ 1 用・シェイプ非依存）
再現実装の API 形（`check`/`annotate` が文字列配列でもハッシュ配列でも）に依存せず挙動だけを採点。
`MOD = ChibiRigor | Chibirigor` を動的解決、`check` はサイズ、`annotate` は `"N: 型"` か
`{line:,type:}` を型文字列に正規化して期待値と照合。代表 34 項目（各章の核＋FP 安全ケース：
dead-branch `is_a?`／Union 引数の非 FP／未知キー→nil／`untyped` 沈黙／def 本体検査／baseline）。
`ruby -I <repro>/lib grade.rb` で `SCORE: n/34`。過去 3 名いずれも 34/34（前編は本文だけで再現可）。

## 起動 → 統合 → 適用
1. 依頼に応じてレンズを選ぶ（既定＝全 7。引数で `copyedit` / `fidelity` / `java-reader` 等の単独も可）。独立なので
   **同一メッセージで並列起動**。
2. 各レンズの所見を **`draft/_<lens>-review.md` にコミット**（永続・非同期の台帳）。
3. **統合と選択適用**：
   - **軸を最上位に。すべては反映しない。** ERROR・明確な言語誤用・事実乖離を優先。判断が要る
     もの（形式の入れ具合・図・大改稿）は記録して著者裁量へ。
   - 適用は **`git add <個別ファイル>`**（`-A` 禁止。並行セッションの WIP を巻き込まない）。
   - 前編に手を入れたら `test/test_part*.rb` が緑のままか確認（lib 不変なら自明）。
4. 結果を `draft/_handoff-state.md` のバックログに反映。

## 注意
- レンズは**節目で**回す（per-edit ではない）。全 7 は重いので、軽い点検は該当レンズだけ。
- `opus` 既定。再現性・Java 読者レンズは `sonnet` でも可（複数名で共通の詰まりを見る）。
- このスキルは**方法論の凍結**であり、ペルソナ文面は各回の対象（章/巻）に合わせて差し替えてよい。
