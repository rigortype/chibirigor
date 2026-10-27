# Rigorフィデリティ・チェック — Rigor 0.4.6追随（2026-10-27）

- 照合先：`/Users/megurine/repo/ruby/rigor` HEAD `9f6ef254d`（2026-10-26、`VERSION = "0.4.6"`）。
- 前回監査：2026-06-13（Rigor `0f559176b`、0.1.x系）。この間にRigor側で約6,000コミット。
- 対象：`book/v2/ja/`（前編・後編・付録・用語集）、`AGENTS.md`用語節、`docs/20260607-type-systems-distilled-rigor-mapping.md`。
- 方法：レンズ3本（前編＋付録、後編＋用語集＋台帳、Rigor差分の棚卸し）を並列に実施。実Rigor 0.4.6を
  scratchpad上の使い捨てサンプルで実行（`ruby -I<rigor>/lib <rigor>/exe/rigor`）して出力を比較。
  ERROR判定のうち主要なもの（ADR-83／ADR-67 opt-in／`.rb`インライン可／erasureがリテラルを保つ／
  RBS record＝closed／`--unreachable`不在／`union_size`未実装／`Dynamic[top]`表示）は統括側で一次情報を再確認済み。
- 本書の「意図的な簡略化」は乖離として扱っていない。対象は、**③「Rigorだと」で実Rigorについて事実を述べた文**だけ。

## 結論

1. **本書の骨格は今のRigorにもそのまま当てはまる。**合成中心の推論、三値のaccepts、FactStoreの6バケツ、
   dispatchのカスケード、Unionのメンバごとのディスパッチ、ADR-5は、いずれも現行のRigorと一致する。
2. **一方で、③ の事実記述にはERROR／STALEが20件超ある。**大半は前回監査の時点ですでに不正確だった
   （例：closed record、`--unreachable`、erasureの例）。今回の差分で新しく古くなったのは、ADR-67／83／93／109の4系統。
3. **新章は追加しない。**新しい教えどころは、どれも既存章の発展ノートに1難所として収まる（§D）。
4. **章の軸に関わるのは前編Part 6（open shape）の1件だけ。**著者判断が要る（§C）。

---

## A. 一文〜数文の事実訂正（chibirigor側の簡略化はそのまま、③ だけを直す）

