# 型理論エキスパート査読ノート（chibirigor v1 / ja）― 2026-06-11 フルサイクル

レンズ：TAPL を教えられる水準の型理論家。形式的・技術的正確さの査読。
対象：前編 little/part0–9＋README、共通 README/glossary/appendix a1–a5、後編 seasoned 全 part。
方針：honest な簡略化（やさしい前編の意図的単純化）は咎めない。重大度＝ERROR / MISLEADING / 表記 / nitpick。

総括：**形式的な ERROR は検出されず。** 部分型・変性・gradual consistency・余帰納・α 変換・
erasure・健全性（progress/preservation）・正規化の扱いはいずれも教科書的に正しく、honest な
簡略化として明示されている。TAPL/『しくみ』の参照番号も概ね正確（下表で 1 件だけ要確認）。
Rigor 実装との整合も主要点を実ソースで確認し、一致した（fuel=64、acceptance.rb 一律共変＋
Slice 5+ 未実装、ADR-35 override、`non-empty-string`=`Difference[String,""]`、json::value arity 1）。
以下は MISLEADING 寄りの精密化提案と表記の微修正のみ。

---

## MISLEADING（条件付きでしか正しくない／誤読を招く）

| 該当箇所（ファイル:節） | 引用 | 問題 | 修正案 | 重大度 |
|---|---|---|---|---|
| seasoned/part5 §5-4 box「① 決定不能性」 | 「ランク 3 以上の多相型推論は決定不能です（Kfoury–Wells 1994; Wells 1999 は無制限 System F の決定不能性）」 | 2 つの別個の結果を 1 文に束ねていて因果が曖昧。**ランク k≥3 の型推論（typability）の決定不能性は Wells 1994/1999（System F typability undecidable）**であり、Kfoury–Wells はランク 2 が**決定可能**であること等で知られる。「Kfoury–Wells 1994」を決定不能性の根拠に並置すると逆の含意（ランク 2 も不能）に読めうる。また HM 本体はランク 1 で**決定可能**なので「HM をフル活用＝決定不能領域」は「HM の高階多相拡張」の意。 | 「ランク 2 までは決定可能（Kfoury–Wells）だが、**ランク 3 以上の型推論は決定不能（Wells 1994/99, System F typability）**。HM をそのまま高階多相へ広げると、この決定不能領域に入る」と、決定可能境界（rank 2）を明示して言い換える。 | MISLEADING |
| seasoned/part4 §4-4 本文＋「正確には」注 | スケッチの `seen` 判定が `naive_eq`（**展開しない** α 同値）で、本式の余帰納（展開後ペアを seen に積む最大不動点）より弱い、と注で断っている。一方本文は「これが余帰納の心」と言い切る | 注で honest に簡約版だと明示済み（ERROR ではない）。だが本文の「seen に覚えて同じペアを再び問われたら打ち切る」と、スケッチが**展開前**のペア `seen + [[s,t]]` を積む点に微ズレ。読者が標準（展開後ペアを積む）と取り違えうる。 | 本文側にも「（このスケッチは展開前ペアで簡約。本式は展開後ペア＝後述）」を一言。注の存在で MISLEADING を軽に。 | MISLEADING（軽） |
| a1-4 格子図（part2 §2-1 と同図） | `{name:}`・`Integer`・`String` を同一段に置き直下が `Bot` | §2-2 で `{name:,age:} <: {name:}`（幅で下に伸びる）と言うため、`{name:}` を極大要素のように描くと §2-2 と緊張。part2 の図には「幅部分型で下に伸びる」補記があるが a1-4 にはない。 | a1-4 の格子図にも part2 と同じ「`{name:}` は幅部分型で下に伸びる（極大ではない）」の一言を添える。 | MISLEADING（軽） |
| seasoned/part2 §2-3→§2-7 | §2-3 で S-Arrow を「動く Ruby」の subtype として一般提示 → §2-7「proc 型は nominal Proc に erase、S-Arrow は override 互換性検査（ADR-35）として実装」 | 実装的には正しい（acceptance.rb・ADR-35 で確認）。だが連続して読むと「§2-3 の subtype が Rigor の proc 部分型そのもの」と誤解しうる。S-Arrow が**第一級 proc 値**には適用されず**メソッド override**にのみ効く、という適用範囲の限定をもう一段明示すると安全。 | §2-3 末か §2-7 で「S-Arrow は chibirigor スケッチでは第一級関数値に効くが、Rigor 実体では proc は erase され、S-Arrow は override 検査でのみ働く」と適用範囲を一文で対比。 | MISLEADING（軽） |

