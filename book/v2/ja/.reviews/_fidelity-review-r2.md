# Rigorフィデリティ査読r2（実Rigor 0.4.6反映後）

- 日付：2026-10-27
- 対象：`git diff 74f7783 72199bd -- book/v2/ja AGENTS.md docs/20260607-type-systems-distilled-rigor-mapping.md`（実Rigor 0.4.6に合わせた改訂箇所）。前後の文脈と、同じ主張が残っている未改訂箇所も確認した。
- 一次情報：`/Users/megurine/repo/ruby/rigor`（`lib/rigor/version.rb` = 0.4.6、HEAD `b8045ea53`）。読み取りのみ。
- 実行：`bundle exec`はvendor下のgemに拡張が無く起動できなかった（`Bundler::GemNotFound`）。代わりに`ruby -I/Users/megurine/repo/ruby/rigor/lib /Users/megurine/repo/ruby/rigor/exe/rigor …`で、チェックアウトのlib（0.4.6）とシステムのprism 1.9.0 / rbs 4.2.0を使って動かした。一時ファイルは`/tmp/fid2/`に置いた。
- 判定の基準：意図的な簡略化（chibirigor自身の挙動を述べる文）は対象外。「実RigorはX」と断言しているのに実態が ¬Xの箇所だけを挙げた。
- 区分：**[新]**は改訂で新たに入った誤り。**[残]**は改訂で直り切っていない箇所（未改訂の行に古い主張が残っている場合や、前提だけ直して結論を据え置いた場合）。

## 要対応（ERROR / MISLEADING）

