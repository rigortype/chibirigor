# chibirigor

最小限の Ruby 型推論器を、ステップバイステップで実装して学ぶオンラインブック／チュートリアル。
[chibivue](https://github.com/chibivue-land/chibivue) をモデルに、本物の
[Rigor](https://github.com/.../rigor) のアーキテクチャの「最小版」を忠実に作っていきます。

## これは何か

- **検査器ではなく推論器**：型注釈を*検査*するのではなく、Ruby のコードから型を*導出*します。
  `check`（型診断）は推論の副産物です。
- **拒まない**：Ruby が構文エラーにしないコードはすべて受け入れます（パーサに Prism を使うので、
  解釈できる範囲はさらに広い）。ただし「型が付く＝動く」を保証するものではありません。
- **動くコードを脅かさない**：型がわからない所は `untyped` に逃がし、確実な矛盾だけを報告します。

副読本として、遠藤侑介『型システムのしくみ ― TypeScript で実装しながら学ぶ型とプログラミング
言語』（ラムダノート）を各章で参照します。型理論 ↔ Ruby/RBS ↔ Rigor の実装、この三つを
やさしく結びつけるのがねらいです。

## 提供機能

```console
$ chibirigor check FILE      # 型診断を出す
$ chibirigor annotate FILE   # 推論した型を見せる
```

## 使ってみる（Part 1 時点）

`check` は型の矛盾を報告します（知らないものは黙って通す）:

```console
$ ruby exe/chibirigor check examples/ok.rb
型エラーはありません

$ ruby exe/chibirigor check examples/ng.rb
examples/ng.rb:4: 整数に true は足せません
```

`annotate` は推論した型を見せます（リテラルは値そのもの、足し算は Integer に丸め、
不明は untyped）:

```console
$ printf '42\n1 + 2\nfoo.bar\n' | ruby exe/chibirigor annotate /dev/stdin
1: 42
2: Integer
3: untyped
```

## テスト

```console
$ ruby test/test_part1.rb
```

## 進捗

- [x] Part 1 ― リテラルと算術（`Const` / `type_of` / `check` / `annotate`）
- [x] Part 2 ― メソッド送信とディスパッチ（`Dispatch` 表 / アリティ・型チェック / 未知は degrade）
- [x] Part 3 ― Scope と文（不変 `Scope` / `eval_statement` で文を縫う / 再代入）
- [x] Part 4 ― Union と絞り込み（`Union` / `if`・三項で枝をまとめる / `nil?`・`is_a?` ナローイング / dead branch は絞らず FP 回避）
- [x] Part 5 ― ハッシュと配列の型（`HashShape` / `Tuple` / `h[:k]`・`a[0]` 読み / 未知キーは nil で FP 回避）
- [x] Part 6 ― 受理判定・三値（`accepts` ＝ `:yes`/`:no`/`:maybe` / `:no` だけ報告 / Union 引数の FP を解消）
- [x] Part 7 ― RBS ひとさじ（手書き表を `Rbs.load` で RBS テキストから生成・differ 置換）
- [x] Part 8 ― annotate を仕上げる（`def` の戻り型を本体から合成・RBS 風 sig・`untyped` で推論の穴を可視化）
- [x] Part 9 ― まとめ：gradual の哲学（`untyped` の伝播 / わざと見逃す総括 / baseline）

> 高度な内容（双方向型付けの形式化、変性、再帰型、本物の型推論、健全性理論ほか）は
> 続編「The Seasoned chibirigor」に送ります。

## ドキュメント

本家 Rigor とは**意図的に分離**しています（実装の*最小版*と*本物*を取り違えないため）。

- **本文（チュートリアル）** ― 二巻構成。Scheme の名著にならい前編 = **The Little chibirigor**、
  後編 = **The Seasoned chibirigor**。
  - 前編（ファーストドラフト・日本語、全 9 章）：[`draft/little/ja/`](draft/little/ja/README.md)
  - 後編：`draft/seasoned/ja/`（未着手）
- **設計・副読本対応**：[`docs/`](docs/README.md)（設計の作業スパイン、『型システムのしくみ』
  との対応レポート）。

## 必要環境

Ruby 3.4 以降（Prism 同梱）。
