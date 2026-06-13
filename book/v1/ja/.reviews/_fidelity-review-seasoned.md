# Rigor フィデリティレビュー（後編 v1）／2026-06-13 L1真

対象：`book/v1/ja/seasoned/`（README＋Part1〜8＋examples）。実 Rigor チェックアウト
`/Users/megurine/repo/ruby/rigor`（読み取り専用）の一次情報（`docs/internal-spec/`・
`docs/type-specification/`・`docs/adr/`・`docs/handbook/`・`lib/rigor/`）と突き合わせ、後編の
「実 Rigor では…／Rigor の中では…」という**事実記述**が実態と一致するかだけを検証した。
意図的な簡略化（最小版・設計スケッチ）は乖離としない。

## 総評

**後編の Rigor 主張は、検証した範囲すべてが実態と一致していた。ERROR はゼロ。**
最重要監査対象である FactStore（6 バケツ・6 フィールド・無効化・join・stability・クロージャ捕獲）
は内部仕様 `inference-engine.md` と*逐語*で一致する。HKT/fuel（既定 64・ADR-20）、推論予算
（ADR-41 Proposed＝未実装）、sig-gen の `untyped` デフォルト（ADR-5/14）、override の引数反変・
戻り共変（ADR-35）、`type_vars:`／`erase_to_rbs`／`ExpressionTyper`／`MethodDispatcher`／
`acceptance.rb`／`AcceptsResult` などのコンポーネント名・パス・キーワードは実在し正確。
ディレクティブのトレーリングコロンも `assert`（無）／`param:`（有）の区別を正しく書いている。
5 つの examples はすべて緑（本文の `run:` ブロックと逐一一致）。

指摘は **MISLEADING 1 件（`~` 記法）＋要確認 3 件**にとどまる。いずれも本書を脅かす乖離では
なく、注記・casing・例示の粒度の話。

## 判定表