| # | 箇所 | 本書の記述 | 実Rigor 0.4.6 | 修正方針 |
|---|---|---|---|---|
| A1 | 前編part7:129 | 「全員`:yes`のときだけ怒れます」 | 逆。1メンバでも`:no`ならerror（実測で`(Integer)`に`"a" \| 1`を渡すとerror）。本章のコードとも食い違う | 「1メンバでも`:no`なら怒れる。全員`:yes`で初めて黙って通る」 |
| A2 | 前編part7:67 | 「`Const[1]`…比べる前にクラスに丸めて判定」 | 値まで比べる（`def one: (1) -> void`に`2`を渡すとerror） | 丸めるのはchibirigorの簡略化と明示し、実Rigorは丸めない、と添える |
| A3 | 前編part5:145／付録a5-5:62-70／a1-3x | 「既定では黙ったまま、`check --unreachable`のopt-in」 | 実Rigorにこのフラグは無い（lib・exeに0件）。`flow.unreachable-branch`（リテラル述語のみ）と`flow.unreachable-clause`は**既定で出る**（manual 04:62,65）。`x=1; if x.is_a?(String)`は0.4.6では出ない | `--unreachable`はchibirigor独自と書き分ける。実Rigorは既定でwarning/infoを出し、is_a? の死に枝は対象外 |
| A4 | 前編part5:197 | 「Unionのメンバ数が上限を超えると…強制的に拡大」 | `union_size`は仕様にあるが**not yet enforced**（`inference-budgets.md:85`、ADR-41はProposed） | 「仕様には予算があるが、現時点では未実装」 |
| A5 | 前編part0:48／part8:230／後編part5:158-172の表 | 「chibirigor（とRigor）は引数を逆算しない」 | 既定ではしない。ただしopt-inの`parameter_inference:`（ADR-67 WD6、既定`false`）と`sig-gen --params=observed`（ADR-14）がある。どちらも精度を足すだけで、引数境界の診断は締めない | 「（とRigor）」を外し、「Rigorも既定ではしない。opt-inで下限として集められるが、診断には使わない」と書く。**chibirigor自身のframing（不可逆点）は変えない** |
| A6 | 後編part1:206／part7:129（コード例も） | 「`.rb`本体にはインラインで書けない」 | rbs-inlineの`# @rbs %a{rigor:v1:…}`で`.rb`にも書ける。rbs-inlineが入っていれば既定で取り込まれる（ADR-93 Accepted 2026-07-18、manual 16:24-39） | 「基本はRBS側。rbs-inlineのコメントでも書ける（ADR-93）」。part7のコード例（素の`# %a{…}`）も直す |
| A7 | 付録a3-2:77-78／用語集:57 | erasureの例「`Constant<"FOO">` → `String`」 | String/Symbol/Integer/bool/nilの定数は**RBSリテラルのまま残る**（`constant.rb:73-96`） | 本当に丸まる例に替える：`non-empty-string`→`String`、`Integer[1..10]`→`Integer`、`3.14`→`Float` |
| A8 | 前編part1:240／a3-2:75,87 | 「`annotate`は内部では精密、境界では粗く丸める二重構造」 | annotate・診断は内部型をそのまま表示する。erasureがかかるのはRBSを書き出すとき（sig-gen） | 二重構造の主語を「Rigor全体（表示は内部型、sig-genが書くRBSはerasure後）」にする |
| A9 | 付録a2:151 | 「`int<min,max>`はディレクティブ語彙や内部表示で使う」 | ADR-109（v0.3.9）で表示は`Integer[1..10]`。`int<…>`は非推奨の入力別名で、使うと非推奨診断が出る | 「旧表記。いまは非推奨の入力別名（ADR-109）」 |
| A10 | 付録a3-1:29-33（出力例） | `info: fell soft to Dynamic[Top] here (RBS not found …)` | 実際は`FILE:L:C: info: fail-soft fallback at Prism::CallNode: Dynamic[top] [fallback]`。未注釈引数の行は出ない | 出力例を実物の書式に差し替える（または「イメージ」と明示）。「param is untyped」の行は削る |
| A11 | 付録a3-2:95 | hoverの型は「人が直接叩くコマンドではない低レベルAPI」 | `rigor type-of FILE:LINE[:COL]`はmanual 05にユーザー向けとして載っている | 2026-06-13の著者方針（type-ofを見せない）と衝突する。**著者判断**：「ある式だけなら`rigor type-of`もある」と一言足すか、低レベルAPIの一文だけ削る |
| A12 | 付録a3-2:95 | `Rigor.dump_type(x)`と完全修飾すればincludeは不要 | 実行時には`require "rigor/testing"`が要る | 「（`require "rigor/testing"`のうえで）」 |
| A13 | 後編part4:165-177（§4-5 NOTEと表） | HKTの判定は「fuel内で還元を試み、尽きれば`:maybe`」 | `App#accepts`は還元せず上界`bound`に委譲する。fuelが尽きた還元は`bound`（既定untyped）に戻る（`app.rb:19-24`、`hkt_reducer.rb:25`） | 「上界`bound`で判定する。既定はuntypedなので、結果として`:maybe`」 |
| A14 | 後編part4:140／part7:160 | 「余帰納の`seen`に当たるのがfuel」 | `seen`に当たるのは還元中スタックによる循環検出（tying-the-knot、ADR-20 D4）で、fuelはその外側の安全網 | 一文差し替え。停止性の三段の比喩は保てる |
| A15 | 後編part4:145-158（symbolize_names NOTE） | RBSが`true`のリテラル型で宣言し、HKTの型引数になる | Rigorに組み込みの判別表`HktBuiltins::METHOD_RETURN_OVERRIDES`が、呼び出し地点の`Constant[true]`を見て切り替える | 仕組みの説明だけ差し替える。「`Const`が生きている例」という教育上の軸は保つ |
| A16 | 後編part8:106 | 「フロー事実の持ち方 → ADR-46」 | ADR-46は増分解析の依存グラフ。FactStoreはADR-4、ブロック捕獲はADR-56 | ADR番号を差し替える。ADR-32にはADR-93を併記 |
| A17 | 後編part8:132 | 「`ExpressionTyper`の最初の`case`」 | 入口はハッシュ表`PRISM_DISPATCH`（ノード種→ハンドラ） | 差し替える。前編の`case`版と比べられるので、教材としても使える |
| A18 | 後編part5:186 | 「能力ロールが道Aの『集めた能力』の実体」 | ロールのカタログと明示的な`conforms-to`はある。ただし本体から能力を集める推論は保留（ADR-67 WD2。構造的なキャリアが無いため） | 「道Aそのものは実Rigorでも保留」。chibirigorのHashShapeとの対比が活きる |
| A19 | 後編part5:160 | 「推論予算でコスト上限を確保」 | ADR-41はProposed。実際に効いているのはハードコードの打ち切り上限 | §7-6の記述（Proposed）に揃える |
| A20 | 後編part3:150 | 「リテラルunionは基底クラスへ」 | RBSで書けるリテラルはそのまま残る。基底へ広がるのは、RBSで書けないリテラル種と予算超過のとき | 条件を付ける |
| A21 | 後編part8:45 | 「ADR-50（性能ゲート）」 | 題はRelease engineering and stability strategyで、StatusはProposed | 括弧書きを直す |
| A22 | 付録a2-3:80-88 | エスケープ時は「捕獲した変数すべて」を無効化、「RBSから推定」 | 無効化するのは**ブロックが書き換え得るローカル**だけ。判定は組み込みの（クラス, メソッド）カタログ | 例を「ブロック内で再代入する」形にし、判定元を直す |
| A23 | 表記全般（`Dynamic[Top]` 23箇所） | `Dynamic[Top]` | 実際の表示もhandbook/specも`Dynamic[top]`（小文字。2026-06にRigor側で統一） | Rigorの表示を引く箇所は`Dynamic[top]`にする。格子の両端を表す概念の`Top`/`Bot`は大文字のまま |

