# The Seasoned chibirigor ― 動く設計スケッチ

後編の中核アルゴリズムを、**単体で走る最小の Ruby** にしたもの。前編が「コードは実際に動かして
検証」したのと同じ流儀で、後編の主張も*動いて緑になる*形で裏付けます。依存ゼロ、Ruby 3.4+
（テストフレームワークも使いません）。各ファイルは末尾に自己チェックを持ち、`ruby <file>` で
`PASS` が並べば成功です。

| ファイル | 対応章 | 何を確かめるか |
|---|---|---|
| [`subtype.rb`](subtype.rb) | Part 2 部分型と変性 | 幅／深さ部分型、**引数反変・戻り共変**（逆向きが false になる） |
| [`mu_typeeq.rb`](mu_typeeq.rb) | Part 4 再帰型 | μ 型の畳/展開の等価、**α 同値**、**余帰納（`seen`）で停止** |
| [`subst.rb`](subst.rb) | Part 6 型代入 | **シャドーイング**で止める、**変数捕獲**を fresh 変数で回避 |

```console
$ ruby subtype.rb
$ ruby mu_typeeq.rb
$ ruby subst.rb
```

> これらは*教育用の設計スケッチ*で、本物の Rigor のコードではありません（型の表現も判定も最小
> 限）。それでも、後編が言葉で説明した仕組み ― 反変・余帰納・捕獲回避 ― が、実際に手元で
> 走って正しい答えを返すことを確かめられます。

## ドリフト防止 ― 本文コードと example を機械的に同期する

本文（章 `.md`）に貼ったコードは、手で写すと必ず実装からズレます（実際、整形ツールが
`_1` を `it` に書き換えただけでもズレます）。これを **`check_docs.rb`** が機械的に防ぎます。

仕組みは 3 つだけ：

1. **examples が全部緑か** ― `ruby <file>` が exit 0（自己チェック PASS）。
2. **コードの逐語同期** ― 本文の ```code ブロックに `<!-- include: file.rb#region -->` を付けると、
   その file の「`# region <id>` … `# endregion`」区間と**バイト一致**しているかを検査。
3. **出力の逐語同期** ― 本文の ```text ブロックに `<!-- run: file.rb -->` を付けると、各行が
   その file の**実出力にそのまま含まれる**か（subset 可）を検査。

使い方（依存ゼロ・stdlib のみ）：

```console
$ ruby check_docs.rb        # 検査（ドリフトがあれば exit 1）
$ ruby check_docs.rb --fix  # include ブロックを region から再生成して同期
```

**運用**：本文に新しくコードを引くときは、example 側に `# region <id> … # endregion` を置き、
本文側に `<!-- include: file#id -->` を付ける（中身は `--fix` が region から流し込む）。整形ツールが
example を書き換えても、`check_docs.rb --fix` を一度回せば本文が追従します。CI に
`ruby check_docs.rb` を入れれば、ズレたまま push されるのを止められます。

> **前編・実装への拡張**：同じ仕組みは前編（`draft/little/ja/*.md` の本文 ↔ `lib/chibirigor/*.rb`）
> にもそのまま使えます ― `lib` 側に region を置き、本文に `include` を付け、対象 md ディレクトリを
> 増やすだけ。前編側の適用は別途（引き継ぎメモ参照）。
