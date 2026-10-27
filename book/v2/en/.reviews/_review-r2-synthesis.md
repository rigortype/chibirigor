# 英語版 査読r2の統合（2026-10-27）

対象：b1e59ad..e6c11d0で英語版に入った差分（実Rigor 0.4.6反映と日本語版r2の同期分）。
回した観点は4つ：英日忠実性／FP・型理論研究者／技術書編集者（モダン＋ベテラン）／英文校正。

各観点の所見：`_fidelity-review-r2.md`／`_en-fp-researcher-review-r2.md`／`_en-editor-review-r2.md`／`_en-copyedit-review-r2.md`。

## 英語版で反映したもの

- **忠実性ERROR**：後編Part 7のrbs-inline assert例で`@rbs`が3行とも欠落していた（同期時のperl展開事故）。
  `# @rbs`を復元。`@`トークンの両版比較で他に欠落はない。
- **編集**：E1〜E6、I1〜I38（I10は前編Part 7の分のみ。I39は除く）、N1〜N10。
  同期で足した文の継ぎ目、カルク、但し書きの重複、見出し・まとめとの不整合。
- **校正**：E1〜E2、F1〜F7、NIT。

## 日本語版を先に直すべきもの（未反映・両版共通の内容）

英語版単独では直さない。正本の日本語版で直してから英語版へ同期する。

- **FP ERROR 1**（後編Part 7 §7-5の三値表の注）：「`untyped`が絡む検査では`:maybe`でも`:yes`でも診断は出ない」は実Rigorでは偽。
  実Rigorは実引数のUnionをメンバごとに判定する（`acceptance.rb` `accepts_union_other`：どれか`:no`なら`:no`）。
  `1 | Dynamic[Top]`を`String`の仮引数に渡すと`call.argument-type-mismatch`が出る。
  `:yes`になるのは実引数の側が丸ごと`Dynamic`のときだけ。前編Part 9 §9-1の記述（`1 | Dynamic[Top]`）とも食い違う。
- **FP ERROR 2**（後編Part 4のHKT注と表、§4-7まとめ）：`App#accepts`が`bound`に委ねるのは`App`が**期待側**のときだけ。
  `App`が実引数側だと`acceptance.rb`に特別扱いがなく、`String`の仮引数に`json::value[String]`を渡すと拒否される（再現済み）。
  「上界なしの`App`は`:yes`で通る」は期待側に限定して書く必要がある。
- **編集E7**（前編Part 7の86行目、182行目）：`:maybe`で黙る仕組みをRigorの手柄として語っている。主語をchibirigorにするか、「黙る」だけをRigorと共有する形に。
- **編集E8**（後編Part 4の表見出し「判定できない理由」）：実Rigorの列は判定している。FP ERROR 2と一緒に直す。
- **編集I39**（付録a2 a2-3の注）：`each`で外側ローカルを書き換えると`captured_local`を無効化するとだけ書くが、後編Part 6は実RigorがADR-56でブロック本体を再評価してjoinを書き戻すと述べる。
  a2の注を「最小モデルでは」と限定するか、ADR-56の書き戻しを添える。
- **FP MISLEADING**：
  - 後編Part 4の表の行「`untyped`が混じる→`:yes`」は「一方の側が丸ごと`untyped`」に限る。
  - 後編Part 7：未知キーを`nil`と断定すると進行だけでなく保存も破れうる（`h[:zzz] = 5; h[:zzz]`）。「保存は自明に保たれる」は強すぎる。
  - 前編Part 5：「`untyped`を`is_a?(String)`で絞っても誤検知は出ない」は誤り。`String`のサブクラスのメソッドで`call.undefined-method`が出る（再現済み）。
  - 後編Part 5：「1メソッドずつ・局所＋カタログ」は、呼び出しごとの評価（ADR-84）と並べると、呼び出し文脈に感応する（polyvariant）解析だと言う方が正確。TypeProfの単一の合流型（monovariant）との対比で書く。
  - 後編Part 3：反変から「だから`sig-gen`は`untyped`を書く」への接続が逆向きに読める。`untyped`がgradualな未知であって`Top`ではないから成り立つ。
- **差分の範囲外**：
  - 後編Part 5:80-83（ERROR）：`n + 1`の`n`は引数ではなくレシーバ。`N = Integer`は唯一解ではなく推測（`Float`も解）。
  - 後編Part 4:144-149：`seen`のスケッチを「TAPLより弱い」と言うが、既にTAPL図21-4の形をしている。

## 実Rigor側への申し送り（read-onlyのため記録のみ）

- `App`を実引数に渡すと`bound`を見ずに拒否される。期待側との非対称は、実Rigor自身の誤検知の可能性がある。
