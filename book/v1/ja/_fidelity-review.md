# Rigor 事実記述 fidelity 査読メモ（v1 清書版）

査読範囲：v1/ja の付録 a1–a4、glossary、後編 Part5/6/7 の「実 Rigor では…／Rigor の中では…」
事実記述。判定基準：「実 Rigor が X なのに本書が ¬X と断言」する*事故的不正確さ*のみを対象とし、
意図的な簡略化（各章「続編／付録に送ったもの」）は乖離としない。

一次情報：`/Users/megurine/repo/ruby/rigor/` の docs/{type-specification,internal-spec,adr,handbook}
および lib/rigor/。

---

## 重大度サマリ

- **乖離=要修正**：2 件（いずれも軽微・局所的な表の誤り。設計記述の骨格は正しい）
- **要確認**：1 件（a2-6 の見出しの過一般化）
- **問題なし**：付録 a1/a3/a4、Part5/6/7 の主要事実記述はすべて実装・spec と一致

---

## 乖離=要修正

### F1. `lowercase-string` の PHPStan 対応欄が「―」だが、PHPStan は持つ
- **本書**：a2-6 の PHPStan 語彙対応表（a2-narrowing-patterns.md 行218）と glossary（行46）で
  `lowercase-string | ―`（PHPStan に対応物なし）と記載。
- **実態**：Rigor handbook `appendix-phpstan.md` 行67 に `lowercase-string | lowercase-string |
  Identical.` ― PHPStan は `lowercase-string` を持つ。ADR-3 OQ3 も `lowercase-string` を
  PHPStan 由来の名として扱う。
- **判定**：乖離（軽微）。`lowercase-string` の対応欄を `lowercase-string` に直すべき。
  なお `uppercase-string | ―` は正しい（PHPStan は uppercase を持たない）。a2-6 表と glossary 表の
  両方に同じ誤り。

### F2. refinement carrier をすべて「集合差（Difference 型）」と説明（predicate-subset 半分を取りこぼし）
- **本書**：a2-6 見出し「refinement carrier はなぜ Difference 型（集合差）か」、本文「refinement
  carrier の『なぜその名か』の答えはここにある」、glossary「`Difference` 型」項も同趣旨。表には
  `numeric-string`・`lowercase-string`・`uppercase-string`・`literal-string` を refinement carrier
  として列挙。
- **実態**：ADR-3 OQ3 Working Decision は **two-tier hybrid**。
  - point-removal → `Difference`（`non-empty-string = Difference[String, ""]`、non-empty-array/hash）
  - `positive-int`/`non-negative-int` → `IntegerRange`（"structurally a Difference against the
    complementary half-line"）
  - predicate-subset → `Refined`（`lowercase-string = Refined[String, :lowercase]`、
    `uppercase-string`、`numeric-string`、`decimal-int-string` 等）。
  `Refined` は `Difference` の* peer*（lib/rigor/type/refined.rb・difference.rb で別クラス）であり、
  集合差ではない。
- **判定**：乖離（軽微・限定）。`non-empty-string = String - ""` という*具体例*は完全に正しい。
  しかし見出し／結語が「refinement carrier 一般＝集合差」と過一般化しており、表に挙げた
  `numeric-string`・`lowercase-string`・`uppercase-string` についてはその「なぜその名か」の答えが
  集合差ではなく述語サブセット（`Refined`）。「非空系（point-removal）は集合差、述語系は `Refined`」
  と two-tier を一言添えれば解消。F2 を要確認に留めず要修正に置くのは、表で名指しした carrier に
  対して断定が当たらないため。

---

## 要確認

### Q1. a2-6 見出しの強さ
- F2 の通り。最小修正（具体例 non-empty-string に限定する一文を見出し直下に置く、または two-tier を
  明示）で「事故的不正確さ」を「正しい縮約」に戻せる。chibirigor 本体が扱わない領域なので影響は
  説明文のみ。

---

## 問題なし（検証済みで一致）

