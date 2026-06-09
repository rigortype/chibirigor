# L1 真・型理論エキスパート 査読（2026-06-09）

対象：前編 `book/v1/ja/little/part0–part9`、付録 `a1–a5`、`glossary.md`、`README.md`。
観点：形式的・技術的正確さ（gradual／双方向／変性／再帰型／HM／System F／健全性／HKT）。
重大度：ERROR（型理論として誤り）／MISLEADING（条件付きでしか正しくない）／REF（参照番号・出典）／nitpick。
各項に **〔事故的不正確さ〕**（無条件断言で理論的に誤り）／**〔意図的簡略化＝許容〕** を明記。

実装裏取り：`lib/chibirigor/{type,narrowing,dispatch,accepts,type_of}.rb` を実行確認。
Rigor 一次情報：`rigor/docs/adr/1-types.md`・`adr/3-type-representation.md`・
`type-specification/special-types.md` で `void`／`untyped`＝`Dynamic[Top]` の記述を裏取り。

---

## 総評（先に結論）

**重点 5 箇所（Part1 コラム／Part8 `void` BC-break／Part5 null 安全・`remove_nil`／Part4 無タグ
Union vs タグ付き variant／付録 a5・a1）は、型理論として*いずれも正しい*。** 事故的な ERROR は
発見できなかった。本文⇔実装の不整合も無し（`remove_nil`・`narrow`・`accepts` の三値・`union` の
untyped 伝播は、本文の主張どおりに実装が振る舞う ― 実行確認済み）。TAPL／『しくみ』の参照番号も
標準目次と一致（前回 L1 査読の結論を再確認）。

指摘はすべて **MISLEADING 1 件＋ nitpick 数件** にとどまる ― いずれも「無条件に断言すると一段
強すぎる」種類で、軸（やさしい前編・gradual・脅かさない）を壊す要求ではない。最も実のあるのは
**a1-2／a1-5 の `void`＝「トップ型の別名・トップそのもの」** が、Rigor／RBS 自身の表現
（"top-**like** but context-limited"）よりわずかに強い点（下記 #1）。

---

## 重点 5 箇所の判定（要請に応じ個別に）

| 重点 | 本文の主張 | 判定 | 裏取り |
|---|---|---|---|
| Part1 コラム「2 流派」 | `unknown`＝絞れ（慎重派）／`untyped`・`any`＝黙る（寛容派）。`untyped`≠トップ型 | **正しい** | a1-1 の軸 A（格子位置）／軸 B（チェック有無）分解は標準的かつ正確。`any` が「てっぺんと底に同時にいる」は TS `any` の通説的記述として妥当 |
| Part8 `void` BC-break | `-> nil` は「nil を返す」を縛る／`void` は縛らない（実装が戻り値を変えても BC-break にならない） | **正しい** | Rigor `adr/1-types.md` L519「a `void` return contract … not to **propagate** a more precise inferred return」と一致。返り共変の観点でも `-> nil` 宣言は本体を縛る |
| Part5 null 安全・`remove_nil`／`narrow` | nil は型で防げるバグ＝null 安全。`remove_nil` は `Const[nil]`/`Nominal[:NilClass]` の両方を剥がす | **正しい・実装一致** | `Dispatch.class_of(Const[nil]) == :NilClass` を実行確認。本文の明示的 reject と実装の `nil_type?`(class_of 経由) は*挙動同値*。`narrow` の FP 安全（偽枝は触らない・`possible?` ガード）も本文どおり |
| Part4 無タグ Union vs タグ付き variant | 『しくみ』/TAPL が持つのはタグ付き variant、我々のは無タグ Union で別物 | **正しい** | TAPL §11.10 は Variants（タグ付き）。無タグ合併は両書が直接扱わない。a4-2 の「11 章 §11.10／両書はタグ付きのみ」も整合 |
| 付録 a5・a1 | 名前的/構造的部分型・null 安全・HashShape 系譜（Hack→PHPStan→Rigor）・`untyped`=`Dynamic[Top]`・`void`=⊤・`never`=`Bot`・格子 | **おおむね正しい**（`void`＝⊤の*別名*だけ #1 で留保） | `untyped`=`Dynamic[Top]` は `adr/3` L174「`Untyped` resolves to `Dynamic[Top]`」と一致。`Bot <: T <: Top`・双対表・`Bot <: T` 正確 |

---

## 所見表

