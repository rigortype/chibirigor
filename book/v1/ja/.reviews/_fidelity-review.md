# L1 真・Rigor フィデリティ 査読（2026-06-11）

査読観点：本書の「実 Rigor では…／Rigor の中では…」という**事実記述**だけを、Rigor
チェックアウト（`/Users/megurine/repo/ruby/rigor`、read-only・未編集）の一次情報と突き合わせて
一致／不一致を判定する。型理論の正しさ・日本語・読み味は別レンズ担当。意図的な簡略化（続編送り・
設計ドラフト記録済み）は乖離としない。

---

## 総評

**結論：本書の Rigor 事実記述は総じて高精度。** 検証した一次情報すべて（FactStore の 6 フィールド・
6 バケツ、dispatch カスケードの相対順序、変性の実装状態、override 互換性 ＝ ADR-35、`type-of` の
2 段表示と erasure、refinement carrier の二層構成、ADR-22 baseline、ADR-47 unreachable-clause、
`RbsTypeTranslator.translate(type_vars:)`、proc erase）で本書の断言が実装・spec と一致した。
**事故的乖離は 1 件のみ**（`rigor:v1:assert` ディレクティブのコロン）。残りは意図的簡略化として
正しく枠付けされている。

---

## 観点ごとの所見

| 引用（本書） | 問題 | 実 Rigor の実態 | 修正案 | 重大度・区分 |
|---|---|---|---|---|
| seasoned/part7 L161,174・`%a{rigor:v1:assert: x is String}`（`assert` の直後にコロン） | ディレクティブ動詞の表記が実構文と食い違う。読者がそのまま写すと無効構文になる | `lib/rigor/rbs_extended.rb` L21-23 と `docs/type-specification/rbs-extended.md` L18,63,78-80 は一貫して `rigor:v1:assert <target> is <T>`（**`assert` の後にコロンなし**）。コロンを取るのは `param:`/`return:` のみ。`assert`/`assert-if-true`/`assert-if-false`/`predicate`/`conforms` はコロンなし | `assert:` → `assert`（コロン削除）。part7 L161・L174 の 2 箇所。なお同 part1 L237 の `param:` は正しい（実構文どおり） | **ERROR（事故的乖離・軽微だが構文として誤り）** |
| a3-tooling §a3-3「dispatch 5 段カスケード（① 定数畳み込み → ② shape → ③ RBS → ④ in-source → ⑤ fallback）」 | 実際は ~12 段（plugin contribution・HKT builtin・static refinement・synthetic・project-patched・dependency-source・discovered・stub・user-class fallback…） | `lib/rigor/inference/method_dispatcher.rb#resolve`。ただし本書が断言する**相対順序は保持**：定数畳み込み < shape < RBS < in-source(`try_user_method_inference`) < fallback。RBS が in-source に勝つ（L256-258 のコメントが明言）も正しい | 修正不要。5 段は中間段を畳んだ意図的簡略化で、§a3-3 冒頭・Part 2 L284 で続編送りと明記済み | 乖離ではない（簡略化） |
| a3-tooling §a3-2・`type-of` が「内部の精密型 ＋ 境界で丸めた保守型」の 2 段を見せ、境界操作を **erasure** と呼ぶ | — | `lib/rigor/cli/type_of_renderer.rb` L28-29 が `type:`（`describe`）と `erased:`（`erase_to_rbs`）を併記。`erase_to_rbs` メソッドが各 carrier に実在。`Constant<3>` 表記も manual 05 で一致 | 修正不要・正確 | 乖離ではない（正確） |
| seasoned/part6・FactStore の Fact は `bucket/target/predicate/payload/polarity/stability` の 6 フィールド、バケツは 6 種 | — | `lib/rigor/analysis/fact_store.rb` L32 の `Fact#initialize` が正にこの 6 フィールド。L13-18 が 6 バケツ（`local_binding/captured_local/object_content/global_storage/dynamic_origin/relational`）。`docs/internal-spec/inference-engine.md` L56 も同一。脚注「6 つ目として並ぶ」も正しい | 修正不要・正確 | 乖離ではない（正確） |
| seasoned/part2 2-7・「現実装では Nominal の型引数を一律共変、宣言サイト変性は設計済み・未実装」「proc 型は nominal Proc に erase」「戻り共変・引数反変はメソッド override 互換性（ADR-35）」 | — | `lib/rigor/inference/acceptance.rb` L404-405「treated covariantly element-wise (gradual default; declared variance lands in Slice 5+)」が一字一句一致。`rbs-compatible-types.md` L31 proc=「Same after erased operands」。ADR-35 L8-10,101-102 が param 反変・return 共変を実装と明記 | 修正不要・正確 | 乖離ではない（正確） |
| glossary・appendix a2-6・「refinement carrier は二層構成：点除去 `Difference`／述語部分集合 `Refined`／範囲整数 `IntegerRange`（ADR-3）」 | — | `lib/rigor/type/{difference,refined,integer_range}.rb` 実在。`refined.rb` L9-13「predicate-subset half … Difference carries the point-removal half」が二層を裏付け。ADR-3 L67,99-103 が 3 carrier を記録 | 修正不要・正確 | 乖離ではない（正確） |
| appendix a2「値の表記は `Integer[1..10]`、`int<min,max>` はディレクティブ語彙・内部表示で使う」 | — | `integer_range.rb` L96-99 `describe` が `int<...>`（内部表示）。`rbs-extended.md` L26 が directive で `int<min, max>` 受理。`imported-built-in-types.md` L18 がカタログ値表記を `Integer[1..10]` と規定（`int<1,10>` を alias にしない）。3 つの使い分けが正確 | 修正不要・正確 | 乖離ではない（正確） |
| little/part9 L96-100・「実 Rigor の baseline（ADR-22）はデフォルトでルール ID で照合、行番号はキーに含めない、列は見ない」 | — | ADR-22「PHPStan-shaped」。`docs/internal-spec/baseline.md` L18,40 ― baseline 行は tuple キーで `match_mode` により粒度可変。ルール ID 基調・列非依存の趣旨と整合 | 修正不要 | 乖離ではない（正確） |
| appendix a1-2・「unreachable arm／空集合の扱いは実 Rigor の ADR-47（`flow.unreachable-clause`）の縮図」 | — | ADR-47 実在。narrowing が subject を `Type::Bot` に絞る→`body_scope == bot` で発火、の記述が L5,87-100 と一致。balanced は `:info` 既定 | 修正不要 | 乖離ではない（正確） |
| seasoned/part1 1-5・「`ExpressionTyper`＝合成、呼び出し地点の `accepts`＝照合、`Scope`/`FactStore`＝Γ」 | — | 各コンポーネント実在（`lib/rigor/inference/expression_typer.rb`・`acceptance.rb`・`analysis/fact_store.rb`）。役割対応も妥当 | 修正不要 | 乖離ではない（正確） |
| seasoned/part3 3-6・「型代入は `RbsTypeTranslator.translate(..., type_vars:)`、erasure は `Type#erase_to_rbs`」 | — | `lib/rigor/inference/rbs_type_translator.rb` L89 `translate(rbs_type, …, type_vars: …)` 実在。`erase_to_rbs` 各 carrier 実在 | 修正不要・正確 | 乖離ではない（正確） |
| seasoned/part5 5-4a・TypeProf（whole-program 抽象解釈・call site から引数推論）vs Rigor（local+catalog・推論予算）。sig-gen は ADR-5 で観測型をデフォルトにしない | — | local+catalog 方針・推論予算が ADR/spec と整合。`lib/rigor/cli/sig_gen_command.rb` 実在。appendix-typeprof 対比とも整合 | 修正不要 | 乖離ではない（正確） |
| seasoned/part8 8-2・「chibirigor の 1 点レジストリは Rigor ADR-2 拡張 API の骨格、本物は ADR-9・37 でプラグイン間連携が加わる」 | — | ADR-2/9/37 実在（`docs/adr/`）。`lib/rigor/plugin/` 実在 | 修正不要 | 乖離ではない（正確） |
| little/part4・Union 読み出しは「全メンバを回して一番弱い結論を採る」 | — | 直近 lib に入った Union ディスパッチ（レシーバ分配＋メンバ直積畳み込み）は **chibirigor 側**のコミット。本書は Rigor について偽を断言していない。Rigor の Union 受理も最弱結論を採る方針で整合 | 修正不要 | 乖離ではない（正確） |

---

## 補足

- 本書が「実 Rigor では…」と断言する事実記述で、実装・spec と矛盾するものは **`assert:` コロン 1 件のみ**。
- 5 段カスケード・「6 バケツのうち 5 つは対象で分ける」図など、簡略化箇所はいずれも本文・脚注・付録で
  「本書では素朴／実物はこう」と枠付けされており、軸（やさしい前編／形式的な後編・gradual・脅かさない）
  を壊していない。
