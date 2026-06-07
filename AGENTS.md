# AGENTS.md — chibirigor

chibirigor は **Rigor の教育用・最小版**です。実物の
[Rigor](https://github.com/)（Ruby 向けの漸進的型検査器）のアーキテクチャの「最小版」を、
ステップバイステップで作っていく教材リポジトリです。

## 最重要：Rigor と混同しない

- **ここは Rigor 本体ではありません。** Rigor の実装仕様 ― 豊富な型カリア群、推論エンジン、
  RBS 連携、ADR、正規化規則、FactStore の 6 バケツ、HKT/`App`、性能最適化など ― を
  **持ち込まないでください。**
- chibirigor は*意図的に簡略化したモデル*です。Rigor の挙動を「正解」として chibirigor の
  コードを“直さ”ないでください。逆に、chibirigor の簡略モデルを Rigor の仕様と
  **取り違えないでください**。
- `docs/` に Rigor との対応レポートがありますが、それは「学習用の地図」であって移植指示では
  ありません。

## 設計の鉄則（正は `docs/` の設計ドラフト）

- **とても易しく。** 1 step ＝ 難しいこと 1 つ。複数の難所を同時に持ち込まない。
- **誤検知を出さない**（"never frighten working code"）。わからない所は `untyped`（`Dynamic`）に
  逃がす。
- **拒まない**：Ruby が構文エラーにしないコードは受理する（パーサに Prism を使うので解釈できる
  範囲はさらに広い）。ただし「型が付く＝動く」は保証しない。
- **高度な内容は続編へ**：双方向型付けの形式化・変性・再帰型・本物の型推論・健全性理論は
  「The Seasoned chibirigor」に送る（本編に持ち込まない）。

## アーキテクチャ（最小版）

`lib/chibirigor/` … `type.rb`（型カリア）/ `type_of.rb`（式→型の合成）/ `scope.rb` /
`evaluator.rb`（文を縫う）/ `dispatch.rb`（メソッド送信）/ `rbs.rb`（ミニ RBS 読み込み）/
`accepts.rb`（三値受理判定）/ `narrowing.rb` / `checker.rb` / `annotator.rb`、
`exe/chibirigor`（CLI）。各 Part が 1 ファイルを足す/育てる。

## 実行・テスト

```console
$ ruby exe/chibirigor check FILE      # 型診断
$ ruby exe/chibirigor annotate FILE   # 推論型の表示
$ ruby test/test_part1.rb             # 依存ゼロの plain Ruby テスト
```

Ruby 3.4 以降（Prism 同梱）。テストフレームワークは使わない。

## フォーマッタ注意

環境の rubocop autocorrect が `examples/*.rb` の「わざとおかしいコード」（未使用変数・型
エラーを含む例）を削除/書き換えます。対策：

- サンプルは rubocop-clean に保つ。
- 挙動の検証は `test/` の**文字列ソース**で行う（整形の影響外）。
- リテラル列の `annotate` デモは `/dev/stdin` パイプで見せる。

## 本文・ドキュメント

- **本文（チュートリアル）**：`draft/` 以下。二巻構成で前編 = **The Little chibirigor**
  （`draft/little/ja/`、ファーストドラフト・全 9 章）、後編 = **The Seasoned chibirigor**
  （`draft/seasoned/ja/`、未着手）。
- **設計・副読本対応**：`docs/`。
- いずれも Rigor リポジトリには置かない（分離を維持するため）。
