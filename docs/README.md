# chibirigor ドキュメント

このディレクトリには **設計・副読本対応** を置く。**本文（チュートリアル）は `draft/` に分離**。
chibirigor 全体は本家 Rigor のリポジトリとは分離している（実装の*最小版*と*本物*を取り違えない
ため）。

## 本文（チュートリアル）― `draft/` 以下

二巻構成。Scheme の名著にならって前編 = **The Little chibirigor**、後編 = **The Seasoned
chibirigor** と呼ぶ。

- 前編（ファーストドラフト・日本語）：[`../draft/little/ja/`](../draft/little/ja/README.md)
  ― 全 9 章。各章は実装（`lib/`）と対応し、コードは実 Prism/Ruby で動作確認済み。
- 後編：`draft/seasoned/ja/`（未着手）― 双方向型付けの形式化・変性・再帰型・本物の型推論・
  健全性理論など。

## 設計

- [`20260607-chibirigor-tutorial-draft.md`](20260607-chibirigor-tutorial-draft.md) ―
  チュートリアル全体の設計ドラフト。前編／後編の分割、全 Part の step 一覧、実装スケルトン、
  トーン・複雑さ予算、深掘り考察（v1→v3）。**作業スパインはこれ。**

## 副読本との対応

- [`20260607-type-systems-distilled-rigor-mapping.md`](20260607-type-systems-distilled-rigor-mapping.md)
  ― 遠藤侑介『型システムのしくみ』↔ Rigor 実装の対応レポート。§12 が chibirigor 副読本としての
  評価（章ごとの価値表、前編/後編の分割原則）。

> 命名の日付プレフィックス（`20260607-`）は初期ドラフトの名残。設計・対応レポートを確定稿に
> 起こす際に整理する予定。
