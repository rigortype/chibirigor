---
title: The Little chibirigor (前編)
description: 動く最小版（check + annotate）を Part 0〜9 で作り切る、やさしい導入の巻。
sidebar:
  order: 1
---

# The Little chibirigor（前編）

最小の Ruby 型チェッカー `chibirigor` を、ステップバイステップで作りながら学ぶ前編です。
`check`（型診断）と `annotate`（推論した型の表示）が動く最小実装を Part 0〜9 で作り切ります。
コードは前編だけで動き切り、形式的な理論は後編へ譲ります。

> はじめての方は、二巻共通の入口 [README](../README.md)（動機、対象読者、環境構築）から読み始めてください。
> 用語は [用語集](../glossary.md)、巻をまたぐ参照情報は [付録](../appendix/) で引けます。
> 各章は「① 型理論 ↔ ② Ruby/RBS ↔ ③ Rigor 実装」という三つの視点から書かれ、章末に演習があります（くわしい読み方は [Part 0](part0-introduction.md) で）。

## 章立て

| Part | テーマ |
|---|---|
| [0](part0-introduction.md) | はじめに：推論を土台にした型チェッカー／拒まない入力／2 つの関数 |
| [1](part1-literals-and-arithmetic.md) | リテラルと算術（`Const`/`type_of`/`check`/`annotate`） |
| [2](part2-method-dispatch.md) | メソッド送信とディスパッチ（＋発展：定数畳み込み） |
| [3](part3-scope-and-statements.md) | ローカル変数と不変 Scope |
| [4](part4-union.md) | 型が一本に決まらない Union |
| [5](part5-narrowing.md) | 場合分けで型を絞るナローイング |
| [6](part6-hash-and-tuple.md) | ハッシュと配列の型（`HashShape`/`Tuple`） |
| [7](part7-accepts-and-trinary.md) | 受理判定と三値（`accepts` ＝ `:yes`/`:no`/`:maybe`） |
| [8](part8-rbs-and-signatures.md) | RBS と型シグネチャ（RBS 由来の表 → `def` から戻り型合成） |
| [9](part9-gradual-philosophy.md) | gradual の哲学（最終章） |

読み終えたら、後編 [The Seasoned chibirigor](../seasoned/README.md)（読み解く巻）へ続きます。
