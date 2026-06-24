---
title: 付録 a3　道具（実 Rigor の CLI と dispatch カスケード）
description: 本編が 1 行ポインタで送った実 Rigor のツール挙動（`check --explain`、型表示の二段構え／erasure、`trace`、dispatch 5 段カスケード）を 1 箇所にまとめる参照付録。
sidebar:
  order: 23
---

# 付録 a3　道具（実RigorのCLIとdispatchカスケード）

これは**参照付録**です。本書（chibirigor）は「動く最小版」を手で作るのが主目的なので、本編の各所では実物のRigorのツール挙動を1行ポインタで送りました。その送り先がここです。chibirigorの最小版と実Rigorの差を「本書では素朴／実物はこう」という橋渡しで並べます。

> [!NOTE]
> **本編からの戻りポインタ**
>
> - 前編**Part 2**（メソッド送信とディスパッチ）から：素朴な「表引き」一段に留めたdispatchを、実物は**5段カスケード**で引きます。本付録 §a3-3を参照してください
> - 前編**Part 9**（gradualの哲学）から：`rigor check --explain`が`Dynamic[Top]`のfail-softした箇所を一覧にする「地図」の仕組みについては本付録 §a3-1を参照してください
> - 前編**Part 1**（リテラルと算術）のコラムから：実Rigorが内部の精密型とRBS境界の保守型を二段で持つ仕組み（erasure）については本付録 §a3-2を参照してください

ここに書くRigorの事実記述は本編の原稿と一致させています（5段の順序、名称は原稿どおりです）。本書のコード挙動を変える記述ではありません。

---

## a3-1. `rigor check --explain`（fail-softの地図を出す）

通常の`rigor check`は、**証明できた問題だけ**を診断として報告し、`Dynamic[Top]`（本文の最小版`Dynamic`の内部表記＝`untyped`。付録[a1-1](a1-special-types.md)参照）にfail-softした箇所については黙っています（前編Part 2「知らなければ黙る」、Part 9「わざと見逃す」の実物です）。これは誤検知を出さないための態度ですが、裏を返せば「静かに見逃している」でもあります。

`--explain`を付けると、その**fail-softした全箇所が`:info`診断として現れます**。「ここで型を見失いました」という地図が出力されます。

```console
$ rigor check --explain app/models/user.rb
app/models/user.rb:42:7: info: fell soft to Dynamic[Top] here (RBS not found for `external_call`)
app/models/user.rb:51:3: info: fell soft to Dynamic[Top] here (param `opts` is untyped)
```

使い道はこうです：

- 「このバグを見落としているのでは？」という疑問が出たとき、`--explain`の出力で「どこで型が消えたか」を遡る。
- たどり着いたfail-soft地点から、RBSの不足、プラグイン未設定、型注釈の抜け漏れを発見できる。

### なぜ「地図」が描けるのか（`Dynamic[Top]`マーカーの回収）

この一覧が成り立つのは、前編Part 1で触れた**`Dynamic[Top]`の`Dynamic`マーカー**が、fail-softした箇所に**消えずに残っている**からです。`untyped`を「ただの穴」ではなく「`Dynamic`という印の付いた`Top`」として持つことで、「どこで黙ったか」が**構造として残ります**。だからこそ、後から`--explain`で一覧に起こせます。

| | 本書（chibirigor） | 実物（Rigor） |
|---|---|---|
| `untyped`の正体 | `Type::Dynamic`（印だけ） | `Dynamic[Top]`（`Top`に`Dynamic`マーカー） |
| fail-soft地点 | 黙って`Dynamic`を返すだけ | 地点を構造に保持し、`--explain`で一覧化 |
| 沈黙の可視化 | 仕組みなし（最小版の対象外） | `check --explain`が`:info`診断で地図を出す |

chibirigorの「知らなければ黙る」は誤検知を防ぎますが、`--explain`は**その沈黙そのものを可視化する道具**です。

