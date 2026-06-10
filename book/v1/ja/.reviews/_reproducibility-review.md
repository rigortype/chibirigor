# L2 伝・再現性 再査読（2026-06-09／改修後）

**実験設定**：型理論の知識ゼロの Ruby 中級者を演じ、`book/v1/ja/little/part0〜9` の**本文だけ**を
Part 0 から順に読み、各章で導入された機能を `/tmp/chibirigor-repro-l2/lib/chibirigor.rb` に
インクリメンタルに実装。素の `ruby 4.0.5 (+PRISM)` で `ruby -I .../lib <script>` として実行。
`lib/`・`test/`・`exe/`・`examples/`・`appendix/` は一切開いていない。

**結論（先に）**：本文の再現性は**非常に高い**。最終状態で**期待出力つきの例 36/36 が一致**。
本文だけで `check`/`annotate` が一通り動く型チェッカーを最後まで作り切れた。本物の穴は「**実装を
止める**」級ではなく、すべて**本文内の表記ゆれ（プロセ中の `# =>` がコードの実挙動と食い違う）**と
**少数の未定義ヘルパ**に限られる。付録に逃がした内容が本文実装に必須だった事故は**無し**。

---

## 章ごとの clarity / 推測した所 / 食い違い

| 章 | clarity | 推測した所（本物の穴の候補） | 本文の期待出力との食い違い |
|---|---|---|---|
| 0 | 概念のみ。実装なし。`check`/`annotate`/`type_of`/`accepts` の役割と「拒まない・脅かさない」が明確 | なし | なし（出力例なし） |
| 1 | 完全。`Const`/`Nominal`/`Dynamic`・`type_of`・`type_of_call`・`check`・`annotate` が全文掲載。そのまま動く | なし（コードがほぼ全部載っている） | なし。導入時点で 4/4 一致（`check("1+2")`=[]、`1+"x"` の診断文、`foo.bar`=[]、annotate 4 行） |
| 2 | 高い。`Dispatch` 表・`class_of`・`matches?`・`dispatch`・2-7 の畳み込みまで掲載 | (a) 畳み込みを「Dispatch 側に置く」とは書くが、`fold` の具体配線（diff 後に試す、`*`/`-` も畳む、予算判定の位置）は自前で組んだ。(b) 165-166 の `check(...) # ["..."]` は**文字列の配列**表記だが実戻り値は `{line:,message:}` の配列（表記ゆれ） | なし（実挙動はすべて一致。表記ゆれのみ） |
| 3 | 完全。`Scope`・`eval_statement`・縫い込みループが全文掲載。`type_of` に scope 引数を通す改修も明示 | `type_of` の新シグネチャ `(node, scope, diagnostics)` は「scope を渡す」とあるが全文再掲はない（一意なので推測コストは低い） | なし。3-4 の annotate 4 行・各 check 例とも一致 |
| 4 | 中。`Union`/`union`/`IfNode` 型付けは掲載 | (a) **`nil` リテラルの型付けが本文に無い**のに 99 行・else 無し if（演習2）で `nil` を要求 → `NilNode → Const[nil]` を推測。(b) else 無し if（`node.subsequent` が nil）の扱いは Part 4 のコードに無く Part 5 で初出 → Part 4 時点では推測が必要。(c) `union` の置き場所が bare def で曖昧（Part 5 で `Type.union` と判明） | **2件**：62 行 `type_of(...) # => Integer \| String` だが実挙動は `1 \| "a"`（Const のまま）。演習1 は `Integer \| Integer ではなく Integer になる` と書くが実挙動は `1 \| 2`。どちらも**プロセが「クラスへ丸めた名前」で書かれており Const 設計と矛盾**。`<!-- run -->` ブロック（`c ? 1 : "a" -> 1 \| "a"`）は実挙動と一致 |
| 5 | 中。`remove_nil`/`narrow`/`possible?`/`IfNode` 絞り込みを掲載 | **nil 表現の不整合**：`remove_nil` は `== Nominal[:NilClass]` で除去、`narrow` は truthy 側を `Nominal[:NilClass]` に絞るが、Part 4 の `nil` は `Const[nil]`。**本文のコードを逐語実装すると `Integer\|nil` から nil を剥がせず、自分の章の例が通らない**。`Const[nil]`/`Nominal[:NilClass]` 両対応の `nil_type?` を自前で補って解決した（本物の穴） | 逐語実装なら食い違うが、補正後は一致。`<!-- run -->` の挙動（nil? narrowing OK / String 診断）は再現 |
| 6 | 高い。`HashShape`/`Tuple`・`type_of` の2 case・`read_index` を掲載 | `read_index` を `type_of_call` の `[]` 経路にどう挿すか（引数1個・型ではなく**生ノード**を渡す）は本文に配線図が無く推測。挿し場所は自然に決まる | なし。run-block 4 行・読み出し例・ネスト演習すべて一致 |
| 7 | 高い。`accepts`（三値・widen・Union 両方向）・`dispatch` の `:no` だけ報告を掲載 | **`type_of_body` を本文未定義のまま呼ぶ**（143 行 `ParenthesesNode then type_of_body(node.body,...)`）。Part 8 で正式定義されるが Part 7 時点では推測実装が必要（本物の穴・小） | なし。`accepts` 5 例・check 例・Union 引数例すべて一致。run-block の診断文も一致 |
| 8 | 高い。`Rbs`(CLASS_LINE/DEF_LINE/CORE/load)・`Dispatch::METHODS=Rbs.load`・`DefNode`・`method_*`・`annotate` 改修を掲載。`type_of_body` をここで正式定義 | `type_of_body` の定義が Part 7（`Dynamic` 想定）と Part 8（`Const[nil]` 想定・scope を縫う）で**2 種**あり、後者を採用。差し替えで Part 1〜7 が緑、は実際に確認 | なし（明示の `# =>` 範囲では）。**注記**：演習4 `def f; 1+2; end` のシグネチャは畳み込みが効いて `def f: () -> 3` になる（`Integer` ではなく値 `3` が RBS 風 sig に漏れる）。演習に明示期待値は無いので不一致カウントには入れない |
| 9 | 高い。`union` の untyped 伝播・`baseline` を掲載 | なし（配線は明確） | **1件（コードと本文テキストの食い違い）**：baseline のコード片は `baseline.include?(d)`（ハッシュ完全一致）だが、診断は `column`/`length` を持つので**桁ズレで baseline が外れる** → §9-3 本文が明言する「**行＋メッセージで照合・列は含めない**」と矛盾。本文テキスト（＝意図）に従って line+message 照合で実装。run-block・baseline 演習は一致 |