| # | severity | ラベル | 箇所 | 原文引用 | 何が（どう条件付きで）誤りか | 修正案 |
|---|---|---|---|---|---|---|
| 1 | **MISLEADING** | 事故的不正確さ（軽） | a1-2 §「⊤（トップ型）の別名」L81–83／a1-5 表 L198 | 「`void` は**トップ型（⊤）の別名**として扱われます」「`void` は格子上は**トップそのもの**」 | RBS／Rigor 自身は `void` を **"top-**like** but context-limited"**（`adr/1-types.md` L469・`special-types.md` L66–「`void` is **not** an ordinary value type … a result marker」）と位置づける。「別名（alias）／トップそのもの」と無条件に書くと、(a) `void` が値位置で素通りするトップ値であるかのように読め、(b) 同じ a1 が ADR を引いて「値位置に来たら *recover して `top`*・診断を出す」とする扱い（＝別名ではなく*マーカー*）と一段ズレる。本文自身 a1-5 で「`void` は格子に**トップとして自立する**／`untyped` は自立しない」と対比しており、その「自立する＝別名」断定がやや強い | 「格子上の**ふるまいはトップ型と同じ**（top-like）。ただし RBS では戻り位置に限られた*マーカー*で、値位置に出ると `top` に戻して診断する」程度に和らげる。「別名」→「トップ型扱い（top-like）」、「トップそのもの」→「格子上はトップと同じふるまい」。意図（込めたメッセージが違う）の対比はそのまま活かせる |
| 2 | nitpick | 意図的簡略化＝許容（確認のみ） | a1-1 L39–41 | 「TypeScript の `any` は…格子の**てっぺんと底に同時にいる**ようにふるまう ― 部分型関係としては矛盾」 | TS `any` を「⊤かつ⊥」と描くのは*健全な*部分型格子に無理に載せたときの通説的説明で、厳密には `any` は consistency（gradual の `~`）の対象であって `<:` 格子の住人ではない。本文は「**ようにふるまう**」「部分型関係としては矛盾」と*効果*として正しく逃がしており、honest | 修正不要。あえて言えば「`<:` で見ると矛盾＝だから `<:` でなく gradual の整合 `~` で扱う」と一言添えると完璧だが、前編には過剰 |
| 3 | nitpick | 意図的簡略化＝許容 | Part7 §7-3a ① L147 | 「Union のメンバ全員が通って初めて `:yes`（一番弱い結論を採る＝union-subtyping）」 | `actual` が Union のとき全メンバ要求は正しい（`A∨B <: T ⇔ A<:T ∧ B<:T`）。一方 `expected` が Union の「どれか 1 つで `:yes`」は、この素朴な型系では正しいが、一般には `T <: A∨B` の**十分条件**であって必要条件ではない（分配が要る場合がある）。前編の素朴部分型では問題にならない | 修正不要。後編で `expected`-Union 側の不完全性に触れる前提なら現状でよい |
| 4 | nitpick | 意図的簡略化＝許容 | glossary「丸め／正規化」L18 | 「丸め／正規化（normalization）… `Const[3]` を `Integer` に戻すこと。**TAPL 12 章**。」 | TAPL 12 章「Normalization」は STLC の**項の正規化（強正規化定理）**であって、型の精度を粗くする「丸め（widening）」とは別概念。本書の「丸め」はむしろ widening／abstraction に近い。ただし用語集は「もう一段覗きたい人への道しるべ」であり、`normalization` という語の出典として 12 章を指す体裁。誤読リスクは小 | 任意。「丸め」の理論的な隣人は widening（抽象解釈）寄りなので、参照を外すか「※TAPL 12 章の項正規化とは別概念（こちらは型の粗化）」と一言。必須ではない |
| 5 | nitpick（確認のみ・問題なし） | ― | Part5 §5-2 `remove_nil` L84 コメント／a1-3 L121–126 | 本文「nil は…`Const[nil]` か `Nominal[:NilClass]` で来る。両方剥がす」／a1-3「Integer かつ String ＝空集合＝ボトム」 | 実装は `class_of`==`:NilClass` で一括判定するが、`class_of(Const[nil])==:NilClass` を実行確認 ―**挙動同値**。a1-3 の `Integer ∩ String = ∅ = Bot` も正確で、かつ「chibirigor は Bot を型として作らず診断で扱う」と正しく自己申告 | 修正不要。模範的 |

---

## 個別の追加メモ（ERROR/MISLEADING 無しの確認結果）

- **Part0/Part9 「健全性より誤検知回避」**：soundness=progress+preservation（TAPL 8 章 §8.3）、
  gradual はその先、という位置づけは正確。「sound でない＝わざと見逃す 4 箇所」（Part9）の列挙も
  型理論的に筋が通る。
- **Part6 レコード部分型の向き**：「キーが多い方が部分型（`{name:,age:} <: {name:}`）」は width
  subtyping の正しい向き。『しくみ』が完全一致（closed）で締めたのと*逆*（open）にする、という
  対比も妥当。
- **Part7 Postel/robustness（返り厳・引数寛）**：返り共変・引数反変の置換可能性へ落ちる、という
  脚注の予告は正しい方向。
- **a1-4 格子・双対**：`Bot <: T <: Top`、双対表、`untyped`≠トップ型（軸 B を重ねた別物）の注意
  書き ― すべて正確。
- **a5-2 名前的/構造的部分型**：継承＝名前的、構造（キー一致）＝構造的、Go/TS が構造的 ― 正確。
- **a5-5 missing arm vs unreachable arm**：網羅性（missing）と到達不能（unreachable）の方向の違いの
  整理は正確で、健全性 vs 誤検知回避の軸に正しく結びつけている。
- **glossary `Difference` 型**：`non-empty-string` = `String - ""`（集合差）は集合論的に正しく、
  Rigor の二層構成（`Difference`／`Refined`／`IntegerRange`、ADR-3）の自己申告も誠実。

## 参照番号（TAPL／『しくみ』）― 合格

前編で参照する TAPL 章：1・8 §8.3・9・§11.5・§11.7・§11.8・§11.10・12・15・22・23 ―
標準目次と一致。§11.10＝Variants（タグ付き）で Part4 の「タグ付きのみ」主張と整合。誤りなし。

## 本文⇔実装の整合 ― 合格

実行確認：`accepts(Integer|String, Integer)=:yes`／`accepts(Integer, 1|2)=:yes`／
`accepts(Integer, 1|"a")=:no`／`remove_nil(Const[nil]|Integer)=Integer`／
`class_of(Const[nil])=:NilClass`。すべて本文の記述どおり。`union` の untyped 伝播（Part9）・
`possible?` ガード（Part5）も実装と一致。