## B. 橋渡しの一文／短いNOTE（chibirigorのコードは据え置き）

| # | 箇所 | 内容 |
|---|---|---|
| B1 | 前編part9:38 | 「`1 \| untyped`と中途半端に持つより`untyped`と正直に言う。**これがgradualの考え方**」は、実Rigorの現行方針の逆。ADR-83（2026-07-11）で`T \| Dynamic[U] = T \| Dynamic[U]`（具体側を保持）になった（`value-lattice.md:43,50`）。→ 断言を「chibirigorが選んだ保守的な形」に弱め、「実Rigorは`1 \| Dynamic[top]`と具体側を残す」と橋渡しする。part4:131のディスパッチ規則（未知のメンバがあればnil）は今も正しいので、混同しないよう書き分ける |
| B2 | 前編part5:122-124／part9:49（穴の④）／後編part7:66 | 実Rigorは信頼できるガードの下で`Dynamic`も具体化する（ADR-83）。→ 「chibirigorの穴」であることを明確にする |
| B3 | 前編part1:180-189／part2:175-186 | 実Rigorは、既知クラスに無いメソッドを`call.undefined-method`（error）として報告する。トップレベルの未解決呼び出しは`call.unresolved-toplevel`（warning、ADR-34）。黙るのはレシーバがDynamicのとき。→ ③ に一文足す |
| B4 | 前編part4:125-131 | `x = cond ? 1 : nil; x + 1`は、実Rigorでは`call.possible-nil-receiver`（error）。→ 例をnil以外の未知メンバに替えるか、一文添える |
| B5 | 付録a3-1:40-48,69 | 「`Dynamic`マーカーが構造に残るから地図が描ける」は仕組みが違う。実際は`FallbackTracer`がfail-softの瞬間を記録する（traceと同じ発想）。理由つきの由来は別のサイドテーブル（ADR-75/82、`coverage --protection`）。→ 節の理屈を書き直す（数段落） |
| B6 | 後編part6:44-53・図6-1 | `dynamic_origin`バケツが現状記録しているのはブロックのエスケープの事実。untypedの由来はFactStoreの外のサイドテーブル`Scope#dynamic_origins`（ADR-75）。→ バケツの説明と図のラベルを直す |
| B7 | 後編part6:131-135, 62（§6-4） | 実Rigorは、非エスケープのブロック（`each`など）を上限付きfixpointのjoinで書き戻し、精度を保つ（ADR-56）。`captured_local`/`object_content`/`global_storage`は語彙として定義されているだけで、libからは使われていない。→ 「本章の最小モデルでは」と主語を明示し、ADR-56への橋渡しを一文足す |
| B8 | 後編part5 §5-4aの直後（発展ノート） | 「Rigorの道Bは精度を足すだけ」：プロジェクト内のメソッドは本体を局所的に推論する（ADR-57/84）。引数推論はopt-inで、下限として種付けする（ADR-67）。**下限の型では否定的な診断を出さない**、という一点が難所。後編Part 7の「わざと手放す」と響き合う |