### a3-1x. 発展：chibirigorにも極小`--explain`がある

上の表は本書を「沈黙の可視化＝仕組みなし」としていますが、ここにも**極小版**を足しました。`check --explain`を付けると、推論が`untyped`に倒した地点（未知メソッドのディスパッチ）を`:info`診断として併せて出します：

```console
$ printf 'x = mystery_call\ny = x + 2\n' > demo.rb
$ ruby exe/chibirigor check --explain demo.rb
demo.rb:1:5: info: fell to untyped here (can't look up the type of `mystery_call`)
  x = mystery_call
      ^^^^^^^^^^^^
demo.rb:2:5: info: fell to untyped here (can't look up the type of `+`)
  y = x + 2
      ^^^^^
```

注目したいのは**2行目**です。`mystery_call`の型がわからず`x`が`untyped`になり、その`x`に対する`+`も型を引けずに`untyped`へ倒れています。**沈黙が伝播していく**様子が地図に出ます。`--explain`無しなら（誤検知を出さないので）`No type errors`と黙るだけです。`:info`は終了コードを汚さない（`exit 0`）ので、CIを止めずに「どこで型が消えたか」だけ覗けます。

実物との差は、実Rigorが`Dynamic[Top]`の`Dynamic`マーカーを構造に保持してあらゆるfail-soft（RBS不足、未注釈引数、プラグイン未設定など）を一覧化するのに対し、chibirigorが拾うのは**未知ディスパッチ1種**だけ、という点です（実装は`lib/chibirigor/dispatch.rb`の`signature`がnilの枝でprovenanceを1行積むだけです）。沈黙を**地図にする**という発想は同じです。

---

## a3-2. Rigorの型表示（内部の精密型とRBS境界の保守型）

本書の`annotate`は、推論した型を1つそのまま見せます。実Rigorも`annotate`（本書と同じ名前の道具です）やエディタのhoverで型を見せますが、実物には本書に無い**二段構え**があります：

1. Rigor内部での**精密な型**（例：`Constant<"FOO">`）。
2. RBSの境界で**粗い型に落とした**後の保守的な型（例：`String`）。

chibirigorの`annotate`は内部型だけを見せます。実物は「**内部では精密に知っているが、境界では捨てる**」という二重構造を持つので、「推論はもっと知っていそうなのに、シグネチャ（RBS）はなぜこんなに粗いのか」という状況が起こります。その答えが、この二段構えです。

> [!NOTE]
> 精密な内部型をRBSで表せる粗い型に落とす境界の操作を、Rigorは**erasure**と呼びます（Javaジェネリクスの実行時「型消去」とは別物です。あちらは実行時の話で、こちらは静的な精度を境界で丸める話です）。仕組みは後編で扱います。

| | 本書（chibirigor） | 実物（Rigor） |
|---|---|---|
| 型の見せ方 | `annotate`が行単位で内部型を並べる | `annotate`／hoverで見せ、境界ではerasureで丸める |
| 見せる型 | 内部型1つ | 内部の精密な型、境界で丸めた保守的な型の二段 |
| ズレの正体 | （境界が無いので起きない） | 内部精密型とRBS境界型の食い違い（erasure） |

> [!NOTE]
> chibirigor側は道具を`check`と`annotate`の2つに絞っています。推論型を見たいときは`annotate`（行単位）で足ります。「内部では精密、境界では粗く」という二段構えとerasureは、本物のRigorに進んだときに出会う景色です。

> [!NOTE]
> **実Rigorで「ある式」の型だけをピンポイントに見たいとき**は、`annotate`（ファイル全体）のほかに、ソースに`dump_type(式)`と書いて`check`する手があります（裸の`dump_type(x)`には`include Rigor::Testing`が要りますが、`Rigor.dump_type(x)`と完全修飾すればincludeは不要です）。`dump_type`は実行時には値をそのまま返すだけです。`check`したときに**その位置の推論型を`:info`診断として印字**します（PHPStanの`dumpType()`と同じ発想で、§a3-1の`--explain`と同じ`:info`の仕組みに乗ります。エラーではないので散りばめても診断は赤くなりません）。エディタのhoverが見せる型も同じ推論ですが、そちらは**ツール向けの低レベルAPI**が裏で支えていて、人が直接叩くコマンドではありません。

