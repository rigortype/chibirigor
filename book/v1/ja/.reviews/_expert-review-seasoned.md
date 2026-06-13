# 型理論エキスパートレビュー（後編 v1）／2026-06-13 L1真

後編 The Seasoned chibirigor（Part 1〜8 ＋ README ＋ examples）を、形式的・技術的正確さの観点で査読した。
横断参照として glossary.md・appendix a1/a3-2/a4、前編 little/、lib/chibirigor/、Rigor チェックアウト
（ADR-5/20/35/41/47, acceptance.rb）も事実確認に用いた。`examples/*.rb` 全 5 本と `check_docs.rb` は緑。

## 総括（先に結論）

**形式的 ERROR はゼロ。** 探した「条件付きでしか正しくない記述」も実害のあるものは無く、見つかったのは
参照の精密化に値する nitpick・MISLEADING 寄りの軽微点が数件のみ。後編が掲げる「TAPL 並みの厳密さは目指さず、
explicit に断った簡略化は咎めない」という設計方針に照らして、**この巻は形式的に信頼できる**。

特に良かった点（簡略化の正しさが守られている所）：

- 双方向（Part 1）：subsumption `(Sub)` と Var-Synth を正しく書き、「Var が x∉Γ で失敗する」TAPL 規則を
  chibirigor が `|| Dynamic` で全域化している差分を「脚注つき」と明示。診断が `⇐` でしか出ない理由を
  「合成の全域化＋照合の寛容」の 2 点に正しく分解（lib の `accepts.rb` が `Dynamic→:maybe` で裏付く）。
- 変性（Part 2）：S-Arrow の引数反変・戻り共変、width/depth、アルゴリズム的部分型付け（反射・推移の吸収）が
  正確。`examples/subtype.rb` の `subtype(tp, sp)` 入れ替えが反変の実証になっている。Rigor 実装の注（Nominal
  一律共変・宣言サイト変性未実装・S-Arrow は override 互換 ADR-35）は acceptance.rb / ADR-35 と一致。
- 代入（Part 3）：シャドーイング（params.include? で停止）と変数捕獲（fresh α 変換）の 2 落とし穴を正しく
  分離。TAPL 23.7 erasure 定理と Java の型消去を別物と明示し、さらに Rigor の `erase_to_rbs`（境界の保守的
  変換）を「もう一つの意味」と断った上で重ねている — 三者の混同を防いでいる。
- 再帰型（Part 4）：iso/equi（TAPL 20.2）、余帰納＝最大不動点（TAPL 21）、HKT の一次根拠が再帰型章ではなく
  TAPL 29（カインド）である、を正しく区別。スケッチの `seen` 判定が α 同値ベースの「健全な簡約版」で本式
  （展開後ペアの最大不動点）より弱い、と自己申告している誠実さは特筆に値する。
- 推論（Part 5）：道 A（capability）/道 B（制約＋単一化）、`(X)->X` がジェネリックに残る話、occurs-check を
  省いたスケッチへの明示注記、HM ランク境界（後述）、TypeProf 対比が正確。
- 健全性（Part 7）：progress＋preservation（TAPL 8.3）、正規化（TAPL 12）と Ω の型付け不能、「手放すのは
  主に progress、preservation は untyped widen で自明に保つ」という非対称の整理が正しい。gradual の 2 規律
  （consistency `~` 非推移／gradual guarantee）の区別も正確。

erasure の前編再アンカー（glossary「P1 予告→P3 本式」・a3-2「内部精密型↔RBS 境界保守型」）と後編 Part 3 §3-5・
Part 8 §8-3 の記述は**整合しており、矛盾・二重定義は無い**。

---

## 指摘表

