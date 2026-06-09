---
title: 付録 a3 ― 道具（実 Rigor の CLI と dispatch カスケード）
description: 本編が 1 行ポインタで送った実 Rigor のツール挙動（`check --explain`・`type-of`・dispatch 5 段カスケード）を 1 箇所にまとめる参照付録。
sidebar:
  order: 23
---

# 付録 a3 ― 道具（実 Rigor の CLI と dispatch カスケード）

これは**参照付録**です。本書（chibirigor）は「動く最小版」を手で作るのが主目的なので、
本編の各所では実物の **Rigor** のツール挙動を*1 行ポインタ*で送りました。その送り先がここです。
chibirigor の最小版と実 Rigor の差を「**本書では素朴／実物はこう**」の橋渡しで並べます。

> **本編からの戻りポインタ**
>
> - 前編 **Part 2**（メソッド送信とディスパッチ）から：素朴な「表引き」一段に留めた dispatch を、
>   実物は **5 段カスケード**で引きます → 本付録 §a3-3。
> - 前編 **Part 9**（gradual の哲学）から：`rigor check --explain` が `Dynamic[Top]` の
>   *fail-soft* した箇所を一覧にする「地図」の仕組み → 本付録 §a3-1。
> - 前編 **Part 1**（リテラルと算術）のコラム「実 Rigor の `rigor type-of`」から：位置指定で
>   推論型を引くコマンドの詳細 → 本付録 §a3-2。

ここに書く Rigor の事実記述は本編の原稿と一致させています（5 段の順序・名称は原稿どおり）。
本書のコード挙動を変える記述ではありません。

---

## a3-1. `rigor check --explain` ― fail-soft の地図を出す

通常の `rigor check` は、**証明できた問題だけ**を診断として報告し、`Dynamic[Top]`（本文の最小版
`Dynamic` の内部表記＝`untyped`。付録 a1-1 参照）に *fail-soft* した箇所については黙っています（前編 Part 2「知らなければ黙る」、Part 9「わざと
見逃す」の実物）。これは誤検知を出さないための態度ですが、裏を返せば「**静かに見逃している**」
でもあります。

`--explain` を付けると、その **fail-soft した全箇所が `:info` 診断として現れます** ―
「ここで型を見失いました」という地図が出力されます。

```console
$ rigor check --explain app/models/user.rb
app/models/user.rb:42:7: info: fell soft to Dynamic[Top] here (RBS not found for `external_call`)
app/models/user.rb:51:3: info: fell soft to Dynamic[Top] here (param `opts` is untyped)
```

使い道はこうです：

- 「このバグを見落としているのでは？」という疑問が出たとき、`--explain` の出力で
  「**どこで型が消えたか**」を遡る。
- たどり着いた fail-soft 地点から、**RBS の不足・プラグイン未設定・型注釈の抜け漏れ**を発見できる。

### なぜ「地図」が描けるのか ― `Dynamic[Top]` マーカーの回収

この一覧が成り立つのは、前編 Part 1 で触れた **`Dynamic[Top]` の `Dynamic` マーカー**が、
fail-soft した箇所に**消えずに残っている**からです。`untyped` を「ただの穴」ではなく
「`Dynamic` という印の付いた `Top`」として持つことで、「どこで黙ったか」が**構造として残ります**。
だからこそ、後から `--explain` で一覧に起こせます。

| | 本書（chibirigor） | 実物（Rigor） |
|---|---|---|
| `untyped` の正体 | `Type::Dynamic`（印だけ） | `Dynamic[Top]`（`Top` に `Dynamic` マーカー） |
| fail-soft 地点 | 黙って `Dynamic` を返すだけ | 地点を構造に保持し、`--explain` で一覧化 |
| 沈黙の可視化 | 仕組みなし（最小版の対象外） | `check --explain` が `:info` 診断で地図を出す |

chibirigor の「知らなければ黙る」は誤検知を防ぎますが、`--explain` は**その沈黙そのものを
可視化する道具**です。

---

## a3-2. `rigor type-of file:line:col` ― 位置指定で推論型を引く

実 Rigor には、ソースの**特定の位置の式**について推論型を引くコマンドがあります：

```console
$ rigor type-of app/models/user.rb:10:5
```

`file:line:col`（ファイル・行・列）で式を 1 つ指して、その**推論型を表示**します。特徴は、
**2 種類の型を並べて見せる**ことです：

1. Rigor 内部での**精密な型**（例：`Constant<"FOO">`）。
2. RBS の境界で**粗い型に落とした**後の保守的な型（例：`String`）。

この 2 段構えが、本書の `annotate` との違いを説明します。chibirigor の `annotate` は内部型だけを
見せますが、実ツールには「**内部では精密に知っているが、境界では捨てる**」という二重構造が
あります。「なぜ `annotate` の出力とシグネチャが食い違うのか」を調べるときに `type-of` を使います。

> 精密な内部型を RBS で表せる粗い型に落とす境界の操作を、Rigor は **erasure** と呼びます
> （Java ジェネリクスの実行時「型消去」とは別物 ― あちらは実行時、こちらは静的な精度を境界で
> 丸める話）。仕組みは後編で扱います。

