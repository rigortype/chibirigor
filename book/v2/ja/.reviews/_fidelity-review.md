# Rigor フィデリティレビュー — part1-bidirectional-typing

## 所見

| 重大度 | 箇所（§・引用） | 実Rigorの実態 | 問題・乖離 | 修正案（または「簡略化として許容」） |
|---|---|---|---|---|
| OK | §1-5「**`ExpressionTyper`**（式の型付け）＝ 合成`⇒` / 純粋かつ非破壊で、式から型を組み立てる」 | `lib/rigor/inference/expression_typer.rb` 実在。クラスdoc冒頭に "Translates AST nodes into Rigor::Type values … **Pure: never mutates the receiver scope.**" と明記。`type_of` がノード→型を組み立てる入口。 | 乖離なし。「純粋かつ非破壊」「式から型を組み立てる」は実装コメントとそのまま一致。 | 修正不要。 |
| OK | §1-5「呼び出し地点での`accepts`（受理判定、**三値 ＋ 理由**）＝ 照合`⇐` / RBSの宣言が期待型Tを与える所でだけ働く」 | `Type#accepts(other, mode:)`（`type/app.rb` / `type/acceptance_router.rb`）→ `Inference::Acceptance.accepts`。戻り値 `Type::AcceptsResult` が `trinary`（三値）＋ `reasons`（理由配列）を保持。call-site は `overload_selector.rb:266,413` で `param_type.accepts(arg, mode: :gradual)` を呼び、`call.argument-type-mismatch`（`check_rules.rb:62`）診断を出す。 | 乖離なし。「受理判定・三値・理由・呼び出し地点・RBS宣言が期待型を与える所」すべて実態どおり。AcceptsResult のdocも "is `other` passable to `self` at a method-parameter or assignment boundary?" と照合方向を述べる。 | 修正不要。 |
| OK | §1-5「**`Scope` / `FactStore`** ＝ 環境 Γ（前編より遥かにリッチ。Part 6で扱う）」 | `lib/rigor/scope.rb`：`Scope` は `environment, locals, fact_store, self_type, …` を保持。`fact_store`（`analysis/fact_store.rb`）は "**Immutable storage for flow-sensitive facts** attached to a Scope snapshot"、6バケツ（local_binding/object_content/relational ほか）。 | 乖離なし。Γ を Scope＋FactStore の組で表すのは妥当で、FactStore がフロー感応・多バケツでchibirigorのScopeより遥かにリッチという記述も実態どおり。Part 6（§1-6「フロー感応な事実の集合へ拡張」）の予告とも整合。 | 修正不要。 |
| OK | §1-7-a NOTE「`%a{rigor:v1:param: name is String}` … (1) 本体ナローイング (2) 呼び出し地点の`⇐`照合 … `.rb`本体にはインラインで書けず、RBS側に置く」 | `lib/rigor/rbs_extended.rb`：`rigor:v1:param: <name> [is] <type>` を `ParamOverride` として読む。`docs/type-specification/rbs-extended.md:75`「Tightens the RBS-declared type of parameter `name` to `T`, **both at overload selection / argument-type checks AND inside the method body during inference**. The `is` glue word is optional.」アノテーションは RBS の `%a{…}` 内（`def` の上）に置く。 | 乖離なし。「2つの仕事＝本体ナローイング＋呼び出し地点の引数照合」が仕様文と逐語的に一致。`is` 任意・RBS側に置く点も正しい。`is String`（クラス名）形も rbs-extended.md:26 が "RBS-style class name (`String`…)" として明示的に許容（ハンドブック例は `non-empty-string` 等の refinement だが、クラス名形も等しく有効）。 | 修正不要。 |
| nitpick | §1-7-a NOTE「`is_a?`を書かなくても最初から宣言型で推論が進む」 | `param:` は overload 選択／引数チェックと本体推論の両方で型を締める（rbs-extended.md:75）。本体側は「宣言型から推論が始まる」効果。 | 乖離ではない。「`is_a?` ガード無しでも本体が宣言型から始まる」は本体ナローイングの正しい言い換え。ただし厳密には `param:` は RBS宣言型を「絞り込む（tighten）」もので、無宣言からゼロ生成するわけではない——本書の文脈（注釈ゼロのRubyに型の出発点を与える）では十分正確。 | 簡略化として許容。 |

## 総評

§1-5 と §1-7-a の事実記述は、Rigor 実装と仕様文書に照らしてすべて正確でした。`ExpressionTyper`（純粋・非破壊）、call-site の `accepts`（三値＝`AcceptsResult.trinary` ＋ 理由＝`reasons`）、環境 Γ を `Scope`＋`FactStore`（フロー感応・多バケツ）で表す対応、`param:` ディレクティブの「本体ナローイング＋呼び出し地点の引数照合」という二重の役割は、いずれもコードコメント・ADR・型仕様書の記述と逐語に近いレベルで一致しています。ADR-5（robustness principle ＝ Postel の法則、返り厳密／引数寛容）も §1-4 の枠組みを裏づけます。ERROR・MISLEADING 級の事故的不正確さは検出されず、唯一の指摘は「絞り込み」を「出発点を与える」と言い換えた nitpick 一点で、本書の意図的な平易化の範囲内です。