---

## 自己採点タリー（期待出力つきの例）

最終状態（Part 9 まで実装した状態）で、本文中の `# =>` / `check(...)` / run-block / コンソール例 /
演習の明示期待を実際に走らせて照合：

**最終状態：36 / 36 一致（100%）**

内訳：P2 表/畳み込み 8、P3 4、P4 run-block 2、P5 2、P6 2、P7 10、P8 4、P9 4。
（各章「導入時点」でも、その章の例はすべて一致していた。例：Part 1 は導入時点で 4/4。）

### 集計外＝設計進化／表記ゆれ／逐語コードの不整合

1. **Part 1 の診断文・`annotate("1+2")`**（導入時点では一致）。最終状態では Part 2 がディスパッチ表へ
   置換し診断文が `Integer が必要ですが "x" が渡されました` に、`1+2` は畳み込みで `3` になる。
   本文が明示する**意図的な進化**（本文：「`+` 専用コードを捨てて」「`exe` で `1+2` が `3` と出る」）。
   再現失敗ではない。

2. **Part 4 line 62 `# => Integer | String`** … 実挙動 `1 | "a"`。引用：
   > `type_of(parse("rand < 0.5 ? 1 : \"a\""))   # => Integer | String`
   Const 設計（Part 1：リテラルは丸めず `Const`、annotate で `42` と出る）と矛盾。
   同章の `<!-- run -->` は `c ? 1 : "a"  ->  1 | "a"` で**実挙動側が正**。

3. **Part 4 演習1** … 引用：
   > `rand < 0.5 ? 1 : 2` の型を `annotate` で確かめ、なぜ `Integer | Integer` ではなく `Integer` に
   > なるのか
   実挙動は `1 | 2`。`union` の重複除去は効くが、被るのは**同一 Const のときだけ**。`Const[1]` と
   `Const[2]` は別物なので畳まれない。プロセが「リテラルはクラスに丸まる」前提で書かれている。