そして**chibirigorも`dump_type`を基本機能として持っています**。道具は`check`と`annotate`の2つに絞ったままです。これはコマンドではなく、`check`が認識する**式**です。`include`も要らず、`dump_type(式)`と書いて`check`するだけです：

```console
$ printf 'x = c ? 1 : "a"\ndump_type(x)\n' > demo.rb
$ ruby exe/chibirigor check demo.rb
demo.rb:2:1: info: dump_type: 1 | "a"
  dump_type(x)
  ^^^^^^^^^^^^
```

`:info`なので**診断は赤くならず**（終了コードは0）、本物の型エラーと一緒に出しても邪魔をしません。仕組みは前編Part 9で作った`:info`診断そのままです。`type_of`が`dump_type(式)`を見つけたら、引数の推論型を`:info`診断に記録し（`check`がそれを表に出します）、値はそのまま返します（だから`dump_type(x)`の型は`x`の型のままです）。実装は`lib/chibirigor/type_of.rb`、挙動の網羅は`test/test_dump_type.rb`です。

---

## a3-3. dispatch 5段カスケード（表引きの実物）

前編Part 2は、メソッド送信の型付けを**素朴な表引き一段**に留めました（`(クラス, メソッド)`で`METHODS`表を引き、見つかれば戻り型、見つからなければ`untyped`）。実物のRigorは、この「表引き」を**5段のカスケード**にしています。**上の段から順に当て、その段が当てられなければ次の段へ落ちます**（fall through）。各段が何を解決し、外れたら何に渡すかを示します：

| 段 | 名前 | 何を当てるか | 外れたら |
|---|---|---|---|
| ① | **定数畳み込み** | `1 + 2`のように両辺が既知の定数なら、その場で**実際に計算**して結果の型（`3`）を出す | ② へ |
| ② | **shape dispatch** | `HashShape`のキー読み出しなど、**型の構造に直接触れる**操作を構造から直接解く | ③ へ |
| ③ | **RBS** | コア、stdlib、プラグインが提供する**RBSの型**で引く（本書の手書き`METHODS`表の実物） | ④ へ |
| ④ | **in-source**（本体推論） | RBSに無いメソッドは、**本体を推論**して戻り型を合成する（前編Part 8の戻り型合成の実物） | ⑤ へ |
| ⑤ | **fallback** | どの段でも当たらなければ**`Dynamic[Top]`**にdegrade（脅かさない） | （ここで止まる） |

### 流れの読み方

ひとつの呼び出しは、上から順に「この段で解けるか？」を問われ、解けた段で打ち止めになります。解けない段は黙って次に渡すだけです。**誤検知を出さず、最後は必ず`Dynamic[Top]`で受ける**ので、未知の呼び出しでも止まりません（前編Part 2「知らないメソッドは脅かさない」の実物です）。

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

### 優先順位が効く例（なぜ ③ が ④ に勝つか）

カスケードは**順序そのものが意味を持ちます**。たとえば`RBS::Extended`ディレクティブで宣言した型が、メソッド本体の推論に**勝つ**のは、③ RBSが ④ in-sourceより**先に当たる**からです。「宣言を本体に優先する」という設計判断が、段の並び順として表れています。

