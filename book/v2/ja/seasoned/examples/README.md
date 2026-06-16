# The Seasoned chibirigor ― 動く設計スケッチ

後編の中核アルゴリズムを、**単体で走る最小のRuby**にしたもの。前編が「コードは実際に動かして
検証」したのと同じ流儀で、後編の主張も*動いて緑になる*形で裏付けます。依存ゼロ、Ruby 3.4+
（テストフレームワークも使いません）。各ファイルは末尾に自己チェックを持ち、`ruby <file>`で
`PASS`が並べば成功です。

| ファイル | 対応章 | 何を確かめるか |
|---|---|---|
| [`subtype.rb`](subtype.rb) | Part 2 部分型と変性 | 幅／深さ部分型、**引数反変・戻り共変**（逆向きが false になる） |
| [`mu_typeeq.rb`](mu_typeeq.rb) | Part 4 再帰型 | μ 型の畳/展開の等価、**α 同値**、**余帰納（`seen`）で停止** |
| [`subst.rb`](subst.rb) | Part 3 ジェネリクスと型代入 | **シャドーイング**で止める、**変数捕獲**を fresh 変数で回避 |

```console
$ ruby subtype.rb
$ ruby mu_typeeq.rb
$ ruby subst.rb
```

> これらは*教育用の設計スケッチ*で、本物のRigorのコードではありません（型の表現も判定も最小
> 限）。それでも、後編が言葉で説明した仕組み ― 反変・余帰納・捕獲回避 ― が、実際に手元で
> 走って正しい答えを返すことを確かめられます。

## ドリフト防止 ― 本文コードとexampleを機械的に同期する

本文（章`.md`）に貼ったコードは、手で写すと必ず実装からズレます（実際、整形ツールが
`_1`を`it`に書き換えただけでもズレます）。これを**`check_docs.rb`**が機械的に防ぎます。

仕組みは3つだけ：

1. **examplesが全部緑か** ― `ruby <file>`がexit 0（自己チェックPASS）。
2. **コードの逐語同期** ― 本文の```code ブロックに `<!-- include: file.rb#region -->`を付けると、
   そのfileの「`# region <id>` … `# endregion`」区間と**バイト一致**しているかをチェック。
3. **出力の逐語同期** ― 本文の```text ブロックに `<!-- run: file.rb -->`を付けると、各行が
   そのfileの**実出力にそのまま含まれる**か（subset可）をチェック。

使い方（依存ゼロ・stdlibのみ）：

```console
$ ruby check_docs.rb        # チェック（ドリフトがあれば exit 1）
$ ruby check_docs.rb --fix  # include ブロックを region から再生成して同期
```

**運用**：本文に新しくコードを引くときは、example側に`# region <id> … # endregion`を置き、
本文側に`<!-- include: file#id -->`を付ける（中身は`--fix`がregionから流し込む）。整形ツールが
exampleを書き換えても、`check_docs.rb --fix`を一度回せば本文が追従します。CIに
`ruby check_docs.rb`を入れれば、ズレたままpushされるのを止められます。

> **前編・実装への拡張**：同じ仕組みは前編（`draft/little/ja/*.md`の本文 ↔ `lib/chibirigor/*.rb`）
> にもそのまま使えます ― `lib`側にregionを置き、本文に`include`を付け、対象mdディレクトリを
> 増やすだけ。前編側の適用は別途（引き継ぎメモ参照）。
