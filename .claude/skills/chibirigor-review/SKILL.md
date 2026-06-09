---
name: chibirigor-review
description: Run the multi-lens review battery on the chibirigor book draft. Spawns independent-context reviewer subagents — reproducibility (a no-type-theory reader implements from the prose and is graded), type-theory expert, technical-book editor, domain author (mametter = 『しくみ』 author + TypeProf dev), Japanese copyeditor, Rigor-fidelity, a Java mid-level-engineer reader (no type-theory background, vague on generics, flags explanatory leaps from a static-typed-language reader's seat), a Ruby/Rails junior reader (knows Ruby classes incl. Data but has no type-declaration mental model — flags where "type ≈ class" breaks, without over-simplifying), and a book critic (a broad-minded generalist who reads commercial tech books across fields, prizes narrative prose over bare information dumps and rewards clearly woven-in background / related context) — records each lens to a draft/_<lens>-review.md note, then synthesizes only the necessary, axis-preserving fixes. Use when asked to "review chibirigor", "run the review lenses", "查読して/校閲して", validate a chapter or volume before a milestone, or check the book's faithfulness to real Rigor. Not for tiny single edits.
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

## 9 つのレンズ

各レンズは**独立した新規コンテキストのサブエージェント**（Agent ツール、`general-purpose`）として
起動する。互いに依存しないので**並列で投げてよい**。`opus` を既定（再現性・Java/Ruby 読者レンズは `sonnet` 可）。

| # | レンズ | 主眼 | 出力ノート |
|---|---|---|---|
| 1 | 再現性（reproducibility） | 型知識ゼロの読者が本文だけで再実装できるか＋挙動採点 | （実装は `/tmp`、所見は本文へ） |
| 2 | 型理論エキスパート | 形式的・技術的正確さ | `draft/_expert-review-findings.md` |
| 3 | 技術書編集者 | 構成・学習設計・刊行完成度 | `draft/_editorial-review.md` |
| 4 | ドメイン著者（mametter） | 概念の位置づけ・自著引用の公正さ・TypeProf 対比 | `draft/_mametter-review.md` |
| 5 | 日本語校正・校閲 | 言語の質（慣用句誤用・AI 調・表記ゆれ） | `draft/_copyedit-review.md` |
| 6 | Rigor フィデリティ | 「実 Rigor では…」の記述が実態と一致するか | `draft/_fidelity-review.md` |
| 7 | Java 中堅読者（説明の飛躍） | 型理論ゼロ・静的型付け経験ありの読者に飛躍がないか | `draft/_java-reader-review.md` |
| 8 | Ruby/Rails ジュニア読者 | 型宣言のメンタルモデルが無い Ruby 読者に通じるか | `draft/_ruby-reader-review.md` |
| 9 | 書評家（読み物としての質） | 散文・背景の厚み・関連情報の織り込み | `draft/_book-review.md` |

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

### レンズ 8：Ruby/Rails ジュニア読者（型宣言のメンタルモデル不在）
- ペルソナ：**Ruby で入門し、Rails である程度自由にアプリを作れるジュニアエンジニア**。本書の
  *いちばん地面に近い*主要読者層＝「Ruby の世界しか知らない」読者の代理人。具体的な前提：
  - `String`/`Integer`/`Array`/`Hash`、そして新しい **`Data`** クラスは**実物として理解している**
    （`Data.define` も使える）。Ruby の `class`・`nil`・`if`・例外・ブロックは日常。
  - 一方 **他言語の型宣言・型注釈のメンタルモデルが*無い***。「型を*書く*」「コンパイル時に
    型が合う／合わない」という経験が無く、**そもそも「型」を「クラス」と同一視している**
    （`Integer` という型＝`Integer` クラス、という素朴な像しか持たない）。
  - したがって：型注釈・静的型・ジェネリクス（総称型）・部分型・union/narrowing・RBS・健全性…は
    **概念ごと初見**。TypeScript の構文（`x: number`、`A | B`）も読めない（比較で出されても足場が無い）。
- 主眼：**「型≒クラス」の素朴な像しか持たない読者に通じるか**。本文が（a）「型」と「クラス」の
  ズレ（`Const[1]`・Union・構造的な型はクラスでは無い）を断りなく使う、（b）型注釈／静的型付けという
  *行為そのもの*を既知として話を進める、（c）TypeScript・RBS など**他言語/別記法の例を足場
  扱い**する、箇所を、引っかかった瞬間に率直に申告する。`Data` のような**既知の足場が活きている**
  好例も挙げてよい。
- やり方：本文だけ（`draft/little/ja/` を Part 0 から順に。後編は依頼があれば*軽く*）を読む。
  `lib/`・`test/`・`examples/` は開かない。**実装はしない**。各章で「ここで前提が飛んだ／
  Ruby しか知らないと意味が取れない／TS の例で逆に迷った」を**読者の生の声**で記録する。
- **過剰な平易化はしない（重要）**：このレンズの目的は*必要な配慮*の発見であって、**文書のレベルを
  必要以上に下げることではない**。本書は「Ruby 経験者が型を学ぶ」本なので、(i) 文脈・後続の説明で
  無理なく学べる新概念は飛躍ではない（初出で完全定義を要求しない）、(ii) TS 比較などの*補助*は
  「足場」ではなく「おまけ」と位置づけられていれば可、と判断する。直すべきは**「Ruby だけの読者が
  そこで本当に詰まって先に進めない」真の段差**だけ。「やさしくできる」ではなく「ここが*壊れている*」
  を選り分けて報告すること。
- 探す：型≒クラスの素朴像との未接続、型注釈/静的型を既知扱いした飛躍、TS/他記法の足場化、未定義の
  専門語。重大度（**BLOCK**＝Ruby だけの読者が詰まり先へ進めない／**FRICTION**＝引っかかるが文脈で
  回復可能／nitpick）を付ける。Java 読者レンズ(7)とは前提が違う（あちらは静的型の素養あり、こちらは
  *それも無い*）点に注意。出力ノート `draft/_ruby-reader-review.md`。

### レンズ 9：書評家（読み物としての質・背景の厚み）
- ペルソナ：**特定技術の専門家ではないが、知識と好奇心が極めて幅広い書評家**。分野を横断して
  商業技術書を濫読しており、「本として読ませるか」を見抜く目を持つ。**情報の羅列より散文
  （ナラティブ）を重んじ**、本筋の技術の**背景・歴史・関連分野へのつながり**が明確に説明されて
  いると高く評価する。逆に、定義や手順が脈絡なく並ぶだけ／「なぜそうなのか」「どこから来たのか」が
  省かれていると物足りなさを覚える。
- 主眼：本書が**読み物として成立しているか**。(a) 各概念に**背景・由来・関連情報**（なぜ Ruby で
  型か、Hack→PHPStan→Rigor の系譜、TAPL/『しくみ』との関係、gradual の思想史…）が*本筋に
  織り込まれて*説明されているか、(b) 箇条書き・表・コードの**羅列に散文の接続線**が通っているか
  （「次に何を・なぜ」が地の文で繋がるか）、(c) 章をまたいだ**物語の弧**（伏線と回収・動機づけ）が
  あるか。**高評価できる好例も積極的に挙げる**（このレンズは減点だけでなく*どこが効いているか*の
  言語化も仕事）。
- やり方：本文だけ（`draft/little/ja/` を Part 0 から順に。共通・付録も読む。後編は依頼があれば
  *軽く*）を一読者として通読する。`lib/`・`test/`・`examples/` は開かない。**実装も技術的検証も
  しない**（正確さは別レンズ）。「読み物として面白かった／背景が腑に落ちた／ここは無味乾燥だった」を
  *読者の率直な読後感*で記録する。
- **軸を壊さない（重要）**：背景・関連情報を評価するレンズだが、それは**冗長・脱線の推奨ではない**。
  やさしい前編／本筋の流れを乱す長広舌や、初学者を脅かす深掘りは**逆に減点**する。深い背景を
  付録・後編に逃がす設計（現状の構成）はむしろ正しいと尊重し、「*本筋に最小限の背景が気持ちよく
  織り込まれているか*」で評価する。"never frighten working code"・gradual の軸は保つ。
- 探す：背景・動機・由来の欠落（「天下り的に手順だけ」）、羅列に散文が伴わない箇所、関連分野への
  接続不足、章の動機づけの薄さ。併せて**特に効いている散文・文脈づけを称揚**する。重大度
  （**高評価**＝読み物として光る所／**物足りない**＝背景・関連情報が薄く読後感が痩せる所／
  **羅列的**＝散文の接続線が要る所／nitpick）。出力ノート `draft/_book-review.md`。

## 採点ハーネス（レンズ 1 用・シェイプ非依存）
再現実装の API 形（`check`/`annotate` が文字列配列でもハッシュ配列でも）に依存せず挙動だけを採点。
`MOD = ChibiRigor | Chibirigor` を動的解決、`check` はサイズ、`annotate` は `"N: 型"` か
`{line:,type:}` を型文字列に正規化して期待値と照合。代表 34 項目（各章の核＋FP 安全ケース：
dead-branch `is_a?`／Union 引数の非 FP／未知キー→nil／`untyped` 沈黙／def 本体検査／baseline）。
`ruby -I <repro>/lib grade.rb` で `SCORE: n/34`。過去 3 名いずれも 34/34（前編は本文だけで再現可）。

## 起動 → 統合 → 適用
1. 依頼に応じてレンズを選ぶ（既定＝全 9。引数で `copyedit` / `fidelity` / `java-reader` / `ruby-reader` / `book-review` 等の単独も可）。独立なので
   **同一メッセージで並列起動**。
2. 各レンズの所見を **`draft/_<lens>-review.md` にコミット**（永続・非同期の台帳）。
3. **統合と選択適用**：
   - **軸を最上位に。すべては反映しない。** ERROR・明確な言語誤用・事実乖離を優先。判断が要る
     もの（形式の入れ具合・図・大改稿）は記録して著者裁量へ。
   - 適用は **`git add <個別ファイル>`**（`-A` 禁止。並行セッションの WIP を巻き込まない）。
   - 前編に手を入れたら `test/test_part*.rb` が緑のままか確認（lib 不変なら自明）。
4. 結果を `draft/_handoff-state.md` のバックログに反映。

## 注意
- レンズは**節目で**回す（per-edit ではない）。全 9 は重いので、軽い点検は該当レンズだけ。
- `opus` 既定。再現性・Java 読者レンズは `sonnet` でも可（複数名で共通の詰まりを見る）。
- このスキルは**方法論の凍結**であり、ペルソナ文面は各回の対象（章/巻）に合わせて差し替えてよい。
