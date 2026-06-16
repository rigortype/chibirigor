---
title: 最小の Ruby 型チェッカー chibirigor を作りながら学ぶ
description: 二巻構成のオンラインブック。前編で動く最小版を作り、後編でその裏側を読み解く。
sidebar:
  order: 0
---

# chibirigor

最小のRuby型チェッカー`chibirigor`を、手を動かして作りながら学ぶオンラインチュートリアルです。
[chibivue](https://github.com/chibivue-land/chibivue)がVueを小さく作り直して学ぶように、私たちは本物の[Rigor](https://github.com/rigortype/rigor)（Ruby向けの漸進的（gradual）型チェッカー）を小さく作り直して学びます。

> [!NOTE]
> このディレクトリは清書版v2（日本語）です。原稿は`draft/`にあります。

## この本で得られるもの

数十〜数百行のRubyを書きながら、次の問いに自分の手で答えられるようになります：

- 型チェッカーは、ソースコードからどうやって型を「推論」するのか。
- なぜRigorは「型の付かないコード」を拒まず、それでも矛盾だけを報告できるのか。
- 「動くコードを脅かさない（never frighten working code）」とは、設計としてどういうことか。

読み終えると、`check`（型診断）と`annotate`（推論した型の表示）が動く小さな型チェッカーが手元に残り、本物のRigorのソースを読む足がかりが得られます。

## 対象読者と前提知識

- **Ruby中級者**：クラス、モジュール、`case`/`when`、ブロックを普通に読み書きできること。
- **型システムの理論知識は不要**（前編）。前編の本体で使う概念は、その都度やさしく導入します。
- 前提とするのはRubyだけ。数式や証明は前編では使いません（後編で扱います）。

## 二巻構成

Schemeの名著（*The Little Schemer* / *The Seasoned Schemer*）にならい、二巻に分かれています。

- **前編 = The Little chibirigor**（[`little/`](little/README.md)）：やさしい導入。`check`/`annotate`が動く最小実装をPart 0〜9で作り切ります。コードは前編だけで動き切り、形式的な理論は後編へ譲ります。
- **後編 = The Seasoned chibirigor**（[`seasoned/`](seasoned/README.md)）：熟練編。前編で作ったものの裏側を、形式の言葉で読み解く巻です（双方向型付け、部分型と変性、ジェネリクス、再帰型、型推論、FactStore、健全性）。記法と用語を扱います。

> [!NOTE]
> 前編は「作る」巻、後編は「読み解く」巻です。後編はコードより概念が主役になります。

どちらから読んでもよいですが、初めてなら前編 → 後編の順を勧めます。

## 各章を貫く三つの視点

各章は、小さな**三つの視点（パースペクティブ）**から書かれています。
「① 型理論 ↔ ② Ruby/RBS ↔ ③ Rigor実装」を重ね合わせる構成です（くわしくは[Part 0](little/part0-introduction.md)で）。

各章末には、手を動かして確かめるための演習があります。
巻末に用語集（[`glossary.md`](glossary.md)）と、巻をまたぐ参照情報をまとめた付録（[`appendix/`](appendix/)）も用意しました。

## 参考書（任意）

型理論をもう一段深く覗きたい方のために、各章の脇に参考書メモを添えます。
**どちらも必読ではありません**（無くても本書だけで完結します）。

- **『しくみ』**：遠藤侑介『型システムのしくみ ― TypeScriptで実装しながら学ぶ型とプログラミング言語』（ラムダノート）。
- **TAPL**：Benjamin C. Pierce『型システム入門 ― プログラミング言語と型の理論』（オーム社。原著*Types and Programming Languages*）。英語で読むなら原著がそのまま参照先になります。

対応早見表は付録[`appendix/a4-bibliography.md`](appendix/a4-bibliography.md)に一元化しています。

## 環境構築

Ruby 3.4以降（パーサの**Prism**が同梱）があれば動きます。テストフレームワークも使いません。

```console
$ git clone <このリポジトリ>
$ cd chibirigor
$ ruby exe/chibirigor check    path/to/file.rb   # 型診断
$ ruby exe/chibirigor annotate path/to/file.rb   # 推論型の表示
```

本文のコードを写経していくなら、自分の作業ディレクトリに`lib/`を作り、章ごとにファイルを育てていけば十分です。
各章のコードはすべて、実際のPrism/Rubyで動作することを確認しています。

## 目次

### 前編 The Little chibirigor（作る）

| Part | テーマ |
|---|---|
| [0](little/part0-introduction.md) | はじめに：推論を土台にした型チェッカー |
| [1](little/part1-literals-and-arithmetic.md) | リテラルと算術 |
| [2](little/part2-method-dispatch.md) | メソッド送信とディスパッチ |
| [3](little/part3-scope-and-statements.md) | ローカル変数と不変Scope |
| [4](little/part4-union.md) | 型が一本に決まらないUnion |
| [5](little/part5-narrowing.md) | 場合分けで型を絞るナローイング |
| [6](little/part6-hash-and-tuple.md) | ハッシュと配列の型 |
| [7](little/part7-accepts-and-trinary.md) | 受理判定と三値 |
| [8](little/part8-rbs-and-signatures.md) | RBSと型シグネチャ |
| [9](little/part9-gradual-philosophy.md) | gradualの哲学（最終章） |

### 後編 The Seasoned chibirigor（読み解く）

| Part | テーマ |
|---|---|
| [1](seasoned/part1-bidirectional-typing.md) | 双方向型付けの正体 |
| [2](seasoned/part2-subtyping-and-variance.md) | 部分型と変性 |
| [3](seasoned/part3-generics-and-substitution.md) | ジェネリクスと型代入 |
| [4](seasoned/part4-recursive-types.md) | μ と余帰納でみる再帰型 |
| [5](seasoned/part5-real-inference.md) | 引数を埋める本物の型推論 |
| [6](seasoned/part6-fact-store.md) | 完全なFactStore |
| [7](seasoned/part7-soundness.md) | 健全性と正規化、そして「わざとunsound」 |
| [8](seasoned/part8-toward-rigor.md) | 本物のRigorへ（最終章） |

### 付録

- [a1特別な型カタログ](appendix/a1-special-types.md)：`untyped`/`void`/`never`/`Top`/`Bot`
- [a2ナローイングのパターン集](appendix/a2-narrowing-patterns.md)（後編Part 6の予習。前編だけなら飛ばし可）
- [a3道具](appendix/a3-tooling.md)：`--explain`/型表示の二段構え/`trace`/dispatchカスケード
- [a4参考書とADRの対応早見表](appendix/a4-bibliography.md)
- [a5他言語からの橋渡し](appendix/a5-other-languages.md)：null安全、名前的と構造的の部分型、HashShapeの系譜、無タグのUnion

では、Part 0から始めましょう。