| # | 箇所（ファイル:行と引用） | 実Rigorの実態（一次情報のパス・行または実行結果） | 判定 | 修正案 |
|---|---|---|---|---|
| 1 | `seasoned/part7-soundness.md:131-133` 「`# rbs_inline: enabled` / `# RBS側に書くか、rbs-inlineのコメントで.rbに書く（ADR-93）` / `#: %a{rigor:v1:assert x is non-empty-string} (String?) -> void`」 **[新]** | `#:`に書いたメソッド型そのもの（`(String?) -> void`）は読まれる。実際`D.new.a!(1)`に対して`expected String?, got 1`が出た。ところが`%a{…assert…}`のほうは効かない。`v = gets; E.new.a!(v); Rigor.dump_type(v)`の結果は`String?`のままだった（`/tmp/fid2/ri/lib/e.rb`）。同じassertを`# @rbs %a{rigor:v1:assert x is non-empty-string}`と`# @rbs x: String?`と`# @rbs return: void`の形で書くと、呼び出し側が`non-empty-string`に絞られた（`/tmp/fid2/ri/lib/a.rb:19`）。文書化されている形も`# @rbs %a{…}`だけ（`docs/CHANGELOG-0.3.x.md:585,591`、`docs/adr/111-inline-refinement-carrier.md:47-48`）。もう一点、ADR-93では`# rbs_inline: enabled`は不要（`docs/adr/93-default-rbs-inline-ingestion.md:24-27`「MUST NOT require `# rbs_inline: enabled`」）。 | ERROR | コード例を`# @rbs %a{rigor:v1:assert x is non-empty-string}` / `# @rbs x: String?` / `# @rbs return: void` / `def ensure_present!(x) = …`に差し替える。`# rbs_inline: enabled`の行は消す（残すなら「不要だが書いてもよい」と添える）。 |
| 2 | `seasoned/part4-recursive-types.md:167` 「`bound`の既定は`untyped`なので、上界を登録していない`App`の判定は結果として`:maybe`になります」。あわせて`:169`「上界が`untyped`のまま → `:maybe`」と、表の行`:175`「HKTの上界`bound`が既定の`untyped`のまま \| `:maybe`」 **[残]**（前提は「fuel」から「boundへの委譲」に直ったが、結論の`:maybe`が据え置かれた） | `Type::App#accepts`は`Inference::Acceptance.accepts(bound, other)`に処理を渡すだけ（`lib/rigor/type/app.rb:73-74`）。boundが`Dynamic[top]`のときは`accepts_dynamic`が**yes**を返す（`lib/rigor/inference/acceptance.rb:156-161`「gradual: Dynamic[T] accepts any concrete type」）。fuelが尽きたときも`app.bound`が返る（`lib/rigor/inference/hkt_reducer.rb:44-47`）。 | ERROR | 結論を「`bound`の既定は`untyped`なので、上界を登録していない`App`はgradualに素通しになります（`:yes`）」に改める。表のこの行は`:maybe`の表から外すか、「`:yes`（gradualの素通し）」として別扱いにする。 |
| 3 | 同じNOTEの表`seasoned/part4-recursive-types.md:173` 「`untyped`が混入している \| `:maybe`」と`:177` 「Rigorは『分からなければ黙って通す』を一貫して`:maybe`という単一の語で表現します」 **[残]**（今回のdiffの外だが、#2と同じ囲みの中にある） | 実Rigorの受理判定では、`Dynamic`がどちらの側にあっても**yes**になる（`acceptance.rb:37-39`「gradual: Dynamic[T] passes any boundary」、`:156-161`）。`maybe`は判断できない構造の組み合わせ（closed shapeとopen sourceの組など、`acceptance.rb:1058-1060`）に限って返る。「untyped → `:maybe`」はchibirigorのPart 7の簡略化であって、実Rigorの挙動ではない。 | ERROR | 主語を分ける。たとえば「chibirigorは三値の`:maybe`に寄せますが、実Rigorは`untyped`が関わる判定をgradualの`:yes`で素通しし、`:maybe`は形を確かめきれない場合に使います」とする。囲み全体の主張（「HKTの複雑さが三値に吸収される」）は、「`bound`に委ねるので三値の外へ漏れない」という形で残せる。 |
| 4 | `little/part8-rbs-and-signatures.md:232` 「opt-inの`parameter_inference:`設定や`sig-gen --params=observed`で呼び出し元の型を集める道はありますが、集めた型は精度を足すためだけに使い、引数の診断を厳しくする根拠にはしません」 **[新]** | `parameter_inference:`についてはこの通り。ADR-67がprecision-additiveと定め、境界での否定的な規則も止めている（`docs/adr/67-parameter-type-inference.md:1-12,226-229,293`、`lib/rigor/configuration.rb:98`の既定false）。一方`sig-gen --params=observed`は、観測した型のunionを**RBSの引数型として書き出す**（`docs/adr/14-rbs-sig-generation.md:245-252`）。ADR-14 WD2はこれを「parameter tightening」と呼び、利用者が選ぶものとしている（`:477-485`）。書き出したRBSを採用すれば、それ以降は宣言として引数境界の診断を生む。 | MISLEADING | 二つを分けて書く。「`parameter_inference:`は精度を足すだけで、引数の診断を厳しくしません。`sig-gen --params=observed`は観測した型をRBSの引数型として書き出す道具で、それを採用するかどうかは書き手が決めます」。 |
| 5 | `seasoned/part5-real-inference.md:177-181` 「`rigor sig-gen --params=observed`も、観測した呼び出しから引数型を集める同系統の手段です」に続けて「だからRigorは、集めた型を…引数の境界で呼び出しを拒む根拠にはしません」 **[新]** | #4と同じ（ADR-14:245-252, 477-485）。「拒む根拠にしない」が成り立つのはADR-67の`parameter_inference:`だけ。 | MISLEADING（軽） | sig-genの文に「ただしこちらは型をRBSとして書き出すので、採用すれば宣言になります」と一言足す。あるいは、「下限だから拒まない」の段落の主語を`parameter_inference:`に限定する。 |
| 6 | `seasoned/part3-generics-and-substitution.md:150` 「RBSで書けないリテラル種や**予算を超えたリテラルunion**は基底クラスへ」 **[新]** | 予算によるwidenはhash-shapeのキーと値のunionにしか無い（`docs/type-specification/rbs-erasure.md:96-107`、`hash_erasure_keys`=16 / `hash_erasure_values`=8）。一般の戻り値のunionは`Union#erase_to_rbs`がメンバを並べるだけ（`lib/rigor/type/union.rb:62-66`）。実際に、41個の整数リテラルからなるunionを返すメソッドを`sig-gen`にかけると、`(0 \| 1 \| … \| 40)`がそのまま出力された（`/tmp/fid2/t4.rb`）。`union_size`予算は配線されていない（`inference-budgets.md:85`）。前半の「RBSで書けないリテラル種」は正しい（`1.5` → `Float`、`rbs-erasure.md:11`）。 | MISLEADING（軽） | 「予算を超えた**ハッシュのキーや値の**リテラルunionは基底クラスへ」と範囲を限る。 |
| 7 | `seasoned/part5-real-inference.md:195` 「集めた能力を入れる構造的なキャリアが、まだ無いからです。前編Part 6の`HashShape`のような構造的な型が、ここで必要になります」 **[新]** | ADR-67によると、欠けているのはresponds-to集合を持つ**structural-interface / capabilityのキャリア**である。`hash_shape`は既にあるcarrierとして列挙されている（`docs/adr/67-parameter-type-inference.md:123-133`）。 | MISLEADING（軽） | 「`HashShape`がキーの集合を持つように、『応えるメソッドの集合』を持つ構造的なキャリアが、ここで必要になります」とする。HashShapeは類比として出し、HashShapeそのものが要るとは書かない。 |
| 8 | `seasoned/part4-recursive-types.md:157` 「なお呼び出し側に見えるのは還元後の形で、不透明な`App`そのものは表に出ません」 **[新]** | 外側の型は還元後のunionとして見える。ただし入れ子の再帰部分はAppの表記のまま残る。`Rigor.dump_type(JSON.parse(s))`の結果は`bool \| Array[json::value[String]] \| Float \| Hash[String, json::value[String]] \| Integer \| String \| nil`だった（`/tmp/fid2/t6.rb`）。`symbolize_names: true`のときは`json::value[Symbol]`、変数のときは`String`版になる。判別表の記述自体は正確（`lib/rigor/builtins/hkt_builtins.rb:162-167,199-210,274-296`）。 | MISLEADING（軽） | 「外側は還元後の形で見え、入れ子の再帰部分だけが`json::value[String]`のように適用の形で残ります」とする。直前のコード例`:148-153`の`# => App[…]`は概念表記なので、そのままでよい。 |
| 9 | `appendix/a3-tooling.md:261`（付録末の対応表）「`annotate`／hoverの裏で内部精密型とRBS境界型をerasureで丸める」 **[残]** | 同じ付録の改訂済み行`:81`／`:88`と食い違う。erasureが効くのは`sig-gen`の書き出しだけで、`annotate`・`type-of`・診断は内部の形を出す（`docs/type-specification/rbs-erasure.md:15`「diagnostic messages, `rigor type-of` and `rigor annotate` print the internal form」）。実行結果も同じで、`annotate`は`"FOO"`・`1.5`を出し、`sig-gen`は`"FOO"`・`Float`を出した（`/tmp/fid2/t3.rb`）。 | MISLEADING | 「`annotate`／hoverは内部の精密型を見せ、`sig-gen`が書くRBSでerasureにより丸める」と、`:88`に揃える。 |
| 10 | `appendix/a4-bibliography.md:106` 「ADR-20 \| 軽量HKT（還元fuelで再帰を止める。実装済み）」、`:110` 「ADR-32 \| inline RBS」 **[残]**（どちらも軽微） | 再帰を止める主な仕組みは、還元スタックによる循環検出（tying the knot）。fuelは外側の安全網にあたる（`lib/rigor/inference/hkt_reducer.rb:20-26,193-207`、`docs/adr/20-lightweight-hkt.md:27-29`）。inline RBSはADR-32のopt-inから、ADR-93で既定の取り込みに変わった。 | MISLEADING（軽） | `:106`を「（還元を循環検出で止め、fuelで打ち切る。実装済み）」にする。`:110`は「ADR-32、93 \| inline RBS（93で既定の取り込みに）」とする。本文Part 4・Part 7・Part 8に合わせる形。 |

