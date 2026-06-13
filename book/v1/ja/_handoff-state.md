# 査読ハンドオフ状態

最終更新：2026-06-13（lib 先行 2 機能の本文反映＋整/読バックログ一部適用）

## 2026-06-13 適用分
- **Union ディスパッチの発展ノート**：前編 part4 に「4-1x. 発展：Union レシーバへのメソッド送信
  （分配して畳む）」を追加（lib 51f299b の本文反映。出力は実 `annotate` で検証済み、Rigor 言及は
  `inference-engine.md` で裏取り）。
- **trace コマンドの付録節**：a3 に「a3-3b. `rigor trace`」＋「a3-3bx. 発展：chibirigor にも極小
  trace」を追加（lib 19c182d の本文反映。`rigor trace` は `docs/manual/02-cli-reference.md`・
  `05-inspecting-types.md` に実在確認済み。a3-4 早見表に 1 行追加）。
- **三題噺フル定義の一本化**（下記 L4 バックログ消化）：Part 0 を定義の家に、共通 README・little
  README は 1 行参照へ。
- **chibivue 比喩の回収**（下記 L3 M2 消化）：Part 0 で `book/impls` ↔ `impls/dist/partN` の対応を
  一文で回収。
- **後編 part5 unify 抜粋を region と同期**（occurs-check 注記の include 反映漏れ。チェッカー --fix）。

## 進行中のフルサイクル
- [x] **L1 真**（型理論・フィデリティ・mametter）― 査読・適用済み
- [x] **L2 伝**（再現性 43/43・Java・Ruby 読者）― 査読・適用済み
- [x] **L3 読**（書評家 ★4.5・辛口 ★3.5）― 査読・適用済み
- [x] **L4 整**（編集・校閲）― 査読・小修正適用済み。**名前空間ブロッカーは要・著者判断（下記）**

## L1 真 ― 適用済み
- `seasoned/part7`：`rigor:v1:assert:` → `rigor:v1:assert`（コロン誤り。実構文では assert にトレーリングコロンを付けない。3 箇所）【ERROR / fidelity】
- `seasoned/part5 §5-4`：ランク 3 決定不能の文言を精密化（HM=ランク1は決定可能、ランク2まで決定可能、ランク3+不能）【MISLEADING / expert】
- `appendix/a1-4`：格子図に `{name:}` が幅部分型で下に伸びる旨の補記を追加（part2 図と整合）【MISLEADING / expert】

## L1 真 ― 著者裁量バックログ（軽微・判断要）
- `seasoned/part4 §4-4`：再帰型スケッチの `seen` は展開前ペアを積む簡約版。「同じペアを再び問われたら打ち切る」が標準（展開後ペア）と取り違えられうる。本文に一言注記の余地。【軽微 / expert】
- `seasoned/part2 §2-3→§2-7`：S-Arrow が第一級 proc 値には効かずメソッド override にのみ効く適用範囲を一文で対比すると安全。【軽微 / expert】
- occurrence typing の公正さ：型同一性述語（`is_a?`/`nil?`）の絞り込みは TypeProf も行う。Rigor 固有は値述語→refinement carrier。後編 part5 か付録 a2 に一文添えると公平（a4-2「独自地形」表記との擦れ）。【軽微 MISLEADING / mametter】
- 表記：seasoned part5 本文の作者ハンドル「mametter」を付録リンクへ委譲が上品。a4-2 Part5「対応章なし」にタグ付き union narrowing は『しくみ』にある旨の注記。【表記 / mametter】
- nitpick 群：§7-4 表 progress 放棄の因果、consistency 反射性、erasure 節番号、occurs-check 省略の一言（expert ノート末尾参照）。

## L2 伝 ― 適用済み
- `little/part4 §4-1`：IfNode コード断片に `NilNode → Const[nil]` と else 無し if の nil ガードを追加（写経コードが run-block `c ? 1 : nil`・演習2 で動くように。lib と一致）【再現性】
- `little/part7 §7-3a`：actual 側 Union の「なぜ一番弱い答えか」の直感を先出し（expected 側 L137 と対称）【Java 読者】
- `little/part8 §8-1`：RBS の `->` が Ruby のラムダ `->` と別物である断りを一文追加【Ruby 読者 BLOCK】
- `little/part9 §9-4`：Top/Bot/格子の 1 行直感（Top≒Object・Bot=住人ゼロ）を追加し、本式は後編送りと明示【Java 読者】

## L2 伝 ― 著者裁量／L4 送りバックログ
- **型の名前空間が前編全体で混在**：bare（part1,4,7,9）と `Type::`（part2,3,5,6,8）が章ごとにまちまち。lib は `Type::Const`/`Type.union` が正。Part4（bare `union`）→Part5（`Type.union`）の切替わりが特に目立つ。横断的整合は L4 整で規約を一本化（著者決定）。【再現性 #2 / 編集】

## L3 読 ― 適用済み
- `little/part5`：unreachable arm 報告の本文を opt-in（`check --unreachable`・既定は沈黙）と明示。「積極的に報告します」の過剰主張を是正（lib の「unreachable: true のときだけ表に出る」と整合し、脅かさない枠も強化）【辛口・痛烈 #1】