| | 本書（chibirigor） | 実物（Rigor） |
|---|---|---|
| dispatchの段数 | 1段（`METHODS`表を引くだけ） | 5段（① 定数畳み込み、② shape、③ RBS、④ in-source、⑤ fallback） |
| 表の中身 | 手書きの`METHODS` Hash | ③ がRBS（コア、stdlib、プラグイン由来） |
| 本体推論 | `annotate`で別途（前編Part 8） | ④ in-sourceとしてdispatchに組み込み |
| 未知の扱い | `Dynamic`を返す | ⑤ fallbackで`Dynamic[Top]` |
| 宣言vs推論の優先 | （区別なし） | 段の順序（③ が ④ より先）で表現 |

> [!NOTE]
> 本書のPart 2がdispatchを1段に留めたのは、③ RBS（前編Part 8まで未習）や ④ in-sourceを未習のまま列挙すると話が浮くからです。5段の全貌はPart 8まで読み終えた読者が、ここで一望できるように切り出しました。

---

## a3-3b. `rigor trace`（推論の手順をコマ送りで見る）

ここまでの道具（`--explain`、型表示の二段構え、カスケード）は、推論の**答え**や**地図**を見せるものでした。実Rigorにはもう一つ、推論の**手順そのもの**を見せる道具があります。`rigor trace`です。`check`が走らせるのと同じ推論をファイルに対して再走させ、記録した推論イベントを**端末のコマ送りアニメーション**として再生します。1コマ＝推論の1場面で、ローカルがscopeに入る瞬間（`bind`）、分岐の型が1つのunionに溶ける瞬間（`union`）、メソッド送信が解決する（または`Dynamic[top]`にfail-softする）瞬間（`dispatch`）を、評価される範囲をハイライトしながら見せます。

```sh
rigor trace lib/example.rb              # キー押しでコマ送り
rigor trace --delay=0.5 lib/example.rb  # 自動再生
rigor trace --format=json lib/example.rb # 生のイベント列
```

`--verbose`は式ごとのenter/resultまで全部出し、既定では上の3種の「教えどころ」だけに絞ります。JSONのイベント列は安定しているので、教材の図や講義資料の素材にできます。

### a3-3bx. 発展：chibirigorにも極小`trace`がある

これは付録の中でも珍しく、**本書側にも実物とほぼ同じ道具がある**節です（実装は`lib/chibirigor/tracer.rb`）。本書で各Partを写経して作った部品（scopeへの束縛、`Type.union`の畳み込み、dispatchの表引き）が、**動く順番**で目の前を流れていきます。読者が「評価順はこうだろう」「ここでunionになるはず」と頭の中で組み立てた推論を、**目で確かめる**ための学習用の道具です。

3行の例で動かしてみます。代入、三項演算子、メソッド呼び出しを1つずつ含みます：

```console
$ printf 'x = 5\ny = x > 0 ? 1 : -1\nz = y + 2\n' > demo.rb
$ ruby exe/chibirigor trace demo.rb
```

端末では1コマずつEnterで送ります（`q`で終了）。全17コマのうち、要点のコマだけ抜き出すと：

```text
chibirigor trace ─ step 2/17
────────────────────────────────────────────────────────────────
  1  x = 5
  2  y = x > 0 ? 1 : -1
  3  z = y + 2
────────────────────────────────────────────────────────────────
type env   : x: 5
evaluating : (top level)
► bind: x ← 5 (added to type env)
…
chibirigor trace ─ step 5/17
…
type env   : x: 5
evaluating : if (incl. ternary) › call to >
► dispatch: 5.>(0) → untyped (not in table → fail-soft to untyped)
…
chibirigor trace ─ step 7/17
…
evaluating : if (incl. ternary)
► union: 1 , -1 → 1 | -1
…
chibirigor trace ─ step 9/17
…
type env   : x: 5   y: 1 | -1
evaluating : (top level)
► bind: y ← 1 | -1 (added to type env)
…
chibirigor trace ─ step 12/17
…
evaluating : call to +
► dispatch: 1.+(2) → 3 (constant folding)
chibirigor trace ─ step 13/17
…
► dispatch: -1.+(2) → 1 (constant folding)
chibirigor trace ─ step 14/17
…
► union: 3 , 1 → 3 | 1
chibirigor trace ─ step 15/17
…
► dispatch: 1 | -1.+(2) → 3 | 1 (distribute Union to members)
…
chibirigor trace ─ step 17/17
…
type env   : x: 5   y: 1 | -1   z: 3 | 1
evaluating : (top level)
► bind: z ← 3 | 1 (added to type env)

── playback done (17 steps total) ──
```