| 該当箇所 | 本書の主張 | Rigor 実態（出典＋行） | 判定 | 修正案 |
|---|---|---|---|---|
| P6 §6-1 | Fact は `bucket`/`target`/`predicate`/`payload`/`polarity`/`stability` を持つ | `FactStore::Fact` MUST carry まさにこの 6 フィールド（`inference-engine.md:56`） | OK | — |
| P6 §6-2 | 6 バケツ＝`local_binding`/`captured_local`/`object_content`/`global_storage`/`dynamic_origin`/`relational` | 同名 6 種（`inference-engine.md:56`、`lib/rigor/analysis/fact_store.rb:13-18`） | OK | — |
| P6 §6-2 脚注 | `dynamic_origin` だけ「由来追跡」で毛色が違う | 実装上 `dynamic_origin` は `closure_escape` 等の provenance 用（`inference-engine.md:515`）。他 5 つは対象スコープ別 | OK | — |
| P6 §6-3/§6-6 | 不変ストア。`with_fact`/`invalidate_target` は新ストアを返す。バケツ指定無効化あり | `with_fact`/`invalidate_target(target, buckets:)`/`join` すべて新ストアを返す MUST（`inference-engine.md:57-59`） | OK | — |
| P6 §6-3/§6-4 | クロージャ捕獲：エスケープ／書き換えで `captured_local` を保守的に無効化、迷ったら消す→`Dynamic[Top]` | `ClosureEscapeAnalyzer`→`:escaping/:unknown` で `dynamic_origin:closure_escape` fact、捕獲外側ローカルの書きを `Dynamic[Top]` に drop、読みは保持（`inference-engine.md:515`、`statement_evaluator.rb:1948-`） | OK | — |
| P6 §6-5 | join は両枝で成り立つ事実だけ残す（型は union、事実は積） | `Scope#join`：型は `Combinator.union`、fact store は両エッジ共通の事実のみ（`inference-engine.md:44,59`） | OK | — |
| P5 §5-4a/§5-6 | sig-gen は観測 call site をデフォルトにしない（ADR-5）、引数は `untyped` 既定 | `--params` 既定 `untyped`、`observed` は opt-in（`sig_gen_command.rb:42,104,133`、ADR-14:154）。ADR-5 ＝ robustness principle（Accepted） | OK | — |
| P5 §5-4a | TypeProf＝whole-program 抽象解釈／Rigor＝local+catalog、call site 引数推論せず | handbook `appendix-typeprof.md:25,109,132,140` と逐一一致 | OK | — |
| P5 §5-3/§5-6x | 単一化・型変数・直接代入。一般の制約解きは設計スケッチのまま | `unification.rb` 緑。lib 5a/5b は chibirigor 本体（本書側）の話で Rigor 主張ではない | OK | — |
| P5 §5-6 | 解けない型変数は `Dynamic[Top]`（＝`untyped`）に degrade | fail-soft で `Dynamic[Top]`（`inference-engine.md:75`）、`Dynamic[top]`＝`untyped`（`value-lattice.md:28`） | OK | — |
| P5 §5-2/§5-6 | capability role（`_ToS`/`_Each[T]`/`_Reader`）が受理判定に使われる | role_conformance は flow-contribution に実装（`flow_contribution.rb:49,136`）、`_ToS`/`_ToStr` は ADR-2/5 のロール。広い stdlib カタログは一部 future（ADR-5:145） | 要確認 | `_Each[T]`/`_Reader` は例示ロール名。「核は実装、網羅カタログは設計途上」が伝わるなら可。現状の例示粒度で問題なし |
| P7 §7-5 | `%a{rigor:v1:assert x is non-empty-string}`（assert にコロン無）／`%a{rigor:v1:param: x is String}`（param にコロン有） | 正準 `%a{}` 形は `assert` コロン無（`CHANGELOG-0.0.x.md:364`）、`param:` コロン有（同:473,555） | OK | — |
| P1 §1-7-a | `%a{rigor:v1:param: name is String}`（コロン有）＝本体ナローイング＋呼び出し地点照合 | `param:` コロン有、本体＋call-site 2 効果（`CHANGELOG-0.0.x.md:473,555`、ADR-28:51） | OK | — |
| P7 §7-5 | assert＝ガード通過後、呼び出し元の `x` を強制（caller スコープに事実） | `rigor:v1:assert <target> is <Class>`＝post-call scope で引数のローカルを refine（`CHANGELOG-0.0.x.md:1087-1090`） | OK | — |
| P7 §7-6/P4 §4-5 | HKT 還元 fuel 既定 64、尽きれば `:maybe`/`untyped` | WD3：HKT-eval budget 既定 64 reduction steps、`hkt.budget-exhausted`（ADR-20:571-574,462-466） | OK | — |
| P7 §7-6 | 推論予算（ADR-41）は設計済み・未実装（Status: Proposed） | ADR-41 Status: **Proposed, 2026-06-03**（`41-...md:3`） | OK | — |
| P4 §4-5 | JSON は `App[:"json::value",[String]]`、`symbolize_names: true` リテラルで key 型引数 Symbol に | `json::value` 再帰 union、literal `symbolize_names: true` で K=String→Symbol（ADR-20:74-75,123-130,141） | OK | — |
| P4 §4-5 脚注 | HKT の一次根拠は TAPL 29 章（kinding）、Rigor は defunctionalize 版 | 型理論の出典帰属。ADR-20 は `App[F,A]`＋registry＋reducer の実装（実態と整合） | OK | — |
| P3 §3-6 | 型代入＝`RbsTypeTranslator.translate(..., type_vars:)`、`type_vars[:Elem]=String` | `translate(rbs_type, self_type:, instance_type:, type_vars:)`（`rbs_type_translator.rb:89`、`conformance_checker.rb:279`） | OK | — |
| P3 §3-6/§3-7 | erasure＝`Type#erase_to_rbs`、内部型を保守的に RBS へ（広くなるが狭くならない） | `erase_to_rbs` 実在（`sig_gen/generator.rb:560`,`hover_renderer.rb:278`）、`rbs-erasure.md`／`value-lattice.md:65` の round-trip と整合 | OK | — |
| P3 §3-6 | 有界量化は構造契約（interface/capability role）が一部担う | ADR-2/5 のロール機構と整合（TAPL 26 章は理論帰属） | OK | — |
| P2 §2-4/§2-7 | 現実装は Nominal 型引数を一律共変。宣言サイト変性は設計済み・未実装 | `accepts_nominal_args` は formal/actual を同方向 zip→`accepts`（`acceptance.rb:410-418`）。位置別の反変/不変処理なし | OK | — |
| P2 §2-7 | 戻り共変・引数反変は override 互換性（ADR-35）として実装 | ADR-35 Accepted：parameters contravariant / returns covariant（`35-...md:8-10,101-102`） | OK | — |
| P2 §2-7 | proc 型は nominal `Proc` に erase（第一級関数の部分型は持たない） | proc type は erased operands 後同値で扱う（`rbs-compatible-types.md:31`、`value-lattice.md:11`）。第一級関数 `<:` の独立実装は不在 | OK | — |
| P2 §2-7 | join/meet を `Combinator.union` として正規化で計算 | `Type::Combinator.union`（`combinator.rb:410`） | OK | — |
| P1 §1-5/§1-7 | `ExpressionTyper`（純粋・非破壊）＝合成、呼び出し地点 `accepts`＝照合、`Scope`/`FactStore`＝Γ | `lib/rigor/inference/expression_typer.rb` 実在、`acceptance.rb` 実在、Scope+FactStore 構成（`inference-engine.md:40-61`） | OK | — |
| P8 §8-1 | 受理判定＝`accepts`＋`AcceptsResult`（三値＋理由） | `AcceptsResult` は `Trinary`＋ordered `reasons`（`accepts_result.rb:31-38`） | OK | — |
| P8 §8-1 | メソッド解決＝`MethodDispatcher`（多段ティア・RBS・継承） | `lib/rigor/inference/method_dispatcher.rb`＋rbs/shape/iterator/kernel dispatch サブモジュール | OK | — |
| P8 §8-1 | 型キャリア多数（`Refined`/`Intersection`/`App`/…） | `refined.rb`/`intersection.rb`/`app.rb` ほか `lib/rigor/type/` に実在 | OK | — |
| P8 §8-2/§8-3 | ADR 索引（ADR-0/2/4/5/6/9/14/15/16/19/20/22/25/27/32/35/37/41/44/45/46/48/50/51） | 全 ADR 番号が `docs/adr/` に実在（README 索引）。`inference-engine.md` も実在 | OK | — |
| P2 §2-5・P7 §7-5 | gradual consistency を記号 `~` で表記（対称・非推移） | 性質（symmetric・非 transitive）は `relations-and-certainty.md:25,32` と一致。**ただし spec は `~` を gradual-consistency に*使わない*と明記し、`consistent(A,B)` と書く。`~T` は否定/補型に予約**（同:14） | MISLEADING | 記号衝突の注記を 1 行。例：「（実 Rigor の spec は混同回避のため `~` を使わず `consistent(A,B)` と書き、`~T` は否定型に予約している）」。学術慣例（Siek）の `~` 採用自体は誤りでない |