### 付録 a3（道具・dispatch カスケード）
- **a3-1 `rigor check --explain`**：実在する `check` のフラグ（lib/rigor/cli.rb 行396
  `opts.on("--explain", "Surface fail-soft fallback events as :info diagnostics")`）。fail-soft 地点を
  `severity: :info`, `rule: "fallback"` で出す（worker_session.rb `explain_diagnostic`）。本書の
  メッセージ文面「fell soft to Dynamic[Top]…」は例示パラフレーズ（実装は "fail-soft fallback at
  <node>: <type>"）だが、機構（沈黙の可視化・`Dynamic` マーカー回収）は正確。
  ※ なお別に `rigor explain <rule>`（ルール説明）コマンドが存在するが、本書が言うのは `check
  --explain` フラグであり別物。混同なし。
- **a3-2 `rigor type-of file:line:col`**：実在（type_of_command.rb）。renderer が `type:`（内部精密型）
  と `erased:`（RBS 境界へ erase した粗い型）を 2 つ並べる ― 本書の「2 種類の型を並べて見せる」
  「erasure」記述と一致。
- **a3-3 dispatch 5 段カスケード**：本書は ① 定数畳み込み → ② shape → ③ RBS → ④ in-source →
  ⑤ fallback の 5 段に縮約。実 Rigor（inference-engine.md 行156・method_dispatcher.rb）はさらに多段
  （plugin/HKT-builtin/static-refinement/synthetic-method/project-patched/dependency-source/
  discovered-method 等）だが、本書が選んだ 5 名はすべて実在し、**相対順序が正しい**
  （ConstantFolding 先頭、ShapeDispatch は ConstantFolding と RbsDispatch の間 ＝ spec 行220 と一致、
  RBS が in-source 本体推論より先＝③が④に勝つ、最後は `Dynamic[Top]` fallback）。本書自身が
  「素朴／実物」「5 段の順序・名称は原稿どおり」と縮約を明示しており、意図的簡略化として妥当。

### 付録 a1（特別な型）
- **a1-1 `untyped` = `Dynamic[Top]`／軸A・軸B**：special-types.md「`untyped = Dynamic[top]`」、
  `Dynamic[T]` が「gradual 境界を越えた facet」＋「静的 facet `T`」の 2 事実を持つ、と一致。軸A
  （格子位置）/軸B（チェック有無）の分解は spec の `top`（safe-top, unknown 役）と `untyped`
  （Dynamic マーカー）の区別に対応。「`untyped` はトップ型ではない」（a1-4 注）も spec と整合。
- **a1-2 `void` = ⊤ の別名**：spec「RBS treats `void`, `boolish`, `top` equivalently」「Rigor keeps
  `void` distinct internally」。本書は「RBS の型システム上、void はトップ型の別名」と*RBS に帰属*
  させており正確。値文脈で `top` に materialize する点も spec と一致。
  （a1-5 の「`void` は格子上はトップそのもの」はやや強いが、RBS 等価・top への recovery を踏まえた
  許容範囲の縮約。）
- **a1-3 `never` = `bot`（Bot）**：spec「`bot` is the empty type」「`bot <: T`(every result contract)」
  と一致。本書「chibirigor は bot を型として作らず unreachable arm を診断扱い（ADR-47）」も、
  ADR-47 = `47-narrowing-driven-clause-reachability.md`（`flow.unreachable-clause`）で一致。
- **a1-4/a1-5 Top/Bot 格子・3 型比較**：`Bot <: T <: Top`、双対性、void/never/untyped の軸整理は
  spec の value-lattice 記述と整合。

### 付録 a4（ADR 対応）
- 引用 ADR 番号 0/4/5/22/20/41/14/25/32/46/2/37/9/16/48/6/45/19/51/27/44/15/50/47/17/10/24/28 を
  全件突合 ― **番号とタイトル／一言がすべて一致**。
- 載荷の重い主張も実 ADR で確認：
  - **ADR-41 推論予算**：Status: **Proposed** ＝ 本書「Status: Proposed・未実装」○
  - **ADR-20 軽量 HKT**：Status: Accepted (partial)、fuel budget default 64、reducer 実装あり
    （hkt_reducer.rb）＝ 本書「還元 fuel…実装済み」○
  - **ADR-51 CI 出力**：SARIF/GitHub/GitLab（＋checkstyle）＝ 本書「SARIF/GitHub/GitLab」○
  - **ADR-15 並行**：fork-based が shipping backend（Ractor は #22075 でブロック）＝ 本書
    「Ractor/fork」○
  - **ADR-50 性能ゲート**：perf gate ＝ 本書「性能ゲート」○

