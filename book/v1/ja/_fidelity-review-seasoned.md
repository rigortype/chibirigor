# 後編 L1 真・Rigorフィデリティ 査読（2026-06-09）

対象：`book/v1/ja/seasoned/` Part 1–8 ＋ `README.md`、`book/v1/ja/appendix/a1〜a5`。
一次情報：`/Users/megurine/repo/ruby/rigor`（read-only）の `docs/handbook/`・`docs/type-specification/`・
`docs/internal-spec/`・`docs/adr/`・`lib/rigor/`。

レンズ：**「実 Rigor では…／Rigor の中では…」という*事実記述*が実態と一致するか**だけを検証する。
型理論の正しさ・日本語・読み味は対象外。意図的簡略化（スケッチ・縮図）と事故的不正確さを区別して報告。

---

## 総評

後編は前編より実 Rigor への言及が桁違いに多いが、**核心部分のフィデリティは概して非常に高い**。
特筆すべき高精度：

- **Part 6（完全な FactStore）** … 6 バケツ名・Fact の 6 フィールド・`invalidate_target(buckets:)`・
  不変ストア・クロージャ捕獲→`dynamic_origin`／captured-local の `Dynamic[Top]` drop が、
  `docs/internal-spec/inference-engine.md` L52–61 / L505 と**逐語的に一致**。この巻の生命線が守られている。
- **Part 8 / a4 の ADR 表** … 引用した約 22 本の ADR 番号・題が**すべて正確**（ADR-20 実装済み、
  ADR-41 Proposed/未実装まで含めて）。
- **Part 3 erasure・Part 5 推論・a1 特別な型・a2 refinement carrier・a3 dispatch/CLI** も
  一次情報とよく整合。

一方で **RBS::Extended ディレクティブの「書き方・置き場・意味」を扱うコラム群に、同種の事故的不正確さが
3 件集中**している（Part 1 / Part 7 のコラム）。実 Rigor の最重要設計判断「ディレクティブは
`.rbs` の `%a{rigor:v1:…}` でのみ発火し、`.rb` の `# …` コメントには**書けない**」（ADR-1 で
`# @rigor` コメントを明示的に Rejected、handbook 07 で "You **cannot** put these … inside a `.rb` file"）に
正面から反する記述があり、**ERROR 扱い**とした。加えて Part 4 の JSON HKT コラムに、存在しない URI 名・
誤った arity の事故が 1 件。前編で見つかった `fall-soft`→`fail-soft` の同型ミスは**後編には無い**
（`fail-soft` は全箇所正しい）。

重大度別件数：**ERROR 4**／**要確認 2**／**OK（簡略化・重要）3**／**nitpick 3**。

---

## ERROR（事故的不正確さ ― 実 Rigor が X なのに本書が断りなく ¬X と断言）

### E1. `param:` ディレクティブは「メソッド本体にインラインで書く `# param:` コメント」ではない（Part 1 コラム）

| 項目 | 内容 |
|---|---|
| 本書の記述 | Part 1 §1-7-a コラム：「Rigor には、**メソッド本体にインラインで書く** `# param: x String` スタイルの **`param:` ディレクティブ**があります（RBS ファイルを別に書かずにすむ簡略形）」 |
| 一次情報での実態 | `%a{rigor:v1:param: <name> is <type>}` は **RBS ファイルの `def` 行に書く**注釈。`docs/handbook/07-rbs-and-extended.md` L124, L200–215。同 L249–253：「You **cannot** put these `%a{rigor:v1:…}` directives inside a `.rb` file. The directives only fire when read from RBS — that is a design choice (see ADR-5)」。`.rb` にインライン型を書くのは別物の `rigor-rbs-inline` プラグインで、それは upstream rbs-inline の `# @rbs` 文法であり **RBS::Extended ではない**（同 L284–288） |
| 判定 | **ERROR**（置き場・書式が逆。`param:` の*二つの効果*＝本体ナローイング＋呼び出し地点 `⇐` 照合という**意味記述は正確**だが、「`.rb` 本体にコメントで書く簡略形」は実態と正反対） |
| 修正案 | 「RBS ファイルの `def` 行に書く `%a{rigor:v1:param: x is String}` ディレクティブ」と訂正。「RBS を別に書かずにすむ簡略形」は誤りなので削除（簡略形が欲しければ別プラグイン `rigor-rbs-inline` の `# @rbs` で、ただしそれは RBS::Extended ではない、と分けて書く） |

### E2. `assert:` / `param:` を `# @rigor …` コメントで `.rb` に書く例（Part 7 コラム）

