# 型理論エキスパートレビュー（前編 v1）／2026-06-13 L1真・dump_type/erasure 監査

対象：`book/v1/ja/` 前編（little Part0–9・appendix a1/a3/a4・glossary・README）。
照合：`lib/chibirigor/`（type_of.rb・checker.rb・dispatch.rb・accepts.rb・narrowing.rb）、`test/`（test_dump_type / test_explain / test_unreachable 実行・全 PASS）、`exe/chibirigor`。
方針：honest な簡略化は咎めない。形式的 ERROR・条件付き MISLEADING・REF 誤り・実装不整合のみを拾う。やさしい前編／gradual／脅かさないの軸は壊さない。

---

## 今セッションの最重点 3 点（dump_type・erasure・`:info` と健全性）

### 1. dump_type（a3-2・type_of.rb・checker.rb）

**結論：型理論的に妥当。説明は概念的に正確。ERROR なし。**

- 「実行時は値を素通し／型は引数の型」（a3-2 L143、コード L104–110）は **type-level identity ＋ 観測（observation）** として正確。`type_of_call` が `dump_type(式)` を `type_of(arg)` の結果でそのまま返し（`return t`）、副作用は診断列への `:info` push のみ。型システムの判定（subtype・union・dispatch）を一切変えない＝健全性に影響しない、という記述に偽りはない。
- 「だから `dump_type(x)` の型は `x` の型」（L143）は、test_dump_type.rb の `dump_type passes the value type through`（`dump_type("a".upcase)` → `String`）で裏が取れている。
- PHPStan の `dumpType()` との対応づけ（L125・L88 コメント）も妥当。PHPStan の `\PHPStan\dumpType()` も値恒等・型を診断出力する観測子で、性格が一致する。
- checker.rb L48–49 が `:dump_type` を**フラグ非依存で常に併載**するのは、本文「基本機能として持つ」（a3-2 L129）と一致。実装・本文・テスト三者整合。

唯一の nitpick は下表参照（「印字する」の主語）。

### 2. 二段構え／erasure（a3-2・glossary・Part1 L239–241）

**結論：形式的に正確。Java type erasure との切り分けも正しい。ERROR なし。**

- 「内部の精密型（`Constant<"FOO">`）／RBS 境界の保守型（`String`）を境界で丸める」を **erasure** と呼び、Java ジェネリクスの実行時型消去（型引数 `<String>` を実行時に消す）とは別物、という記述（a3-2 L107–109、glossary L71–75）は妥当。
  - 「精度を落とす境界操作（precision-erasing boundary operation）」という説明は、型理論の erasure（型情報を落とす写像）の一般的用法に沿う。実 Rigor が内部の細粒度型を RBS 表現可能な粗い型へ落とす写像を erasure と呼ぶのは、用語の一貫した使用。
  - 対比軸（Java＝実行時／型引数の除去 ↔ Rigor＝静的／精度の境界での丸め）は、両者がともに「型情報を落とす」点で同類でありつつ「何を・いつ落とすか」が異なる、という整理として正確。「別物」と断ずるのは適切（読者の混同を防ぐ）。
- glossary の参照「erasure〔前編 P1（予告：付録 a1）／後編 P3 で本式〕」は **REF 不整合の疑い**（下表参照）。Part1 L239–241 と a4-4 L112 は erasure の所在を「**付録 a3-2**」「後編 Part 3」とする。glossary だけ「付録 a1」を予告先に挙げており、a1 に erasure の節は無い（a1 は untyped/void/never/Top/Bot のカタログ）。

### 3. `:info` と健全性の関係

**結論：整合的に説明できている。ERROR なし。**

- dump_type・--explain・unreachable がいずれも `severity :info` で、`exe/chibirigor` が「info のみなら exit 0、本物のエラーがあるときだけ exit 1」（exe L48–50）とする設計は、本書の「健全性より誤検知ゼロ」（Part9 L150 価値観表・9-2）と矛盾しない。
  - `:info` は「診断だが脅かさない」＝終了コードを汚さない観測子であり、`accepts` の `:maybe`／fail-soft の沈黙という unsound 容認の枠と同じ価値序列（誤検知＝動くコードを脅かすを最重とする）に乗る。
  - unreachable（a1-3x）が「証明できるときだけ断言／少しでも `Dynamic` が混じれば黙る／祖先関係は断言しない」（L172–178、test_unreachable で裏取り）とするのは、健全性側（FP ゼロ）に倒した保守的判定で、本書の枠と完全整合。
- 注意点（nitpick）：Part9 9-2 は「chibirigor は健全（sound）ではない。わざと見逃す」と述べる。`:info` 系（--explain/unreachable）はむしろ「見逃しを可視化する／余分な枝を教える」**向き**の道具で、unsound 化ではない。本文はこれを a3-1/a1-3x 側で正しく「沈黙の可視化」「opt-in の助言」と位置づけており、9-2 の 4 穴と混同させていない。整合的。

---

## 通し点検（その他）

