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

## 参考書について（任意）

後編は型理論を正面から扱うので、もう一段深く知りたい方向けに**任意の参考書**を 2 冊、各章の
脇に添えます。**どちらも必読ではありません**（無くても本書だけで完結します）。必要な所だけ
拾ってください。

- **『しくみ』** … 遠藤侑介『型システムのしくみ ― TypeScript で実装しながら学ぶ型と
  プログラミング言語』（ラムダノート）。TAPL のごく一部を*やさしく蒸留*した薄い本。
- **TAPL** … Benjamin C. Pierce『型システム入門 ― プログラミング言語と型の理論』
  （オーム社。原著 *Types and Programming Languages*, MIT Press）。本格的な教科書。後編が扱う
  変性・再帰型・型再構築・全称型などは、ここに本式の証明があります。

英語版（移植）を作るときは、`TAPL` がそのまま使える共通の参照先になります（『しくみ』は
日本語のみ）。だから後編では **TAPL の章番号を一次の道しるべ**に、『しくみ』を*補助*として
併記します。

## 前編との約束ごとの違い

| | The Little（前編） | The Seasoned（後編） |
|---|---|---|
| トーン | 数式・用語は後出し | 用語・記法を正面から |
| 引数 | `untyped` 止まり | 本体の使われ方から推論 |
| 部分型 | 「箱に入るか」の直感 | width/depth・変性の形式 |
| 再帰型 | 非対象 | μ型・余帰納 ↔ HKT |
| 健全性 | 「わざと見逃す」物語 | 健全性・正規化の理論 |

## 章立て（予定）

| 章 | ファイル | テーマ | 参考書（任意） | 状態 |
|---|---|---|---|---|
| Part 1 | [part1-bidirectional-typing.md](part1-bidirectional-typing.md) | 双方向型付けの正体（`type_of`＝合成 `⇒`／`accepts`＝照合 `⇐`） | TAPL 9 ／『しくみ』3 | ドラフト |
| Part 2 | [part2-subtyping-and-variance.md](part2-subtyping-and-variance.md) | 部分型と変性（width/depth、関数引数の反変） | TAPL 15・16 ／『しくみ』7 | ドラフト |
| Part 3 | [part3-real-inference.md](part3-real-inference.md) | 本物の型推論（引数推論：capability/duck、制約ベースの初歩） | TAPL 22 ／『しくみ』9 演習 | ドラフト |
| Part 4 | [part4-recursive-types.md](part4-recursive-types.md) | 再帰型（μ型・余帰納）↔ Rigor の HKT/`App`＋fuel | TAPL 20・21 ／『しくみ』8 | ドラフト |
| Part 5 | [part5-fact-store.md](part5-fact-store.md) | 完全な FactStore（6 バケツ・stability・クロージャ捕獲） | （フロー解析・Rigor 固有） | ドラフト |
| Part 6 | [part6-generics-and-substitution.md](part6-generics-and-substitution.md) | ジェネリクスと型代入・`erasure`（`subst`・α同値・変数捕獲） | TAPL 22・23 ／『しくみ』9 | ドラフト |
| Part 7 | [part7-soundness.md](part7-soundness.md) | 健全性と正規化の理論（なぜ chibirigor はわざと unsound か） | TAPL 8・12 ／『しくみ』おわりに | ドラフト |
| Part 8 | [part8-toward-rigor.md](part8-toward-rigor.md) | Rigor の作り込みへの橋（プラグイン・キャッシュ・LSP・性能） | （Rigor の ADR 群） | ドラフト |

> 各章は前編の実装を起点にします。前編が `lib/` の最小版で動いているのに対し、後編は
> *拡張版*や*別実装*を扱うので、コードは「前編からの差分」または「設計スケッチ」の形になります。

## 読む順番

前編（The Little chibirigor）を読み終えてから来てください。後編の各章は「前編のあの実装は、
実は◯◯という理論だった」「前編が避けた◯◯を、ここで作る」という形で進みます。
