# The Seasoned chibirigor（後編・日本語版・ドラフト）

`chibirigor` チュートリアルの**後編**。前編 [The Little chibirigor](../../little/ja/README.md)
で *動く*最小版（`check`＋`annotate`）を作り切ったあと、その先にある**高度な型理論と作り込み**へ
進みます。Scheme の名著にならって：

- **前編 = The Little chibirigor** … やさしい導入。動く最小版（完成）。
- **後編 = The Seasoned chibirigor** … 熟練編。理論の形式化と、実装の作り込み（本ディレクトリ）。

> ステータス：**着手（ドラフト）**。前編は「とても易しく・専門用語は後出し」でしたが、後編は
> *用語と形式*を正面から扱います。前編で**わざと避けた**もの（双方向型付けの形式化・変性・
> 再帰型・本物の型推論・健全性理論）を、ここで回収します。各章は前編の実装（`lib/`）を起点に、
> 必要に応じて拡張します。

## 前編との約束ごとの違い

| | The Little（前編） | The Seasoned（後編） |
|---|---|---|
| トーン | 数式・用語は後出し | 用語・記法を正面から |
| 引数 | `untyped` 止まり | 本体の使われ方から推論 |
| 部分型 | 「箱に入るか」の直感 | width/depth・変性の形式 |
| 再帰型 | 非対象 | μ型・余帰納 ↔ HKT |
| 健全性 | 「わざと見逃す」物語 | 健全性・正規化の理論 |

## 章立て（予定）

| 章 | ファイル | テーマ | 状態 |
|---|---|---|---|
| Part 1 | [part1-bidirectional-typing.md](part1-bidirectional-typing.md) | 双方向型付けの正体（`type_of`＝合成 `⇒`／`accepts`＝照合 `⇐`） | ドラフト |
| Part 2 | `part2-subtyping-and-variance.md` | 部分型と変性（width/depth、関数引数の反変。副読本 7 章の本丸） | 未着手 |
| Part 3 | `part3-real-inference.md` | 本物の型推論（引数推論：capability/duck、制約ベースの初歩。副読本 9 章演習の前線） | 未着手 |
| Part 4 | `part4-recursive-types.md` | 再帰型（μ型・余帰納）↔ Rigor の HKT/`App`＋fuel（副読本 8 章） | 未着手 |
| Part 5 | `part5-fact-store.md` | 完全な FactStore（6 バケツ・stability・クロージャ捕獲） | 未着手 |
| Part 6 | `part6-generics-and-substitution.md` | ジェネリクスと型代入・`erasure`（`subst`・α同値・変数捕獲。副読本 9 章） | 未着手 |
| Part 7 | `part7-soundness.md` | 健全性と正規化の理論（なぜ chibirigor はわざと unsound か） | 未着手 |
| Part 8 | `part8-toward-rigor.md` | Rigor の作り込みへの橋（プラグイン・キャッシュ・LSP・性能） | 未着手 |

> 各章は前編の実装を起点にします。前編が `lib/` の最小版で動いているのに対し、後編は
> *拡張版*や*別実装*を扱うので、コードは「前編からの差分」または「設計スケッチ」の形になります。

## 読む順番

前編（The Little chibirigor）を読み終えてから来てください。後編の各章は「前編のあの実装は、
実は◯◯という理論だった」「前編が避けた◯◯を、ここで作る」という形で進みます。