| 項目 | 内容 |
|---|---|
| 本書の記述 | Part 7 §7-5 コラム：`# @rigor assert: self is String` を `def normalized_string` の本体冒頭に置く例。表でも `# @rigor param: x String` / `# @rigor assert: self is String` を挙げる |
| 一次情報での実態 | (1) 構文：実物は `%a{rigor:v1:assert: self is T}` の **RBS 注釈**。`docs/handbook/07-rbs-and-extended.md` L383, `docs/type-specification/rbs-extended.md`。(2) `# @rigor …` という**自由形式コメント形式は ADR-1 で明示的に Rejected**：`docs/adr/1-types.md` L775「Free-form `# @rigor ...` comments in `*.rbs` \| Rejected」。(3) 置き場：`.rbs` の `def` 行であり `.rb` 本体ではない（E1 と同じ ADR-5 設計判断） |
| 判定 | **ERROR**（構文・置き場ともに不正確。「手動で事実を挿入する脱出口」という思想は正しいが、形式が架空） |
| 修正案 | コラムのコード例を RBS 形式に：`assert_connected!: () -> void` の `def` 上に `%a{rigor:v1:assert: self is Connected}`、と書く。表の `# @rigor …` 3 行も `%a{rigor:v1:…}`（RBS）に統一。`is_a?` 行は推論事実なので「コード上の述語」として残してよい |

### E3. `assert:` の意味が「本体内の self ナローイング」ではなく「呼び出し元スコープへのアサーション」（Part 7 コラム）

| 項目 | 内容 |
|---|---|
| 本書の記述 | Part 7 §7-5 コラム：`# @rigor assert: self is String` を `normalized_string` に付けると「**ここからブロック終端まで**、Rigor は self を String として扱う」。意味は「`self` の `local_binding` に `is String` を強制挿入」 |
| 一次情報での実態 | `%a{rigor:v1:assert: <name> is <type>}` の意味は **「このメソッドが return した後、*呼び出し元*スコープの `<name>` が `<type>` になる」**（`docs/handbook/07-rbs-and-extended.md` L125, L154–172 の assertion gate 例：`Validator.new.assert_non_empty(host)` の**後**で `host` が narrow される）。`self` 形も同様に**アサーションメソッドを*呼んだ側*の self/受信側**を narrow する（同 L377–386, `Fact(target_kind: :self)`）。「そのディレクティブが乗っているメソッド自身の本体を narrow する」のではない |
| 判定 | **ERROR**（caller-side ゲートを body-side 内省と取り違え。脱出口という大枠は合っているが効く場所が違う） |
| 修正案 | 「`assert_string!(x)` のような**アサーションメソッドを呼んだ直後**、呼び出し元の `x`（や受信側 `self`）が `String` として扱われる」と訂正。本体冒頭で自分自身を narrow する例は誤解を招くので差し替える |

### E4. JSON HKT の URI 名・arity が架空（Part 4 コラム）

| 項目 | 内容 |
|---|---|
| 本書の記述 | Part 4 §4-5 本文・コラム：`JSON.parse(s)` → `App[:"json::value", []]`（**空 args**）、`JSON.parse(s, symbolize_names: true)` → `App[:"json::symbolized_value", []]`、還元結果のキー型を `Symbol` に切替 |
| 一次情報での実態 | 実物は **単一 URI `json::value`、arity 1**（key 型 `K` でパラメータ化）。`lib/rigor/builtins/hkt_builtins.rb` L20–30, L39–44, L62–69（`arity: 1`）。`JSON.parse(s)` → `App[json::value, [String]]`、`symbolize_names: true` は**discriminator が引数 K を Symbol に差し替える** → `App[json::value, [Symbol]]`（同 L141–146 `args: ["String"]`, L246–253 `discriminated_args` → `[Nominal("Symbol")]`, L255–269 `json_symbolize_names?`）。`json::symbolized_value` という URI は**存在しない**。`App[…, []]` の空 args も誤り |
| 判定 | **ERROR**（「`symbolize_names: true` リテラルでキーが Symbol になる」という**観測可能な挙動は正確**だが、内部表現＝URI 名と arity が架空。Part 4 は「Rigor の別解を*事実として*解説する」発展ノートなので、ここは縮図ではなく事故的不正確さ） |
| 修正案 | `App[:"json::value", [String]]` ／ `symbolize_names: true` で `App[:"json::value", [Symbol]]` に統一（URI は 1 本、key 型引数 `K` が String↔Symbol で切り替わる、と書く）。「`json::symbolized_value`」「空 `[]`」は削除 |

---

## 要確認（一次情報で確証取れず／カテゴリ断定が強すぎ）

### Q1. 「関数型は戻り共変・引数反変（S-Arrow）を実装済み」（Part 2 §2-7）