### 後編 Part5（本物の型推論・TypeProf 対比）
- **TypeProf 対比表**：handbook `appendix-typeprof.md` の対比表をほぼ忠実に翻案。
  whole-program 抽象解釈 vs local+catalog、small/prototype vs whole codebase/CI、
  「引数を call site から推論：できる vs しない（untyped デフォルト）」、診断哲学（stumble report
  vs provable のみ沈黙）― すべて一致。
- **capability role**：本書が挙げた `_ToS`・`_Each[T]`・`_Reader` は core role catalog
  （structural-interfaces-and-object-shapes.md 行150–154）に実在。`role_conformance` slot も lib に
  あり。
- **sig-gen の引数方針（ADR-5）**：「観測 call site をデフォルトにしない／untyped のまま人間に委ねる」
  は handbook typeprof 行174・ADR-5 と整合。

### 後編 Part6（完全な FactStore）
- **Fact の 6 フィールド**：本書「bucket・target・predicate・payload・polarity・stability」＝
  inference-engine.md 行56・lib/rigor/analysis/fact_store.rb 行31
  `Fact < Data.define(:bucket, :target, :predicate, :payload, :polarity, :stability)` と**完全一致**。
- **6 バケツ**：local_binding/captured_local/object_content/global_storage/dynamic_origin/relational
  ＝ `BUCKETS` 定数と**完全一致**。`dynamic_origin` が他5つと毛色違い（由来追跡）も spec の用途と整合。
- **with_fact / invalidate_target(buckets:) / join（両入口の共通だけ残す）**：fact_store.rb・
  inference-engine.md 行57–59 と一致。
- **クロージャ捕獲・エスケープ無効化**（Part6-4 / a2-3）：closure escape → `dynamic_origin` バケツ＋
  `stability: :unstable`＋捕獲 local を保守的に `Dynamic[Top]` 化（inference-engine.md 行505、
  closure_escape_analyzer.rb）と一致。「読むだけは保持・書くなら落とす」も spec と整合。
- **a2 個別パターン**：`&&` 逐次積み／`||` 合流 join、正規表現名前付きキャプチャ → String 事実、
  ivar = 全可視代入の union（object_content）、再代入リセット ＝ いずれも control-flow-analysis.md /
  inference-engine.md の記述と整合。

### 後編 Part7（健全性・予算）
- **fuel 実装済み / inference budget 未実装**：本書「HKT fuel（ADR-20、実装済み）→ 推論予算
  （ADR-41、設計済み・未実装）」＝ DEFAULT_FUEL=64（hkt_reducer.rb）＋ ADR-41 Status: Proposed と一致。
- **`assert: self is T` ディレクティブ**：handbook 07 行383 `%a{rigor:v1:assert: self is Connected}`
  に実在。意味「caller scope に事実を挿入」も spec と一致。本書はコメント形 `# @rigor assert:` で
  提示するが、RBS-inline 形 `%a{rigor:v1:assert:}` の簡略表現として許容。

### 後編 Part3/Part4/Part8（横断確認）
- **Part3 erasure**：`Type#erase_to_rbs` 実在（type/*.rb 全般）、`RbsTypeTranslator.translate(...,
  type_vars:)` 実在（rbs_type_translator.rb 行89）。
- **Part4 HKT/App/fuel**：`Type::App` 実在（type/app.rb）。「再帰型を μ＋余帰納で直接は実装せず
  軽量 HKT＋fuel で別解」「JSON.parse が recursive `json::value` を返す」は ADR-20（行7–8 landed/
  verified）と一致。
- **Part8 ADR マッピング表**：a4-4 と同内容、全件一致（上記 a4 参照）。プラグイン最小実装
  `Chibirigor.register_method` は chibirigor 側の創作（実 Rigor 記述ではない）ため対象外。