- **Const/Nominal/Dynamic**（Part1）：型をデータで表す／`type_of` は失敗しない／知らないものは `Dynamic`、いずれも実装（type_of.rb L12–25）と一致。OK 再確認。
- **Union の untyped 伝播**（Part9 9-1）：`flat.any?(Dynamic)` で全体 untyped（本文 L28–32）。gradual の標準的扱い。OK。
- **ナローイング**（Part5・a1-3x）：閉じた既知型限定・葉クラスの互いに素・祖先は断言しない。健全側に倒した保守判定で formally safe。OK。
- **accepts 三値**（Part7・accepts.rb）：untyped→`:maybe`／actual Union は weakest（全メンバ通って `:yes`）／expected Union は strongest（どれか 1 つで `:yes`）。union subtyping の左右の扱いとして正しい。nitpick 1 件（下表、本文 `widen`＋`==` ↔ 実装 `class_of` 比較）。
- **HashShape 幅部分型**（Part6）：「キーが多い方が部分型（`{name:,age:} <: {name:}`）」「期待は open＝少なくとも」。width subtyping の向きが正確（L108–130）。a1-4 L205 の図注「構造型は幅部分型で下に伸びる」とも整合。OK 再確認。
- **RBS / 戻り型合成**（Part8・checker.rb L28–39）：宣言があるときだけ `⇐`、untyped 宣言は黙る（gradual 保証）。OK。
- **dispatch 5 段カスケード**（a3-3）：① 定数畳み込み → ② shape → ③ RBS → ④ in-source → ⑤ fallback、③＞④ の優先順位の説明（宣言を本体に優先）は順序意味論として妥当。本書 1 段との対応表も正確。OK。
- **a3-1x 極小 --explain**：「dispatch.rb の signature が nil の枝で provenance を 1 行積む」は dispatch.rb L90–98 と一致。`x = mystery_call` → `y = x + 2` で沈黙が伝播（`x` が Dynamic → `+` も class_of nil → fail_soft）という例も実装挙動どおり。OK。
- **TAPL/『しくみ』参照番号**（a4-2/a4-3・各章末三題噺）：健全性＝TAPL 8 章 §8.3（Part9 L159・a4 L84）、部分型＝15 章（Part7・glossary L33）、レコード／組＝11 章 §11.8/§11.7（Part6・a4 L61）、型再構築＝22 章（glossary L69）、HKT＝29 章、再帰型＝20–21 章 ― いずれも TAPL の章割りと一致。REF 誤りは下表の glossary erasure 1 件のみ。

---

## 指摘表

| 該当箇所 | 本書の記述 | 型理論的な問題 | 修正案 | 重大度 |
|---|---|---|---|---|
| glossary.md L71 erasure 項 | 「erasure〔**前編 P1（予告：付録 a1）**／後編 P3 で本式〕」 | erasure の予告先は付録 a1 ではなく **a3-2**。a1（特別な型カタログ）に erasure の節は無く、Part1 L239–241・a4-4 L112 はいずれも所在を a3-2／後編 P3 とする。glossary だけ参照先がズレている。 | 「〔前編 P1（予告：付録 **a3-2**）／後編 P3 で本式〕」に修正。 | REF |
| a3-2 L142–143 | 「`type_of` が `dump_type(式)` を見つけたら、引数の推論型を `:info` で**印字し**、値はそのまま返します」 | 厳密には印字（端末出力）するのは `exe/chibirigor` の `render_diagnostic`。`type_of_call` がするのは「`:info` 診断を**診断列に積む**」こと（type_of.rb L108）。最小版では実害なく本文の流れ上は許容範囲だが、「印字」と書くと出力責務が type_of にあるよう読める。 | 「引数の推論型を `:info` 診断として**記録し**（CLI がそれを印字し）、値はそのまま返します」程度に。 | nitpick |
| Part7 7-2 L49–56 ↔ accepts.rb L20 | 本文は `widen(t)=Type::Nominal[t.value.class.name.to_sym]` を定義し `widen(expected)==widen(actual)` で比較。 | 実装は `widen`＋`==` ではなく `Dispatch.class_of(expected)==Dispatch.class_of(actual)`（accepts.rb L20）で比較する。結果は等価（`Const`→クラス丸め→シンボル比較）だが、本文コードをそのまま写経すると `Generic`（`Array[...]`）の比較で実装と分岐する余地がある（`widen` は `Generic` を素通しし `==` は型引数まで見るのに対し、`class_of` は `:Array` に丸める）。前編の例の範囲では表面化しないが、本文コードと実装の同一性を厳密に主張するなら注記が要る。 | 前編の簡略としては OK。気になるなら 7-2 に「実装は丸めとシンボル比較を `class_of` に集約している」の一文を添える程度で十分。 | nitpick（MISLEADING 未満） |

---

## 総評

**今セッションの最重点 3 点（dump_type／erasure／`:info`×健全性）は、いずれも形式的に妥当で、ERROR・条件付き MISLEADING ともに無し。** dump_type は「type-level identity ＋ 観測」という性格づけが正確で、実装（恒等返し＋`:info` 副作用）・本文・テストの三者が整合。PHPStan `dumpType()` への対応づけも適切。erasure は「精度を落とす境界操作」という説明が型理論の用法に沿い、Java 実行時型消去との「別物」切り分けも読者の混同を正しく防いでいる。`:info`（脅かさない診断）が「健全性より誤検知ゼロ」の価値序列に乗る、という設計の説明も一貫している。

通し点検でも、過去レビューで潰した類の形式的 ERROR の再混入は確認されなかった。Union 受理の weakest/strongest、HashShape の幅部分型の向き、unreachable の保守的断言、TAPL/『しくみ』の章番号、dispatch 5 段の順序意味論 ― いずれも正確。

唯一の実害的指摘は **REF 1 件**（glossary の erasure 予告先が `a1` → 正は `a3-2`）。残り 2 件は nitpick（「印字」の主語、本文 `widen` ↔ 実装 `class_of` の同一性）で、やさしい前編の簡略の範囲。軸（やさしさ・gradual・脅かさない）を脅かす記述は無い。
