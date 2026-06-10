# 再現性レビュー（前編 little, Part 0〜9）／2026-06-11

レンズ：型理論ゼロ・Ruby 中級者。本文だけから型チェッカーを再実装し、挙動を採点。

## 実験設定

- 読んだ対象：`book/v1/ja/little/part0〜9` ＋ `little/README.md` のみ。`lib/`・`test/`・`exe/`・`examples/`・`docs/`・`appendix/` は一切開かず。
- 実装先：`/tmp/chibirigor-repro-fc1/lib/chibirigor.rb`（単一ファイル）。
- 実行：`ruby -I /tmp/chibirigor-repro-fc1/lib`（ruby 4.0.5、Prism 同梱）。
- 採点：自作ハーネス（`/tmp/chibirigor-repro-fc1/harness.rb`）。`MOD` を `ChibiRigor`/`Chibirigor` 動的解決。`check` はサイズ照合、`annotate` は `to_s` 正規化で型文字列照合。代表 43 項目。

## 結論

**本文だけでほぼ完全に再実装でき、挙動も一致した。最終 SCORE: 43/43。**
実装を止める級の穴はゼロ。本文の `# =>`・run-block・コンソール例・演習の明示期待は、たどり着いた最終状態の実挙動と全て一致した。
推測で補ったのは「配線（呼び出し位置）が省略された箇所」2 つだけで、いずれも常識的に一意に決まり、出力は本文どおりになった。

## 章ごとの所見

| Part | clarity | 推測した所 | 食い違い |
|---|---|---|---|
| 0 はじめに | 概念のみ、実装なし。明確 | なし | なし |
| 1 リテラル/算術 | コード完全提示。明確 | なし | §1.3 の `1 + "x"` メッセージ `"整数に … は足せません"` は Part 2 で `"Integer が必要ですが … が渡されました"` に差し替わる（章をまたいだ正常進化。最終状態は後者） |
| 2 ディスパッチ | 表・class_of・dispatch 完全提示。明確 | なし | §1.3 で見せた診断メッセージが本章で置換される点（上記の裏返し） |
| 2-7 定数畳み込み（発展） | 方針は明確だが配線は散文 | **畳み込みを dispatch のどの位置で・どの演算に効かせるか**は「表側に置く」とだけ。`public_send` で汎用化したら本文の全例（`3`/`6`/`"aaa"`/予算超過 `Integer`）が一致 | なし |
| 3 Scope/文 | Scope・eval_statement・縫い込み完全提示。明確 | なし | なし |
| 4 Union | union ヘルパ・IfNode 完全提示。明確 | **else 無し IfNode** の扱い | 本章コード断片は `node.subsequent.statements.body.last` 直叩きで、else 無し `if` だと **その場でクラッシュ**する。だが §演習・run-block は `1 | nil` を期待。Part 5 のコードで `node.subsequent` nil ガード＋`Const[nil]` が補われ回収される設計（前編通読では破綻しないが、Part 4 単独のコードは動かない） |
| 5 ナローイング | remove_nil・narrow・possible? 完全提示。明確 | `is_a?` 引数が `ConstantReadNode` である点を Prism 知識で補完 | **`Type.union`（Type のモジュール関数）を呼ぶが、Part 4 では `union` を `Chibirigor` 直下に定義していた** → 同じ道具の置き場が章間で揺れる。両方に生やして解決 |
| 6 Hash/Tuple | HashShape・Tuple・read_index 完全提示 | **read_index を type_of_call のどこで呼ぶか**が未提示。`[]` 呼び出し時に第1引数ノードで先に引く配線を自作 → 本文の全例（既知キー→値、未知キー→nil、添字 OOB→nil）が一致 | なし |
| 7 accepts/三値 | accepts（Dynamic/Union 両方向）・dispatch の `:no` だけ報告まで完全提示。`ParenthesesNode` も「1 行足せ」と明示。明確 | なし | なし |
| 8 RBS/sig | Rbs.load/CORE・DefNode・type_of_body・method_signature 完全提示。明確 | なし | 「表を RBS 由来に差し替えても診断不変」を確認 ― Part 1〜7 の挙動が全て不変（differ 置換成立） |
| 9 gradual | union の untyped 伝播・baseline 完全提示。明確 | なし | baseline 提示コードは `baseline.include?(d)`（診断ハッシュ全体の包含）。本文の散文は「行＋メッセージで照合・列は含めない」と言うが、提示コードは全フィールド一致。**同一ソースでは例は通るが、列を変える編集を跨ぐと散文どおりには外れない**（実装メモと提示コードの粒度差。挙動例の採点には影響なし） |

## 公開 API の形

- `Chibirigor.check(source, baseline = []) -> Array<{line:,column:,length:,message:}>`
- `Chibirigor.annotate(source) -> Array<{line:, type:}>`（`def` 行はシグネチャ文字列）
- `Chibirigor.type_of(node, scope, diagnostics)` / `accepts(expected, actual) -> :yes/:no/:maybe`
- 型キャリア：`Type::Const/Nominal/Dynamic/Union/HashShape/Tuple`、`Type.union`
- `Scope`（不変）、`Dispatch`（`METHODS = Rbs.load(Rbs::CORE)`）、`Rbs.load/CORE`

## 自己採点タリー

```
SCORE: 43/43
```

内訳：Part1 6/6・Part2(含2-7) 8/8・Part3 4/4・Part4 4/4・Part5 3/3・Part6 5/5・Part7 5/5・Part8 4/4・Part9 4/4。
FP 安全ケース全通過：dead-branch `is_a?`（5.1）・Union 引数の非 FP（7.1）・未知キー→nil（6.3/6.5）・`untyped` 沈黙（7.3/8.4）・def 本体検査（8.3）・baseline（9.3/9.4）。

## 実装を止める級の穴

なし。