| 項目 | 内容 |
|---|---|
| 本書の記述 | Part 2 §2-7：「関数型は戻り共変・引数反変（S-Arrow）を実装済み」 |
| 一次情報での実態 | `lib/rigor/type/` に **Proc/Arrow/Callable の構造化キャリアは存在しない**。RBS の proc 型は `Type::Combinator.nominal_of(Proc)` に**erase**される（`lib/rigor/inference/rbs_type_translator.rb` L66, L195；`docs/type-specification/rbs-compatible-types.md` L31「proc type → Callable object type → Same after erased operands」）。`acceptance.rb` に `(A)->B <: (A')->B'` の一般部分型判定は無い。covariant-return／contravariant-param が実装されているのは**メソッドシグネチャの override/conformance 検査**（ADR-35, `lib/rigor/rbs_extended/conformance_checker.rb` L23–24, L118–155）であって、第一級関数型の部分型ではない |
| 判定 | **要確認（実質オーバークレーム寄り）**。「実装済み」は、proc を構造化部分型する機構としては**不正確**。実態は「メソッドの**継承契約**（override）で引数反変・戻り共変を検査する（ADR-35）」 |
| 修正案 | §2-7 を「関数（proc）型そのものは nominal `Proc` に erase される。引数反変・戻り共変は **メソッドの override 互換性検査**（ADR-35）として実装されている」に訂正。S-Arrow 自体の解説（§2-3）は理論として正しいので維持してよいが、§2-7 の「実装済み」主張は対象をすり替えている |

### Q2. 「実 Rigor は PHPStan 風の `int<1,10>` 記法は採らず `Integer[1..10]` を使う」（a2-6）

| 項目 | 内容 |
|---|---|
| 本書の記述 | a2-6 脚注：「実 Rigor は PHPStan 風の `int<1,10>` 記法は**採らず** `Integer[1..10]` を使う」。PHPStan 表でも `int<m, n>` → `Integer[1..9]` と対応 |
| 一次情報での実態 | 混在している。`docs/type-specification/imported-built-in-types.md` L18：「range notation, such as `Integer[1..10]`. PHPStan-style `int<1, 10>` MUST NOT be added as an alias initially」（本書側を支持）。**しかし** RBS::Extended ディレクティブの refinement-name 文法は `int<5, 10>` / `int<min, max>` を実際に**受理**（`docs/handbook/07-rbs-and-extended.md` L134, L148）、かつ `IntegerRange#describe` の**人間向け出力は `int<min, max>`**（`lib/rigor/type/integer_range.rb` L96–99）。「採らない」と言い切るのは強すぎ |
| 判定 | **要確認（断定が過剰）**。値型表記は `Integer[1..10]`、ディレクティブ語彙と describe 出力は `int<min, max>` ―文脈で両方使う |
| 修正案 | 「型値の表記は `Integer[1..10]`。一方ディレクティブの refinement 名や人間向け出力では `int<min, max>` も使う（PHPStan の `int<m, n>` 由来の命名）」と両用を明記。「採らない」は削除 |

---

## OK（簡略化・縮図 ― 乖離ではないが重要なので明記）

### S1. dispatch「5 段カスケード」（a3-3）は実物の縮図

実物の `MethodDispatcher` は **5 段より多い**（const-fold → shape → plugin-contribution → HKT-builtin →
static-refinement → RBS → synthetic-method(ADR-16) → in-source → fallback：`lib/rigor/inference/method_dispatcher.rb`
L38–59 ＋本体 L95–150）。本書の 5 段（① 定数畳み込み → ② shape → ③ RBS → ④ in-source → ⑤ fallback）は
**順序関係を保った忠実な部分集合**で、「素朴／実物」の橋渡しとしての縮図。判定 **OK**。
ただし a3 冒頭が「5 段の順序・名称は原稿どおり」と*精確さ*を匂わせるので、「主要 5 段（実際はプラグイン・HKT 等の
中間段がさらに挟まる）」と一言添えると親切（nitpick 級）。なお「③ RBS が ④ in-source に勝つ＝宣言優先」は
実物の WD13（user-authored RBS overrides …, method_dispatcher.rb L142）と整合。

### S2. FactStore の例示コード（Part 6 §6-3 `examples/fact_invalidation.rb` 抜粋）は教育用スケッチ

`examples/README.md` が「教育用の設計スケッチで、本物の Rigor のコードではありません」と明記。
6 バケツ名・`invalidate_target(buckets:)` という**API 名・概念は実物と一致**させつつ、実装は最小化。判定 **OK**。

### S3. Part 4 等価判定スケッチが「簡約版」と明記（Part 4 §4-4「正確には」 box）

`seen` 判定を `naive_eq`（展開しない α 同値）にした健全な簡約版だと本文が自己申告。TAPL 21 章の本式
（展開後ペアの最大不動点）より弱いと断り済み。判定 **OK（断り済みの意図的簡略化）**。

---

## nitpick

### N1. `Dynamic[Top]` の大文字 T（巻全体）