## C. 章の軸に関わる（著者判断が要る）：前編Part 6のopen shape

- 本書の記述：part6:101,106,117,155「RigorもHashShapeを**適合**（open）にする」。連動箇所はpart9:47,51,154、
  付録a5-3:45、後編part2:70、台帳 :156。
- 実Rigor：RBSのrecordは**exact closedなHashShape**に翻訳される（`rbs_type_translator.rb:23`）。
  closedな期待型は、余分な既知キーを`:no`にする（`acceptance.rb` "closed target rejects extra keys"）。
  実測：`def take: ({ a: Integer }) -> void`に`{ a: 1, b: 2 }`を渡すと`call.argument-type-mismatch`。
  2026-04-29からこの挙動なので、**前回監査で見落としていたERROR**。
  注釈の無いオプションハッシュは期待型が無いので黙る。open／closed／型付きの余分キーという方針は、キャリア自体が持つ（`hash_shape.rb:140`、既定`:closed`）。
- 選択肢：
  1. **（推奨）chibirigorの簡略化として述べ直す。** chibirigorは「誤検知を出さない」ためにopenを選ぶ（コード・図・語りの山は保つ）。
     ③ だけを「実Rigorはキャリアにopen/closedの方針を持ち、RBSで書いたrecordはclosedとして余分を咎める。
     宣言の無いオプションハッシュには期待型が無いので黙る」に差し替える。「宣言した契約は厳密に、宣言の無い所は黙る」は、
     ADR-5（返りは厳密に、引数は寛容に）やPart 9の思想とも矛盾しない。発展ノートを1本（`6-3x. 発展：閉じた shape`）足せば、
     closedも1難所として扱える。
  2. 章の軸自体を「open／closedは方針の選択」に組み替える。これは学習設計への影響が大きい。
- 付随の裏取り（Rigorの外）：a5-3:41とpart6:122の「Hackのshapeは余分を許す（open）／みなopenを選んできた」。
  Hackのshapeは**既定でclosed**で、`...`を付けたときだけopen（統括側の知識による。Hackの公式ドキュメントで要確認）。
  正しければ、「みなopen」の一般化も直す必要がある。

## D. 新しく教える価値がある概念（任意。すべて既存章の発展ノートで足りる）