## OK（44件）

以下の改訂箇所は一次情報または実行結果と一致した。根拠だけ記す。

| 箇所 | 根拠 |
|---|---|
| `AGENTS.md:138` `Integer[min..max]` | ADR-109:7,81-83。`int<a, b>`は非推奨の入力別名として残る |
| `a1:38-39`型の外に残す記録 | ADR-75:1-12（side-channel）、`expression_typer.rb:1403-1422`（`FallbackTracer`）、`internal-spec/inference-engine.md:26` |
| `a1:156` ADR-47の対象と既定 | ADR-47:3,5,9,31-34。実行でも`if false`はwarning `flow.unreachable-branch`、`when Symbol`はinfo `flow.unreachable-clause`、`is_a?`の死に枝は無報告（`/tmp/fid2/t1.rb`, `t7.rb`） |
| `a2:81-85`書き換え得る変数だけを無効化 | `internal-spec/inference-engine.md:1061`（読むだけの局所変数は絞り込みを保ち、書き込みがあれば`Dynamic[top]`へ落とす） |
| `a2:88` RBSではなく組み込みカタログで判定 | `closure_escape_analyzer.rb:29-33`（「RBS-blind」、将来はRBS::Extendedに置き換え） |
| `a2:151` ADR-109の非推奨別名 | ADR-109:81-83 |
| `a3:31` `--explain`の出力形式 | 実行結果`info: fail-soft fallback at Prism::CallNode: Dynamic[top] [fallback]`、`runner.rb:2817` |
| `a3:41-43` Dynamicの受信側には記録を足さない | 実行結果。untypedな`v`に対する`v.bar` / `v.baz.qux`は記録なし（`/tmp/fid2/t2.rb`）。`rigor trace`も実在（`rigor help`） |
| `a3:70` 2行目の扱いの差 | 同上 |
| `a3:78-88`内部型の表示とsig-genのerasure | `rbs-erasure.md:12,15`、実行結果（`t3.rb`） |
| `a3:96` `require "rigor/testing"`と`Rigor.dump_type` | `lib/rigor/testing.rb:22,38-57`、実行結果（`dump.type`のinfoが出る） |
| `a5:45-47` RBSレコードはclosed | `acceptance.rb:1028-1053`。実行で`expected { name: String }, got { name: "A", admin: true }` |
| `a5:65`実Rigorの既定報告と`is_a?`の除外 | ADR-47:3,31。実行結果 |
| `glossary:57` erasureの例 | `rbs-erasure.md:10-12` |
| `part0:48`全体実行をしない・既定では逆算しない | ADR-67:1-12、`configuration.rb:98` |
| `part1:189` `call.undefined-method`／`call.unresolved-toplevel` | 実行で、`"s".frobnicate`はerror `call.undefined-method`、`foo.bar`はwarning `call.unresolved-toplevel` |
| `part1:240` annotateは内部型で、丸めるのはsig-gen | `rbs-erasure.md:15` |
| `part2:186` | 同part1 |
| `part4:133` `call.possible-nil-receiver` | 実行でerror `possible nil receiver: '+' is undefined on NilClass` |
| `part5:123` `is_a?`の下でuntypedも絞る | 実行で`dump_type: String`（`t2.rb:8`）、ADR-83:3-7 |
| `part5:147`重大度（warning / info） | 実行結果、ADR-47:5 |
| `part5:199` `union_size`は未実装 | `inference-budgets.md:62,85` |
| `part6:117,155` | 同a5 |
| `part7:67`リテラル型の引数に`2`を渡すと報告 | 実行で`expected 1, got 2` |
| `part7:129`メンバが1つでも`:no`なら報告 | 実行で`expected Integer, got "a" \| 1` |
| `part9:40` `1 \| Dynamic[top]`を保つ | 実行で`dump_type: 1 \| Dynamic[top]`（`t5.rb`、`t2.rb`）、ADR-83:3-7,25-30 |
| `part9:55` | 同part5:123 / part6 |
| `s-part1:206` rbs-inlineの`# @rbs %a{…}`（ADR-93） | `CHANGELOG-0.3.x.md:591`、ADR-93。実行でも効いた（`ri/lib/a.rb`） |
| `s-part2:70` open / closedの幅部分型 | `acceptance.rb:1046-1060` |
| `s-part3:150`（#6以外の部分） | 実行で`"FOO"`・`1`・`:a`はリテラルのまま、`1.5`は`Float`、HashShapeはrecord |
| `s-part4:140`循環検出（ADR-20 D4） | ADR-20:27-29、`hkt_reducer.rb:20-26,193-207` |
| `s-part4:157`（判別表の部分） | `hkt_builtins.rb:162-167,206-210,288-296` |
| `s-part4:159`循環検出とfuel 64 | `hkt_reducer.rb:44`、ADR-20 WD3 |
| `s-part5:160` ADR-57、ADR-41は未配線 | ADR-57 Status、ADR-41 Status「Proposed」、`inference-budgets.md:85` |
| `s-part5:166,177-181`（ADR-67の部分） | ADR-67:1-12,219-229,293-301（既定off、下限、否定的な規則を止める） |
| `s-part5:195` WD2は保留 | ADR-67:6,123-138 |
| `s-part6:44-66` `dynamic_origin`バケツ／`Scope#dynamic_origins`（ADR-75） | `inference-engine.md:26,58,1061`、`scope.rb:293-296` |
| `s-part6:135` ADR-56の書き戻し | `inference-engine.md:1061`、ADR-56 Status |
| `s-part7:69` ADR-83でDynamicを絞る | ADR-83:3-7 |
| `s-part7:163`循環検出と、fuel切れでboundに戻る | `hkt_reducer.rb:44-62` |
| `s-part8:45` ADR-50の題と状態（Proposed） | ADR-50:1-12 |
| `s-part8:105-106` ADR-93 / ADR-4（FactStore）/ ADR-56 | ADR-4:292,324-326（FactStore導入） |
| `s-part8:132` `PRISM_DISPATCH` | `expression_typer.rb:84,244,288` |
| `docs/…rigor-mapping.md`の改訂行（ADR-24/57/84、変性は一律共変で宣言変性はSlice 5+、RBSの再帰alias、v0.3.7での暗黙のHKT登録、`App[:"json::value", [String]]`、保守的erasure、framing） | `acceptance.rb:536-537`、ADR-20 Status（slices 4-5 in v0.3.7）、`rbs-erasure.md:3`、実行結果（t6） |

