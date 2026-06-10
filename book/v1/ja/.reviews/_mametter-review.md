# mametter レンズ査読 ― chibirigor v1（日本語）

レビュアー視点：遠藤侑介＝『型システムのしくみ』著者（TAPL 共訳）＋ TypeProf 開発者。
前提として Rigor ハンドブック `appendix-typeprof.md`（TypeProf vs Rigor）を通読済み。
自著 PDF（`型システムのしくみ―…pdf`, 184p）で引用の公正さを実地検証した。

突くべき四観点：①「推論器」の語法、② 引数の call-site 推論の不在、③ TypeProf の扱い・対比の公正さ、
④ 自著『しくみ』引用の公正さ。**本書の軸（やさしい前編／形式的後編・gradual・脅かさない・参考書は任意）は壊さない**前提で読んだ。

---

## 観点① ―「推論器」の語法

| 引用 | 問題 | 修正案 | 重大度 |
|---|---|---|---|
| README / part0 見出し「推論を土台にした型チェッカー」、part8 §8-5「同じ推論エンジン」 | **問題なし（むしろ模範的）**。表看板を「型チェッカー」に据え、「推論器(inferrer)」を製品名として立てていない。part0 §0.1「推論はチェックと別物の前段ではなく、チェックを成り立たせている**土台**」。appendix-typeprof の "checker first" と整合。 | 修正不要 | — |
| seasoned/part1 L15「私たちは**推論器**の心臓として 2 つの関数を作りました」 | 内部コア(the inference core)の素朴な指示で、製品全体を「推論器」と僭称していない。「推論の心臓」の言い換えとして自然。 | 「推論の心臓」で揃えても良いが nitpick 未満 | nitpick |
| a4 L102 / part8 L232「型推論エンジン」 | 内部仕様 inference-engine.md / ADR-4 の名称に準拠。誤用なし。 | 修正不要 | — |

**総括**：語法は健全。TypeProf=推論器(RBS 生成器) / Rigor=チェッカー を截然と分ける appendix-typeprof の立場を、初学者向けに正確に翻訳。**突きどころ無し。**

---

## 観点② ― 引数の call-site 推論の不在

| 引用 | 問題 | 修正案 | 重大度 |
|---|---|---|---|
| part0 §0.1 L43「**引数の型を呼び出し元の使われ方から逆算することは（前編では）しません。引数は分からなければ untyped に倒します**」 | **正確かつ明快**。核心制約を入口で一度・誤解の余地なく宣言。appendix-typeprof「Infers params from call sites? → No」と一致。 | 修正不要 | — |
| part8 §8-6 L253–258「引数を untyped にするのは設計判断…ローカルに見て分からない引数は untyped に倒す（スケールするし誤検知も出ない）」 | 具体例 `def double(n)`→`(untyped)->untyped` で同じ一線に降りる。local+catalog の正しい要約。前編で一度だけ具体化する配分も良い。 | 修正不要 | — |
| seasoned/part5 対比表 L205「引数型を call site から推論？ TypeProf=できる（最大の強み）/ Rigor=しない（untyped 既定）」 | **最も突くと身構えた箇所が最も正確。** call-site 推論不在を「弱点」でなく「スケール＋FP ゼロの別価値観での選択」と提示。appendix「The trade is real and intentional」と一致。 | 修正不要 | — |
| seasoned/part5 §5-4 型理論コラム「HM が使えない理由は 1 つではない」（決定不能性・到達不能性・精度問題） | call-site 全解を採らない理由を工学弁明でなく型理論 3 問題に分解。Kfoury–Wells 1994/Wells 1999 参照も妥当。**興味を引いた点。** | 修正不要 | — |

**総括**：不在は part0（原則）→ part8（具体）→ 後編 part5（対比一本化）と段階配分され、どの層も誤りなし。**ERROR/MISLEADING 皆無。**

---

## 観点③ ― TypeProf の扱い・対比の公正さ

| 引用 | 問題 | 修正案 | 重大度 |
|---|---|---|---|
| part0 L47–53 box「Ruby のもう一つの推論ツール ― TypeProf」 | 前編の早期で「言及すべき所で言及」。要約正確、「あえてしないのは弱点でなく設計の選択」と公正。**「言及していない」疑念は晴れた。** | 修正不要 | — |
| seasoned/part5 §5-4a L179「TypeProf は *mametter（TypeProf 作者）* が設計した」 | 事実は正しいが、本文に作者ハンドルのクレジットを 1 箇所だけ入れる体裁が浮く。appendix-typeprof は機能で記述。学習書なら作者名は付録/脚注向き。 | 「Ruby コア同梱の型レベル抽象インタプリタ」に統合し、作者クレジットは付録 a4/appendix-typeprof リンクへ委譲 | 表記 |
| seasoned/part5 対比表 L207「主な出力：TypeProf=RBS（エラーは副産物）/ Rigor=診断（確実なバグのみ・FP ゼロ）」 | appendix「side effect vs the main product」を忠実圧縮。公正。 | 修正不要 | — |
| seasoned/part5 L214–217「sig-gen は TypeProf と同職だが、観測 call-site を既定にしない方針（ADR-5）が異なる」 | appendix「the one feature where the tools do the same job」(sig-gen↔typeprof CLI) に正確対応。ADR-5 まで踏み込み一面的でない。 | 修正不要 | — |
| **不在の検査**：TypeProf の occurrence typing(`is_a?`/`nil?`)への言及 | appendix は明示（"Both tools do flow-sensitive occurrence typing — TypeProf included"）。本書は「narrowing=Rigor 固有」と断言してはいないが、a4-2 L60 が前編 Part 5 を「独自地形」と表記し、型同一性述語の絞り込みは TypeProf も持つ事実と擦れる余地。本書 narrowing の真の固有性は*値述語 refinement carrier*なので軸は壊れない。 | 後編 part5 か付録 a2 に一文 ―「型同一性述語の絞り込みは TypeProf も行う。Rigor 固有は*値述語*から refinement carrier を作る層」― を添える | MISLEADING（軽微） |

