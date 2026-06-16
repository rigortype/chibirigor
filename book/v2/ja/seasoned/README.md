---
title: The Seasoned chibirigor (後編)
description: 前編で作ったものの裏側を、形式の言葉で読み解く熟練編。Part 1〜8。
sidebar:
  order: 2
---

# The Seasoned chibirigor（後編）

前編[The Little chibirigor](../little/README.md)で動く最小版（`check`＋`annotate`）を
作り切ったあと、その裏側を形式の言葉で読み解く巻です。
前編でわざと避けたもの（双方向型付けの形式化、変性、ジェネリクス、再帰型、本物の型推論、FactStore、健全性）を、ここで回収します。
コードより概念が主役で、各章は前編の実装を起点にした解説と設計スケッチの集まりです（動くスケッチは[`examples/`](examples/README.md)にあり、`ruby <file>`で自己チェックが緑になります）。

> [!IMPORTANT]
> **後編は「作る」より「読み解く」巻です。**前編は「とても易しく、専門用語は後出し」でしたが、後編は用語と形式を扱います。
> 参考書（『しくみ』/TAPL）の対応は[付録a4](../appendix/a4-bibliography.md)に一元化しています（どちらも必読ではありません）。

## 章立て（構造から推論、フローを経て理論の頂点へ、そして橋へと続く一本の坂）

| Part | テーマ | 起点（前編） |
|---|---|---|
| [1](part1-bidirectional-typing.md) | 双方向型付けの正体（`type_of`＝合成`⇒`／`accepts`＝照合`⇐`） | 前編P7、P9 |
| [2](part2-subtyping-and-variance.md) | 部分型と変性（width/depth、戻り共変、引数反変） | 前編P6、P7 |
| [3](part3-generics-and-substitution.md) | ジェネリクスと型代入（`subst`、α 同値、変数捕獲、erasure） | 前編P8 |
| [4](part4-recursive-types.md) | μ と余帰納でみる再帰型（発展：HKT/`App`+fuel） | 前編P6 |
| [5](part5-real-inference.md) | 引数を埋める本物の型推論（能力収集、制約、単一化） | 前編P8 |
| [6](part6-fact-store.md) | 完全なFactStore（6バケツ、stability、クロージャ捕獲、join） | 前編P3、P5 |
| [7](part7-soundness.md) | 健全性と正規化、そして「わざとunsound」（gradualの2規律） | 前編P9 |
| [8](part8-toward-rigor.md) | 本物のRigorへ（プラグイン、キャッシュ、LSP、ADR、性能） | 前編全体 |

## 動く設計スケッチ

中核アルゴリズムは単体で走る最小のRubyとして[`examples/`](examples/README.md)に置いてあります。
`subtype.rb`（Part 2）、`subst.rb`（Part 3）、`mu_typeeq.rb`（Part 4）、`unification.rb`（Part 5）、`fact_invalidation.rb`（Part 6）の5本です。
本文とコードのドリフトは`check_docs.rb`が検出します。

## 読む順番

前編（The Little chibirigor）を読み終えてから来てください。
後編の各章は「前編のあの実装は、実は◯◯という理論だった」「前編が避けた◯◯を、ここで読み解く」という形で進みます。