## 補足（軽微・要確認）

1. **`Dynamic[Top]` の casing**（要確認）：本書は `Dynamic[Top]`（大文字 T）、spec は `Dynamic[top]`
   （小文字）。表記ゆれだが意味は同一（`value-lattice.md:28,65`）。本書は格子の `Top`（⊤）と揃える
   意図と読め、フィデリティ上の事故ではない。修正は任意。

2. **ADR-47 の本文露出**（要確認）：プロンプトが重点に挙げた ADR-47（`flow.unreachable-clause`）は
   実在し Accepted（WD1–3a landed, v0.1.17、`47-...md:5,9`）だが、後編本文では §7-6 の停止性表・
   §8-2 の「誤検知を避ける」行に直接の ADR-47 引用はない。Part 7 が扱う「健全性/正規化/予算」とは
   別系統の機能なので、未引用は**乖離ではなく対象外の選択**。`flow.unreachable-clause` という診断
   コードに本書が言及していないこと自体に不正確さはない。修正不要。

3. **capability role の例示**（要確認）：`_ToS`/`_ToStr` は ADR-2/5 の実ロール、`_Each[T]`/`_Reader`
   は本書の例示ロール名。role_conformance 機構は実装済み（`flow_contribution.rb`）だが、広い stdlib
   カタログは ADR-5:145 が「future capability-role catalog」と書く設計途上領域。本書は最小版・例示
   として扱っているので断定の事故はない。

## 前編からの再確認項目

- erasure の a3/glossary 再アンカー（前編指摘）：後編 §3-5/§3-6 は `Type#erase_to_rbs`＋
  「RBS のスーパーセット・いつでも書き戻せる」を実態どおりに書いており、前編の再アンカーと矛盾なし。**OK（再確認）**。
- 軸（gradual・脅かさない・「TAPL 並み厳密化は不要」）：§5-5/§5-7・§7-4 の「精度が上がっても診断は
  照合位置でしか出ない」「わざと unsound は設計の選択」は軸を保持。**OK（再確認）**。
