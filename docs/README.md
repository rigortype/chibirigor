# chibirigor ドキュメント

このディレクトリに、chibirigor の**設計・本文（チュートリアル）・副読本対応**をすべて置く。
本家 Rigor のリポジトリとは分離している（実装の*最小版*と*本物*を取り違えないため）。

## 設計

- [`20260607-chibirigor-tutorial-draft.md`](20260607-chibirigor-tutorial-draft.md) ―
  チュートリアル全体の設計ドラフト。本編／続編の分割、全 Part の step 一覧、実装スケルトン、
  トーン・複雑さ予算、深掘り考察（v1→v3）。**作業スパインはこれ。**

## 本文（チュートリアル本体・試し書き）

各 Part の試し書き。1 step＝概念 1 つ＋小さなコード、三題噺（型理論 ↔ Ruby/RBS ↔ Rigor
実装の問題）、最後に実 Ruby で確認。コードはすべて実 Prism/Ruby で動作確認済み。

- [`20260607-chibirigor-part1-sample.md`](20260607-chibirigor-part1-sample.md) ―
  Part 1 リテラルと算術（最易・実装は `lib/` に反映済み）
- [`20260607-chibirigor-part2-sample.md`](20260607-chibirigor-part2-sample.md) ―
  Part 2 メソッド送信とディスパッチ（実装は `lib/` に反映済み）
- [`20260607-chibirigor-part3-sample.md`](20260607-chibirigor-part3-sample.md) ―
  Part 3 ローカル変数と不変 Scope（実装は `lib/` に反映済み）
- [`20260607-chibirigor-part4-sample.md`](20260607-chibirigor-part4-sample.md) ―
  Part 4 Union と絞り込み（難所）
- [`20260607-chibirigor-part5-sample.md`](20260607-chibirigor-part5-sample.md) ―
  Part 5 ハッシュと配列の型（中難度・open/closed）
- [`20260607-chibirigor-part6-sample.md`](20260607-chibirigor-part6-sample.md) ―
  Part 6 受理判定・三値・gradual（最難）

## 副読本との対応

- [`20260607-type-systems-distilled-rigor-mapping.md`](20260607-type-systems-distilled-rigor-mapping.md)
  ― 遠藤侑介『型システムのしくみ』↔ Rigor 実装の対応レポート。§12 が chibirigor 副読本としての
  評価（章ごとの価値表、本編/続編の分割原則）。

> 命名の日付プレフィックス（`20260607-`）は初期ドラフトの名残。本文を確定稿に起こす際に
> `book/part1-...` のような構成へ整理する予定（設計ドラフトの「残る未決事項」参照）。
