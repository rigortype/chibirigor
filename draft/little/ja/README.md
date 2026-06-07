# The Little chibirigor（前編・日本語版・ファーストドラフト）

最小限の Ruby 型推論器 `chibirigor` を、ステップバイステップで作りながら学ぶ
オンラインブックの**前編**。Scheme の名著にならい、二巻構成の名前をこう呼びます：

- **前編 = The Little chibirigor** … やさしい導入。`check` ＋ `annotate` が動くまで（本ディレクトリ）。
- **後編 = The Seasoned chibirigor** … 高度な型理論・作り込み（`draft/seasoned/ja/`、未着手）。

> ステータス：**ファーストドラフト**。各章は実装（`lib/`）と対応し、コード断片はすべて実
> Prism/Ruby で動作確認済み。文章・章タイトル・構成は今後の清書で変わり得ます。
> 設計の作業スパインは [`../../../docs/20260607-chibirigor-tutorial-draft.md`](../../../docs/20260607-chibirigor-tutorial-draft.md)、
> 副読本『型システムのしくみ』との対応は
> [`../../../docs/20260607-type-systems-distilled-rigor-mapping.md`](../../../docs/20260607-type-systems-distilled-rigor-mapping.md)。

## 章立て

各章は「三題噺」（① 型理論 ↔ ② Ruby/RBS の見え方 ↔ ③ Rigor 実装の問題）で書かれています。

| 章 | ファイル | テーマ |
|---|---|---|
| Part 0 | [part0-introduction.md](part0-introduction.md) | はじめに：検査器ではなく推論器／拒まない入力／2 つの関数 |
| Part 1 | [part1-literals-and-arithmetic.md](part1-literals-and-arithmetic.md) | リテラルと算術（`Const`/`type_of`/`check`/`annotate`） |
| Part 2 | [part2-method-dispatch.md](part2-method-dispatch.md) | メソッド送信とディスパッチ |
| Part 3 | [part3-scope-and-statements.md](part3-scope-and-statements.md) | ローカル変数と不変 Scope |
| Part 4 | [part4-union-and-narrowing.md](part4-union-and-narrowing.md) | Union と絞り込み（ナローイング） |
| Part 5 | [part5-hash-and-tuple.md](part5-hash-and-tuple.md) | ハッシュと配列の型 |
| Part 6 | [part6-accepts-and-trinary.md](part6-accepts-and-trinary.md) | 受理判定・三値 |
| Part 7 | [part7-rbs.md](part7-rbs.md) | RBS ひとさじ |
| Part 8 | [part8-annotate.md](part8-annotate.md) | annotate を仕上げる |
| Part 9 | [part9-gradual-philosophy.md](part9-gradual-philosophy.md) | gradual の哲学（最終章） |

前編はこれで全 10 章（Part 0〜9）が揃いました。後編 **The Seasoned chibirigor**
（`../../seasoned/ja/`）へ続きます。