| 推奨 | 置き場 | 内容 |
|---|---|---|
| 高 | 前編Part 9の発展ノート（libに数十行） | **精度と保護は違う（protection coverage）**：レシーバがDynamicでないdispatch地点の割合を数える（ADR-63、manual 15「Precision is not protection」）。「沈黙」の裏でuntypedに倒した分だけ守られていない、ということを数字で見せる。Tier 2（mutation）やfusedは持ち込まない |
| 中〜高 | 後編Part 5 | B8と同じ（下限の型では否定的な診断を出さない） |
| 中 | 前編Part 9 §9-2かa1-3xのコラム（文章だけ） | **version guardの死に枝の診断は捨てる**（ADR-47 WD5）。「動くコードを脅かさない」の最もわかりやすい実例 |
| 低〜中 | 後編Part 7のコラム | 楽観的な答えを証明に使わない（ADR-78/101：`Hash#[]`の動的キーの読みはnilを含めない。ただし真偽の判定や枝の削除には使わない） |
| 低 | 後編Part 8 §8-2「読むなら」表に1行 | 効果ラベル（ADR-103）、`rigor unused`（ADR-102）、evidence tier（ADR-65） |

### 新章を追加しない理由
- 核の意味論が変わっていない。
- 変化の大半は精度を足すだけ・opt-in・Proposed／保留のいずれか（ADR-66はProposed・未実装、ADR-111はSuperseded、ADR-112 `@extrbs`は未実装）。
- 章級のテーマは効果ラベル（ADR-103）だけだが、これは「型と効果」という別の理論で、本書の対象と「1 step＝難所1つ」の原則を越える。
- 優先度は**事実の訂正（§A・§C）＞橋渡し（§B）＞新規の発展ノート（§D）**の順。

## E. 統治文書・台帳

> 反映済み（2026-10-27）：AGENTS.mdは`int<min, max>`→`Integer[min..max]`のみ直した。`Dynamic[Top]`は本書の大文字統一（付録a1）に合わせて据え置く。対応レポートは下記6点に加え、fuelの記述を「循環検出＋外側のfuel」に揃えた。

- `AGENTS.md`「用語・表記」：`int<min, max>` → `Integer[min..max]`、`Dynamic[Top]` → `Dynamic[top]`（:126の表と :138）。
  統治ファイルなので**人間の判断で直す**。`Constant<3>`はhandbookの綴りなのでOK（annotateの実際の表示は`3`）。
- `docs/20260607-type-systems-distilled-rigor-mapping.md`：
  - :203「読みは共変、書きは反変」→ 一律共変（宣言変性はSlice 5+ の予定）。
  - :305「ロスレスerasure」→ 「保守的に（広くはなるが狭くはならない）」。
  - :234-236「再帰型をネイティブに持たない」「`App[…, []]`」→ RBSは再帰的なaliasを受け付ける。v0.3.7から暗黙にHKT登録される。`[String]`に直す。
  - :156 → openは期待側の条件に依存する、と直す。
  - :176 → ADR-24/57/84。
  - :330,:344「型推論器vsチェッカー」→ framingの不可逆点に反するので揃える。

## F. 未確認（別レンズで裏取りが要るもの）

- Hackのshapeの既定（上記 §C）。
- 付録a1のElixirの行「Dialyzerの`dynamic()`」：Elixir 1.17以降の集合論的型システムの話で、Dialyzerの話ではない可能性がある。
- 実Rigor 0.4.6が`1 + "x"`を報告しない理由（本書は「Rigorは報告する」と断言していないので、現状はERRORにしていない）。
- `unless s.empty?` → `non-empty-string`（handbookとは一致するが、実測ではまだ`String`のまま）。
- 付録a3-3bのtraceデモを実Rigorで回すと、`5 > 0`が畳まれてunionが出ない（「同型」と言うなら橋渡しの一文があると親切）。

## G. Rigor側への参考（Rigorはread-onlyなので記録のみ）

- Rigorのhandbook `appendix-typeprof.md:125-142`は「params default to Dynamic[top]」のままで、ADR-57/67が反映されていない。

## 反映時の注意

- 正本は`book/v2/ja`。Zenn版は手編集中で、再エクスポートは禁止。v2を直したら、Zennにも同じ内容を手で反映する。
- `book/v2/en`にも同じ記述があるはずなので、EN→JA→Zennの同期台帳に従う。
- 発展ノート方式を守る：教えるためのコードは差し替えず、③ とNOTEだけを直す。