本書は一貫して `Dynamic[Top]` と表記（a1-1 で「`[Top]` は内部表記」とまで述べる）。実 Rigor の内部表記は
**小文字 `Dynamic[top]`**、ボトムも `bot`（`docs/type-specification/value-lattice.md` L28, L61；`bot`/`top` 小文字）。
意味は同じだが「内部表記」と称する以上、`Dynamic[top]` / `bot` / `top` に揃えるのが正確。一貫した casing 差なので
一括置換で対応可。

### N2. ADR-3 を「二層構成」（a2-6 脚注）

ADR-3 自身の語は **"two-tier hybrid"** だが、実際は carrier 3 種（点除去 `Difference`／述語部分集合 `Refined`／
範囲整数 `IntegerRange`）に分岐（`docs/adr/3-type-representation.md` L105–117）。本書は「二層構成」と言いつつ
直後に 3 種を正しく列挙しているので実害は小。「二層（実体は 3 キャリア）」程度に。

### N3. a3-1 の `--explain` 出力メッセージ文言

本書の例 `info: fell soft to Dynamic[Top] here (...)` は**例示として妥当**（`--explain` が fail-soft を
`:info` で出すのは事実：`lib/rigor/cli.rb` L396, `docs/handbook/08-understanding-errors.md` L283–286）。
ただし実出力の正確な文言は未確認なので、断定的な再現出力に見えすぎないよう「例：」を明示すると安全。

---

## 裏取りした主な一致点（参考・修正不要）

- **ExpressionTyper / accepts / AcceptsResult / Scope / FactStore / MethodDispatcher / Refined / Intersection / App**
  … いずれも `lib/rigor/` に実在（Part 1・Part 8 §8-1 の対応表は正確）。
- **robustness principle = Postel の法則 = ADR-5** … `docs/adr/5-robustness-principle.md` 一致。
- **capability role `_ToS` / `_Each[T]` / `_Reader`** … `docs/type-specification/structural-interfaces-and-object-shapes.md`
  L150–177、`robustness-principle.md` L76–77 に実在（Part 5）。
- **fail-soft**（綴り）… 全箇所正しい。前編の `fall-soft` 同型ミスは後編に**無し**。
- **`Dynamic[top]` は untyped、トップ型ではない（軸 A/軸 B）** … `value-lattice.md` L28「untyped is deliberately
  outside the ordinary value lattice」、L57 で `Dynamic[String]` narrowing も整合（a1-1）。
- **`void` は値位置で `top` に materialize** … `docs/type-specification/special-types.md` L89（a1-2）。
- **unreachable arm = `flow.unreachable-clause` = ADR-47** … `docs/adr/47-…` 題一致（a1-3）。
- **`Type#erase_to_rbs` / `RbsTypeTranslator.translate(…, type_vars:)`** … `lib/rigor/type/*.rb`、
  `lib/rigor/inference/rbs_type_translator.rb` L84–91（Part 3）。
- **ADR-20 軽量 HKT・`Type::App`・fuel default 64・json::value 再帰 union（実装済み）** …
  `docs/adr/20-…` L1–30、`lib/rigor/inference/hkt_registry.rb` L164（Part 4）。
- **ADR-41 推論予算「Status: Proposed・未実装」** … `docs/adr/41-…` L3「Nothing here is implemented yet」（Part 7・a4）。
- **FactStore 6 バケツ・6 フィールド・invalidate_target(buckets:)・closure escape→dynamic_origin** …
  `docs/internal-spec/inference-engine.md` L52–61, L505（Part 6 ― 逐語一致）。
- **join：型 union／事実は保守的合流** … `inference-engine.md` IfNode 規則、`control-flow-analysis.md` L25（Part 6）。
- **sig-gen `--params` 既定 untyped・observed は非既定（ADR-5）・TypeProf appendix** …
  `docs/handbook/11-sig-gen.md` L158–168、`appendix-typeprof.md`（Part 5）。
- **dispatch tier 順序（const-fold → shape → … → RBS → in-source → fallback）・RBS が in-source に優先** …
  `method_dispatcher.rb` L38–150（a3-3）。
- **`rigor type-of FILE:LINE:COL` が「内部精密型＋RBS erasure」を出す** … `cli.rb` L28/L958、
  `cli/type_of_command.rb` L21–24（a3-2）。
- **refinement carrier の二層+IntegerRange（ADR-3 Option C）・`literal-string`・`non-empty-literal-string`=交差** …
  `docs/adr/3-…` L105–117、`imported-built-in-types.md` L25, L37（a2-6）。
- **Part 8 / a4 の全 ADR 番号・題**（0/2/4/5/6/9/14/15/16/19/20/22/25/27/32/37/41/44/45/46/48/50/51）… 全一致。