| 該当箇所 | 本書の記述 | 型理論的な問題 | 修正案 | 重大度 |
|---|---|---|---|---|
| Part 5 §5-4 ①（行 160-161） | 「ランク 2 までで境界が引かれ、**ランク 3 以上の多相型推論は決定不能**です（Wells 1994/1999 は無制限 System F の型付け可能性の決定不能性）」 | ランク境界（≤2 決定可能／≥3 決定不能）は正しいが、その根拠は Kfoury & Wells (1999) のランク 3 結果。括弧内の Wells 引用は*無制限* System F に正しくスコープされているため誤りではないが、ランク 3 境界と無制限結果が同一文に同居し、読者がランク 3 境界＝Wells と読み違える余地がある | 「ランク ≥3 は決定不能（Kfoury & Wells 1999）。無制限 System F の型付け可能性そのものの決定不能性は Wells 1994」と分けると正確。最小修正なら現状維持可（事実誤りではない） | nitpick |
| Part 1 §1-2（行 67-77）／§1-5（行 142） | 「`Γ`（ガンマ）は前編の `Scope`」「`Scope`/`FactStore` ＝ 環境 Γ」 | Γ は通常「変数→型」の静的環境で、Part 1 の `Scope` 対応は妥当。一方 Part 6 の `FactStore` はフロー感応な*事実*集合で、純粋な Γ より広い（フロー解析の状態）。「Γ＝FactStore」は近似であって厳密な同一ではない | §1-5 で「FactStore は Γ のフロー感応な拡張（厳密には Γ より広い）」と一言。実際 Part 6 §6-1 では正しく「Γ を一般化」と書いており、Part 1 の同一視と整合させると親切 | nitpick |
| Part 4 §4-5 コラム末（行 190） | リスト/引用のインデント崩れ：「**fuel（既定 64）＋進捗追跡**で安全側に打ち切ります。」の行が引用ブロック `>` の外に出ている | 型理論の問題ではなく整形。直前まで blockquote、当該行だけ `>` 落ち | 行頭に `> ` を補う（内容は正しい。fuel 既定 64 は ADR-20 WD3 と一致） | nitpick |
| Part 2 §2-1（行 40-41） | 「格子の両端 … 前編の `untyped`（後で見る）を除けば、最小の `Bot`…」 | `untyped`/`Dynamic[Top]` を「格子の両端の話から除外」する扱いは a1-4 の「untyped はトップ型ではない（軸 A/B）」と整合的で正しい。ただし本文だけ読むと「untyped も端の候補か」と一瞬誤読し得る | a1 への参照は既にある（行 62・227）ので実害なし。強いて言えば「untyped は格子の*端ではなく*判定を切るマーカー（→a1）」と能動的に否定すると締まる | nitpick |
| Part 7 §7-4 表 ①行（行 86） | 「① `untyped` は何でも受理 ／ progress を放棄（`~` が非推移＝subsumption が漏れる）」 | `~` の非推移性は「`untyped` が無制限の抜け穴になりきらない＝健全側に働く性質」として §7-5・Part 2 で正しく使われている。ここで非推移性を progress 放棄の*理由*側に置くと、同じ性質が「漏れる根拠」と「漏れを抑える根拠」の両方に見え、読者が混乱し得る（事実としては「untyped が `:maybe` を通すこと」が progress 放棄の核） | 「`~` で `untyped` を素通しさせる（`:maybe` を罰しない）」と理由を consistency の*通す*側に寄せる。非推移性は §7-5 の「無秩序でない」側に純化 | MISLEADING |
| Part 6 §6-2 脚注 [^buckets]（行 81・87-89） | 「バケツ名は本物の Rigor の内部仕様（`inference-engine.md`）の正式名と一致します」 | 検証範囲外の主張（spec ファイルの正式名と本文 6 名の一致を本査読では突合していない）。型理論的問題ではないが「一致する」と断言している | spec と語が一致することを著者側で再確認（または「概ね対応」に緩める）。型理論的には無害 | nitpick |

---

## 重点項目ごとの所見（ERROR なしの確認記録）

1. **双方向（Part 1）**：`(Var-Synth)`・`(Sub)` 規則の書式・前提・結論すべて正しい。「照合＝合成してから `<:`」
   の同定が前編「引数を type_of→accepts」と形式的に一致。`check(rbs:)` モードと `check_against`（`:no` のみ診断・
   Dynamic で黙る）は lib/chibirigor/checker.rb の実装と一致。`param:` ディレクティブの「本体ナローイング＋呼び
   出し地点照合」の二役整理も妥当。