## 総評

今回の改訂は、確認の重点に挙がった事実記述（診断ID・重大度・既定で有効かどうか、ADRの番号と内容、`--explain`の形式、erasureの規則、HashShapeのclosedレコード、`parameter_inference:`、HKTの循環検出とfuel、`Integer[1..10]`、`rigor/testing`、`PRISM_DISPATCH`、`Scope#dynamic_origins`、v0.3.7、一律共変）について、ほぼすべて実Rigor 0.4.6の一次情報と実際の出力に一致している。44件がOKだった。とくに、診断IDと重大度（error / warning / info）、`--explain`の1行の形式、`1 | Dynamic[top]`、untypedを`is_a?`で絞ること、closedレコードは、すべて実際に動かして確かめられた。

直すべきものは2系統ある。

1. **[新] ERRORが1件**：後編Part 7のassert例（#1）。`#: %a{…} (…) -> …`と書くとメソッド型は読まれるが、assertは黙って捨てられる。実行して確認した。読者がそのまま写すと動かない例なので、優先度は最も高い。正しい書き方は`# @rbs %a{…}`。
2. **[残] ERRORが2件**：後編Part 4の「HKT判定も三値」の囲み（#2, #3）。前提は「boundに委ねる」に正しく直ったが、結論の`:maybe`が旧稿のまま残った。実Rigorではboundの既定が`untyped`なので、判定はgradualの`:yes`で素通しになる。同じ囲みにある「untyped混入 → `:maybe`」「Rigorは一貫して`:maybe`」も、chibirigorの簡略化を実Rigorの性質として書いてしまっている。囲みの主語を分けて書き直すのがよい。

ほかはMISLEADINGで、多くは軽微である。目立つのは`sig-gen --params=observed`を「精度を足すだけ」の側に含めた点（#4, #5）で、これは書き出したRBSが宣言になる道具なので分けて書く必要がある。残りは、erasureの予算の範囲（#6）、WD2に欠けているキャリアの種類（#7）、還元後にAppが見える範囲（#8）、改訂済みの本文と付録の表がずれている点（#9, #10）。どれも一文の修正で済み、本書の軸（chibirigorは「黙る」側を取り出した簡略化で、実Rigorは「宣言や証明がある所では厳しい」）は崩れない。