| | 本書（chibirigor） | 実物（Rigor） |
|---|---|---|
| 型を引く粒度 | 文ごと（`annotate` が行単位で型を並べる） | `file:line:col` で式 1 つをピンポイント指定 |
| 見せる型 | 内部型 1 つ | 内部の精密な型 ＋ 境界で丸めた保守的な型の 2 つ |
| 用途 | 推論結果の確認 | 内部型と RBS 境界型の**食い違いの調査** |

---

## a3-3. dispatch 5 段カスケード ― 表引きの実物

前編 Part 2 は、メソッド送信の型付けを**素朴な表引き一段**に留めました（`(クラス, メソッド)`
で `METHODS` 表を引き、見つかれば戻り型、見つからなければ `untyped`）。実物の Rigor は、この
「表引き」を **5 段のカスケード**にしています。**上の段から順に当て、その段が当てられなければ
次の段へ落ちます**（fall through）。各段が何を解決し、外れたら何に渡すか：

| 段 | 名前 | 何を当てるか | 外れたら |
|---|---|---|---|
| ① | **定数畳み込み** | `1 + 2` のように両辺が既知の定数なら、その場で**実際に計算**して結果の型（`3`）を出す | ② へ |
| ② | **shape dispatch** | `HashShape` のキー読み出しなど、**型の構造に直接触れる**操作を構造から直接解く | ③ へ |
| ③ | **RBS** | コア・stdlib・プラグインが提供する **RBS の型**で引く（本書の手書き `METHODS` 表の実物） | ④ へ |
| ④ | **in-source**（本体推論） | RBS に無いメソッドは、**本体を推論**して戻り型を合成する（前編 Part 8 の戻り型合成の実物） | ⑤ へ |
| ⑤ | **fallback** | どの段でも当たらなければ **`Dynamic[Top]`** に degrade（脅かさない） | ―（ここで止まる） |

### 流れの読み方

ひとつの呼び出しは、上から順に「この段で解けるか？」を問われ、解けた段で打ち止めになります。
解けない段は黙って次に渡すだけ ― **誤検知を出さず、最後は必ず `Dynamic[Top]` で受ける**ので、
未知の呼び出しでも止まりません（前編 Part 2「知らないメソッドは脅かさない」の実物）。

```text
  receiver.method(args)
    │
    ▼
  ① 定数畳み込み ── 当たる ─→ 結果の型（例: 3）
    │ 外れ
    ▼
  ② shape dispatch ─ 当たる ─→ 構造から直接解いた型
    │ 外れ
    ▼
  ③ RBS ────────── 当たる ─→ RBS 由来の戻り型
    │ 外れ
    ▼
  ④ in-source ──── 当たる ─→ 本体推論で合成した戻り型
    │ 外れ
    ▼
  ⑤ fallback ───────────────→ Dynamic[Top]（untyped）
```

### 優先順位が効く例 ― なぜ ③ が ④ に勝つか

カスケードは**順序そのものが意味を持ちます**。たとえば `RBS::Extended` ディレクティブで
宣言した型が、メソッド本体の推論に**勝つ**のは、③ RBS が ④ in-source より**先に当たる**からです。
「宣言を本体に優先する」という設計判断が、段の並び順として表れています。

| | 本書（chibirigor） | 実物（Rigor） |
|---|---|---|
| dispatch の段数 | 1 段（`METHODS` 表を引くだけ） | 5 段（① 定数畳み込み → ② shape → ③ RBS → ④ in-source → ⑤ fallback） |
| 表の中身 | 手書きの `METHODS` Hash | ③ が RBS（コア・stdlib・プラグイン由来） |
| 本体推論 | `annotate` で別途（前編 Part 8） | ④ in-source として dispatch に組み込み |
| 未知の扱い | `Dynamic` を返す | ⑤ fallback で `Dynamic[Top]` |
| 宣言 vs 推論の優先 | （区別なし） | 段の順序（③ が ④ より先）で表現 |

> 本書の Part 2 が dispatch を 1 段に留めたのは、③ RBS（前編 Part 8 まで未習）や ④ in-source を
> 未習のまま列挙すると話が浮くからです。5 段の全貌は Part 8 まで読み終えた読者が、ここで
> 一望できるように切り出しました。

---

## a3-4. まとめ ― 「素朴／実物」対応の早見

本付録で橋渡しした 3 つの道具を一枚に：

| 道具 | 本書での扱い | 実物の挙動 | 戻りポインタ |
|---|---|---|---|
| `rigor check --explain` | 仕組みなし（黙るだけ） | `Dynamic[Top]` マーカーを手がかりに fail-soft 地点を `:info` で地図化 | 前編 Part 9 |
| `rigor type-of file:line:col` | `annotate`（内部型のみ・行単位） | 位置指定で内部の精密型 ＋ 境界の保守型を 2 つ並べる | 前編 Part 1 |
| dispatch 5 段カスケード | 1 段の表引き（`METHODS`） | ① 定数畳み込み → ② shape → ③ RBS → ④ in-source → ⑤ fallback | 前編 Part 2 |

いずれも、本書で手作りした骨格（`Dynamic` マーカー・`annotate`・`METHODS` 表）が、実 Rigor では
**同じ骨格を拡大した形**で動いている、という対応で読めます。