（2・3 はいずれも**本文プロセの自己矛盾**＝本物の穴。実装は止まらないが、読者が
「本文どおりに作ったのに `# =>` と違う」と必ず混乱する。）

---

## 本物の穴（実装可能性・挙動一致の観点で上位）

1. **【穴・中】Part 4／5 の `nil` 表現が章をまたいで不整合。** Part 4 は `nil` を `Const[nil]`
   （`to_s` が `nil`）で導入する必要があるのに、Part 5 の `remove_nil`/`narrow` は `Nominal[:NilClass]`
   を前提にした逐語コード。**Part 5 のコードをそのまま打つと、自章の `Integer|nil` ナローイング例が
   通らない**。読者は `nil` の表現をどちらかに統一する判断を自力で迫られる。最も「詰まる」箇所。

2. **【穴・中】Part 4 の `nil` リテラルと else 無し `if` の扱いが Part 4 本文に無い。** `NilNode` の
   型付けも、`node.subsequent` が nil のときの分岐も、Part 4 のコードには現れず Part 5 で初出。
   Part 4 の演習2（else 無し if → `1 | nil`）を Part 4 だけで解こうとすると手が止まる。

3. **【穴・中】Part 4 の `# =>` 2 件が Const 設計と矛盾**（上記 2・3）。動かすと本文と違う出力が出る、
   という最も直接的な再現性の傷。

4. **【穴・小】`type_of_body` が Part 7 で未定義のまま使われ、Part 8 でようやく定義（しかも Part 7 の
   想定挙動と微妙に異なる）。** Part 7 単体では推測実装が要る。

5. **【穴・小】Part 9 baseline のコード片（`include?` 完全一致）が、同節本文の「列は含めない」明言と
   矛盾。** 列・長さ付き診断に対し `include?` を使うと桁ズレで照合が壊れる。コードと散文のどちらに
   従うか読者が判断する必要（散文＝意図に従えば動く）。

6. **【表記ゆれ・複数箇所】`check(...) # ["文字列"]`（Part 2・5・7）と `type_of(parse(...)) # => 型`
   （Part 4・6）は、実戻り値（ハッシュ配列／3 引数）と形が違う。** 公開 API の形を一意に決める妨げ。

---

## 公開 API の形は本文から一意に決まったか

- **`check(source, baseline=[]) -> Array<{line:, [column:, length:,] message:}>`** … ほぼ一意。
  Part 1 で `{line:, message:}`、Part 1-4b で `column`/`length` を追加、Part 9 で第2引数 `baseline`。
  **戻り要素が「文字列」ではなく「ハッシュ」である点は、本文の `# ["..."]` 表記からは読み取れず、
  `check` の実装コード（`diagnostics` 配列）から逆算するしかない**。

- **`annotate(source) -> Array<{line:, type:}>`** … 一意。`type` は型オブジェクト（`to_s` で表示）。
  Part 8 で `def` 文のときだけ `type` が**シグネチャ文字列**になる分岐も明示。

- **`type_of`** … 内部 API。本文の `type_of(parse("..."))`（1 引数）と実装の
  `type_of(node, scope, diagnostics)`（3 引数）が食い違う。例示を擬似コードと割り切れば実装は一意。

- **`accepts(expected, actual) -> :yes|:no|:maybe`** … 一意。Part 7 で全文掲載。

総じて、**`check`/`annotate` の引数と戻り値の構造は本文だけで一意に決まる**（戻り要素がハッシュで
ある点だけ、例示の `# ["..."]` ではなく実装コードを見る必要がある）。

---

## 付録依存の事故チェック

本文には「付録 a1/a3/a5 へ」のポインタが多数あるが、いずれも**深掘り・対応表・他言語比較**であって、
**前編の実装に必須の手順が付録へ逃げている箇所は発見できなかった**。付録を開かずとも Part 9 まで
実装し切れ、期待出力 36/36 を再現できた。改稿（本文⇄付録の移動）に伴う「本文だけでは作れない」
事故は起きていない。

---

実装：`/tmp/chibirigor-repro-l2/lib/chibirigor.rb`（最終状態・約 230 行）。