## ★刊行ブロッカー ― 型名前空間の逆流 【解消済み 2026-06-11】
**一括是正を適用**：Part2 §2-7・Part4・Part6 §6-1・Part7 全体・Part9 §9-1 のコードを `Type::` 正規形に統一（キャリア定義は `module Type`＋`module_function` で包囲＝lib `type.rb` と一致、内部参照は bare、使用側は `Type::`/`Type.union`）。Part 1 の bare は §2-0 移行前なので維持（移行演出は保持）。散文・コメント・図ラベル・用語集見出しの bare は不変。検査：コード文脈に取りこぼし bare ゼロ・`test/test_part*.rb` 全緑・本書コードが `examples/dist/*/type.rb` 実ソース構造と一致。

<details><summary>是正前の記録</summary>
- **事実**：Part 2 §2-0 が「以降は `Type::Const` のように」と `Chibirigor::Type` へ引っ越しを宣言。lib も `Type::Const`/`Type.union` が正。にもかかわらず後続章のコードが bare（`Const`/`Union`/関数 `union`）に**逆流**している。写経読者が「`Const` か `Type::Const` か」で必ず手が止まる典型的編集事故。
- **逆流箇所**：Part 2 §2-7 発展ノート ／ Part 4 全体（＋今回の L2 加筆も bare のまま揃えてある）／ Part 7 全体 ／ Part 9 ／ Part 6 §6-1 の定義側 ／ seasoned Part 1 §1-2。正しく `Type::` 維持：Part 3・5・6 本体・8。
- **一本化方針（編集者提案・採用推奨）**：`Type::` を正規形に固定。「Part 1 bare 導入 → Part 2 §2-0 で一度だけ引っ越し → 以降ずっと `Type::`」という *1 回の移行が物語として残る* 現演出は教育的に正しいので壊さず、**逆流だけを根絶**。bare は地の文・図ラベル・用語集 headword に限り許可（凡例 1 行明記）。関数 `union` も `Type.union` に統一。
- **適用後に必ず再現性ハーネスを再走**（コード listing を触るため）。
- **状態**：著者の Go を受け一括適用済み（上記）。
</details>

## L4 整 ― 適用済み（小修正）
- `seasoned/examples/README.md:12`：`subst.rb` の対応章を「Part 6 型代入」→「Part 3 ジェネリクスと型代入」に訂正（subtype=P2・mu_typeeq=P4 の並びの中で番号誤記。part3 末尾が subst.rb を指す）【校閲・誤記】
- `little/part8 §8 三題噺`：「ふるまい不変（…」→「ふるまいは変わらない（…」（カジュアル文体での硬さ解消）【校閲・低】

## L4 整 ― 著者裁量バックログ
- **図の偏在**：図は little Part 0・2・5 のみ。概念ヤマの Part 4（Union）・6（open shape）・7（accepts 三値）に図なし。最低 Part 7・6 に新規図。
- ~~**三題噺の定義が共通 README／little README／Part 0 で三重逐語反復** → フル定義は Part 0 に一本化、他は 1 行参照へ。~~ **適用済み（2026-06-13）**
- **後編の章冒頭様式のばらつき**：README が謳う「前編の◯◯は実は△△」フォーマットが Part 2 以降薄い。「起点（前編 P◯）→ 与える名前」を定型化。
- **用語集に後編/Rigor 固有 headword が混在**（`Difference`/refinement carrier）→ 「前編で出会う／後編・Rigor 固有」で区切る。
- **演習 18 章に解答・確認導線がない** → 巻末解答か examples リンク。
- 表記ゆれ 6 系統（ナローイング/絞り込み、丸める/widen、シグネチャ/sig、チルダ/ダッシュ、dead branch 系、ふるまい/挙動）。カジュアル方針では許容。刊行直前に統制するなら一括で。

## L3 読 ― 著者裁量バックログ（背景強化・要・著者一次知識）
- ~~**【書評 M1・最大の穴】Rigor 自身の来歴が空白**：誰が・なぜ・何に困って作ったか／Sorbet・Steep・RBS 公式との関係。README か Part 0 に 2〜3 文。**著者（Rigor 作者）の一次知識が要るので捏造せず著者記述**。~~ **適用済み（2026-06-13）**：Part 0 に来歴ボックスを追加（著者の一次記述：PHP/動的言語の静的解析10年→RubyKaigi 2026 函館→TS/Python＋PHPStan プラグイン機構を RBS 上に統合＝Rigor／短期完成で全体像を知る人が少ない→chibivue に倣いエッセンスを再実装＝本書の動機）。Sorbet/Steep は著者が名指ししていないため記載せず（RBS のみ事実として明記）。来歴は Part 0 に単一ソース化（README には重複させない）。
- ~~**【書評 M2】chibivue 引用が枕止まり**：Part 0 の chibivue 比喩が回収されない。各章末の実装リンク（impls/dist/partN＝「動く最小版が残る」）に繋げる一文で回収できる。~~ **適用済み（2026-06-13）**
- **【書評 R1】章末「続編に送ったもの」の粒度不揃い**：Part 6 filter_map は「なぜ難しいか」まで書けているが、Part 2 method_missing 等は名詞列挙のみ。各項に一行「なぜ後回しか」。
- **【辛口 #2・要判断】Part 0「作るものは 2 つの機能だけ」 vs 付録 `-x`（check --unreachable/--explain/type-of）**：本編中心機能＝2つ・付録は発展、という擁護は可能。気になるなら Part 0 に「本編で作る中心は」等の限定句、または付録の位置づけを明示。教育的簡略化を壊さない範囲で著者判断。