2. **部分型と変性（Part 2）**：誤りなし。LSP／robustness／S-Arrow の収束（§2-3a）は「同じ規則に別ルートで至る」
   という主張として正しく、過剰主張になっていない。可変コンテナの読み共変・書き反変・両方不変も標準どおり。
3. **代入・System F（Part 3）**：誤りなし。α 同値の map 技法が Part 4 再帰型と「根が一つ」という橋渡しも妥当。
   有界量化＝TAPL 26 章の参照も正しい。
4. **再帰型（Part 4）**：誤りなし。簡約版 `seen` の弱さの自己申告、HKT 根拠＝TAPL 29 章、fuel＝余帰納の工学的
   代替、`symbolize_names: true` のリテラル型による HKT 引数切替（Const 連動）まで一貫。
5. **HM・型再構築・単一化（Part 5）**：occurs-check 省略の明示、let 多相が 22 章でなく 23/System F の領分という
   切り分け、ランク境界（上表 nitpick 以外は正確）、TypeProf（whole-program 抽象解釈）との対比すべて妥当。
6. **健全性（Part 7）**：progress/preservation/正規化/Ω/gradual 2 規律、いずれも正しい。「再帰型を足すと正規化が
   破れる」も（μ 型で Ω が型付け可能になるため）正しい。`assert:` ディレクティブを「unsound キャストではなく
   確かめた上で事実を足すゲート」と位置づけ、TS のユーザー定義型ガードに寄せた整理も妥当。
7. **HKT/カインド（Part 4・8）**：`App[F,A]` の正しさ根拠を TAPL 29 章（kinding）に置く判断は正確。defunctionalize
   した軽量実装という説明も Rigor ADR-20 と整合。
8. **データフローの join/meet（Part 6 §6-5）**：合流点で「両枝で成り立つ事実だけ残す」＝事実格子の meet を、
   慣用語では「join」と呼ぶ、という格子の向きの取り違えを*正しく*回避している（型格子 join≠事実格子 meet を明記）。

## 参照番号・定理名の確認（REF：誤りなし）

- TAPL：8.3（安全性＝進行＋保存）、9（STLC 型付け規則・T-Var）、12（正規化）、15・16（部分型・メタ理論）、
  20.2（iso/equi）、21（再帰型メタ理論・余帰納）、22（型再構築）、23・23.7（System F・erasure 定理）、
  26（有界量化）、29（型演算子とカインド）— **すべて正しい**。
- 人名・文献：Milner「Well-typed programs cannot go wrong」、Pierce & Turner "Local Type Inference"(2000)、
  Dunfield & Krishnaswami "Bidirectional Typing"(2021)、Siek & Taha (2006) gradual typing、Siek ら gradual
  guarantee、Liskov (LSP)、Wells（System F 型付け決定不能）— **帰属に致命的誤りなし**（Wells と Kfoury&Wells の
  ランク 3 結果の区別だけ上表 nitpick）。
- 『しくみ』章対応（3/7/8/9 章・おわりに）と a4 早見表 — 本文各章の参照と整合。

## Rigor 実装との整合（fidelity 観点の事実確認）

- ADR-5（robustness）/ADR-20（軽量 HKT・fuel 既定 64）/ADR-35（override 互換＝param 反変・戻り共変）/
  ADR-41（Status: Proposed＝未実装）/ADR-47（narrowing-driven-clause-reachability＝a1 の `flow.unreachable-clause`）
  すべて実在・記述と一致。
- acceptance.rb：Nominal 型引数は要素ごと共変（zip→covariant）。Part 2「一律共変・宣言サイト変性未実装」と一致。
- lib/chibirigor：`check_against`・`register_method`・`element_read`・`type_of_block`・`accepts(Dynamic→:maybe)`
  すべて本文記述どおり実在。

以上。形式的に堅牢で、簡略化はいずれも正しく断られている。表中の MISLEADING 1 件（Part 7 §7-4 ① の理由づけ）と
nitpick 群のみ、必要に応じて反映されたい。