この17コマで、本編の部品が動く順に並びます。`x`の束縛（step 2）から、三項の条件`x > 0`が**表に無い`>`**なのでfail-softで`untyped`に倒れ（step 5）、2本のアーム`1`と`-1`が**union**に溶けて`y`に束縛され（step 7、9）、`y + 2`は`y`が`1 | -1`なので**メンバへ分配**して各メンバを**定数畳み込み**し（step 12、13）、結果をまたunionでまとめて（step 14、15）`z`を`3 | 1`に束縛する（step 17）という流れです。各Partで別々に作った仕組みが、1つの式で**どう連動するか**が一望できます。`--verbose`を付ければ、間引いた式ごとのenter/resultも全部出ます。JSONで出したいときは`--json`、自動再生は`--delay 0.5`です。

仕組みは実物と同じ発想で、**コアには一切手を入れていません**。`type_of`、`eval_statement`、`Type.union`、`Dispatch.dispatch`に`Module#prepend`でフックを差し込むだけで、レコーダが`nil`（トレース中でない）ときフックは即`super`します。だから`check`や`annotate`の挙動は変わらず、**本編で写経したコードを1行も汚しません**（`tracer.rb`冒頭コメント参照）。

| | 本書（chibirigor） | 実物（Rigor） |
|---|---|---|
| 見せるもの | 推論イベントのコマ送り（`bind`、`union`、`dispatch`） | 同じ（推論のderivationを再生） |
| コアへの干渉 | `Module#prepend`のフック、レコーダnilなら即super | `check`と同じ推論に乗る記録から再生する探針 |
| 出力形式 | 端末アニメーション、`--json`、`--verbose`、`--delay` | 端末アニメーション、`--format=json`、`--verbose`、`--delay`、`--line` |

実物が`--line=N`で1行だけに絞れるのに対し、chibirigorは行フィルタを持たない、といった枝葉の差はありますが、**推論の手順を再生して見せる**という発想と3種のイベントは同じです。本書の他の道具が「Rigorの実物から極小版」だったのに対し、`trace`は珍しく**本書側がほぼ実物と同型**で並びます。

---

## a3-4. まとめ（「素朴／実物」対応の早見）

本付録で橋渡しした4つを一枚に：

| 仕組み | 本書での扱い | 実物の挙動 | 戻りポインタ |
|---|---|---|---|
| `rigor check --explain` | 極小版あり（未知ディスパッチを`:info`地図化、§a3-1x） | `Dynamic[Top]`マーカーを手がかりにfail-soft地点を`:info`で地図化 | 前編Part 9 |
| 型表示の二段構え（erasure） | `annotate`は内部型1つ（境界が無いのでズレない、§a3-2） | `annotate`／hoverの裏で内部精密型とRBS境界型をerasureで丸める | 前編Part 1 |
| `rigor trace file` | ほぼ実物と同型の極小版あり（`bind`、`union`、`dispatch`をコマ送り、§a3-3bx） | 推論の手順を端末アニメーションで再生（`--verbose`、`--line`、`--format=json`） | （なし） |
| dispatch 5段カスケード | 1段の表引き（`METHODS`、極小版は設けず） | ① 定数畳み込み、② shape、③ RBS、④ in-source、⑤ fallback | 前編Part 2 |

いずれも、本書で手作りした骨格（`Dynamic`マーカー、`annotate`、`METHODS`表）が、実Rigorでは**同じ骨格を拡大した形**で動いている、という対応で読めます。