---

## 表記・nitpick（少数）

| 該当箇所 | 指摘 | 重大度 |
|---|---|---|
| seasoned/part7 §7-4 表「① untyped 受理 → progress を放棄（`~` が非推移＝subsumption が漏れる）」 | progress 放棄の機構は「consistency が `<:` でなく素通りさせる」ため。`~` の非推移性は別性質（抜け穴を**限定する**側、§7-5 で正しくそう置く）。表のセルだけ「consistency が照合を素通り」に寄せると因果が整合。 | 表記 |
| glossary「gradual consistency … 対称・非推移」 | 正しいが consistency は**反射的**かつ対称だが非推移。反射性も一言あると初学者の誤解（`untyped ~ untyped`?）を防げる。任意。 | nitpick |
| seasoned/part3 §3-5「TAPL 23.7 の erasure 定理」 | 版で節番号が 23.6/23.7 と揺れる（標準版は 23.7 "Erasure and Evaluation Order"）。問題なし、念のため記録。 | nitpick |
| docs/…-rigor-mapping.md（内部メモ・本書外） | `App[:"json::value", []]`（arity 0）と記すが、実装・後編 part4 は arity 1（`[String]`）。**本書本文は正しい**。内部メモのみ stale。 | nitpick |
| seasoned/part5 §5-3 / examples/unification.rb | 本文 `unify` は occurs-check 省略。スケッチ側コメントで明示済み・TVar/TCon のみで自己チェックは健全。本文に一言あると親切だが honest 簡略化として許容。 | nitpick |

---

## 確認して問題なかった主要点（記録）

- 双方向：synthesize⇒/check⇐、subsumption(Sub)、Var-Synth、診断が⇐位置に限る論理（part1）。
  「合成が untyped に全域化＋照合が untyped を素通り」で working code を守る説明は形式的に正しい。
- 部分型：S-Refl/S-Trans、width/depth（キー多が部分型）、depth 共変、S-Arrow（引数反変・戻り共変）、
  アルゴリズム的部分型付け。subtype.rb のコードも正しい（part2）。LSP/robustness/S-Arrow 収束（§2-3a）妥当。
- ジェネリクス：型抽象/適用、subst の 2 落とし穴、fresh で α 変換、α 同値の名前対応表、
  erasure（意味論 vs Java 型消去の区別）。subst.rb 正しい（part3）。
- 再帰型：μ unfold/fold、iso- vs equi-recursive、余帰納＝seen、最大不動点（TAPL 21）、HKT/App＋fuel（part4）。
- 推論：道A（capability/duck）／道B（型変数＋制約＋単一化＝TAPL 22 / HM 骨子）、let 多相を 23章/System F と
  正しく切り分け。TypeProf 対比（whole-program vs local+catalog）（part5）。
- FactStore：6 バケツ、stability、join＝「データフロー解析の慣用語、事実格子では meet」という正しい注記（part6 §6-5）。
- 健全性：progress＋preservation（TAPL 8.3）、正規化（TAPL 12, Ω が単純型で不能）、わざと unsound＝progress 放棄、
  gradual consistency＋gradual guarantee の 2 規律（part7）。
- 特別な型：untyped=Dynamic[Top]（軸A 格子位置／軸B チェック有無の分解）、void（top-like・戻り限定マーカー）、
  never=Bot（Bot<:T）。他言語対応表も妥当（a1）。無タグ Union vs タグ付き variant の区別（part4/a5-4）honest。
- Rigor 実装整合（実ソース確認）：DEFAULT_FUEL=64（hkt_reducer.rb:43）、acceptance.rb 一律共変＋
  「declared variance lands in Slice 5+」、ADR-35=override-signature-compatibility、
  `non-empty-string`=`Difference[String,""]`（intersection.rb:12）、json::value arity 1（hkt_builtins.rb）。