**総括**：TypeProf 対比は**おおむね公正**。appendix の核（共通点＝注釈不要推論／相違＝whole-program vs local+catalog／sig-gen 同職）を正しく配分。突くなら (a) 作者ハンドルの本文クレジットは付録送りが上品、(b) narrowing=Rigor 独自と読まれかねない含意に予防線を。いずれも軸を壊さぬ微修正。

---

## 観点④ ― 自著『しくみ』引用の公正さ（PDF 実地照合）

| 引用 | PDF 照合結果 | 重大度 |
|---|---|---|
| part0 L84 / part1 L100,184「『しくみ』のチェッカーは型エラーで例外を投げて止まる」 | **正確**。PDF の typecheck は `throw "number expected"` で停止（PDF「typecheck 関数が例外を投げなかったので…」L1269）。 | — |
| part0 L60「TAPL のエッセンスをやさしく蒸留した入門書。注釈付きミニ言語のチェッカーを TS で作る」 | **正確かつ寛厚**。PDF おわりに「TypeScript のサブセット…に対する型検査器を実装」と合致。「裏表の関係」も公正。 | — |
| seasoned/part5 L131,148 / a4-3 L82「『しくみ』9 章演習で『型引数推論は正解を知らない』と解答を放棄し『自明なケースに限定して推論するとよい』と助言」 | **逐語的に正確**。PDF「筆者は正確な答えを知りません。よって…解答も付けていません」「自明なケースに限定して推論するようにするのがよいかもしれません」。本書方針をこの演習にピン留めしたのは**最も筋の良い引用**。**興味を引いた点。** | — |
| a4-2 L59 / a5 L85「『しくみ』は一般 union を『型システムへの影響が大きすぎる』として避け、タグ付き union のみ」 | **正確**。PDF L3440「一般の union 型を組み込もうとすると型システムに与える影響が大きく、TAPL の範囲を越えてしまう」、サポートはタグ付きのみ。 | — |
| seasoned/part2 L84,126「『しくみ』7 章は部分型を subtype 関数で実装し、引数は `subtype(ty2.params[i], ty1.params[i])` と入れ替え反変に」 | **正確**。PDF L4414「if (!subtype(ty2.params[i].type, ty1.params[i].type)) …// 反変」。実装細部まで忠実。 | — |
| a4-2 L60：前編 Part 5（ナローイング）を『しくみ』「対応章なし／独自地形」と表記 | **軽微な不精**。PDF L994 はタグ付き union 文脈で "narrowing" を明示使用。本書 narrowing は*フロー感応・gradual・値述語*まで含むので「独自地形」は弁護可能だが、「『しくみ』は narrowing を扱わない」と読むと不正確。 | 表記 |

**総括（自著引用の公正さ）**：**極めて公正。** 標本 6 引用すべて PDF と逐語一致または忠実要約で、持ち上げも貶しもない。とりわけ「9 章演習で解答を放棄した所を後編 part5 が埋める」は、原著者の留保を尊重しつつ本書貢献を立てる模範的な使い方。唯一の傷は a4-2 の narrowing 表記。

---

## 総評

四観点（call-site 推論・推論器語法・TypeProf 対比・自著引用）で **ERROR 0 件、MISLEADING 1 件（軽微）**。最も突くと身構えた点で本書はことごとく正確だった。

**興味を引いた点（率直に）：**
1. **§5-4「HM が使えない 3 つの別問題」分解**（決定不能性・到達不能性・精度問題＋各別対処）。Ruby の `define_method`/`method_missing` を「到達不能性」として型理論語彙に載せたのは、TypeProf で同じ壁にぶつかった身として膝を打つ整理。
2. **9 章演習へのピン留め**。私が解答を付けなかったその演習を後編が埋める出発点に据える構図。原著の留保を改変せず引き front line と名指しする ― 誠実。
3. **TypeProf を「弱点ある競合」でなく「別価値観での別解」として描く一貫性**。`untyped` を諦めでなく意図的 degrade として描き切る筆致が part0→part5 で崩れない。

**自著引用の公正さ評価：合格(A)。** 抽出全引用が PDF と一致。牽強付会なし。微傷は a4-2 の narrowing 表記のみ。

**最小修正提案（軸を壊さない範囲）：**
- (表記) seasoned/part5 §5-4a の本文作者クレジット「*mametter（TypeProf 作者）*が設計した」を機能記述に統合し、作者名は付録/appendix-typeprof リンクへ。
- (MISLEADING 軽微) 後編 part5 か付録 a2 に一文「型同一性述語(`is_a?`/`nil?`)の絞り込みは TypeProf も行う。Rigor 固有は値述語から refinement carrier を作る層」を添え、narrowing=Rigor 独自の誤読を予防。
- (表記) a4-2 の前編 Part 5 行「『しくみ』対応章なし」に「（タグ付き union の narrowing は『しくみ』にあるが、フロー感応・値述語の絞り込みは独自地形）」と注記。
