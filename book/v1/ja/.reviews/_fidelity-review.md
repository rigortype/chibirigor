# Rigor フィデリティレビュー（前編 v1）／2026-06-13 L1真・dump_type/type-of 監査

実 Rigor チェックアウト（`/Users/megurine/repo/ruby/rigor`・読み取り専用）と本書（前編）の
「実 Rigor では…」という**事実記述**だけを突き合わせた。意図的な簡略化は乖離としない。
判定は ERROR（矛盾断言）／MISLEADING（条件付きでしか正しくない）／要確認／OK／先行（著者方針）。

総括：**事故的乖離は 1 件のみ（dump_type の include 必須という断定＝MISLEADING）**。
他は今セッションの重点 3 項目を含め、ほぼすべて実態と一致。type-of の扱いは「乖離」ではなく
**むしろ本書が実 Rigor manual より保守的（先行・著者方針）**。重点 3 の「二段構え／erasure」は、
実 Rigor の hover / type-of が *実際に* type と erased を並べて見せるため、本書はむしろ控えめで、
**過剰主張は無い**（OK）。

---

## 重点 1 ― `dump_type`（a3-2 / 付録）

| 本書の主張 | Rigor 実態（出典） | 判定 |
|---|---|---|
| 実 Rigor の `Rigor::Testing.dump_type(式)` を `check` すると、その位置の推論型を `:info` 診断で印字 | `lib/rigor/analysis/check_rules.rb:1086-1103` `dump_type_diagnostic` が `message: "dump_type: …"`, `severity: :info` で発火。`testing.rb:9-12` も「`:info`-severity diagnostic showing the inferred type」と明記 | **OK** |
| PHPStan の `dumpType()` 相当 | `rule_catalog.rb:163` summary「PHPStan-style」／`check_rules.rb:1077` コメント「PHPStan-style `dump_type(value)`」 | **OK** |
| 実行時は値をそのまま返す（素通し） | `testing.rb:45-47` `def dump_type(value); value; end` | **OK** |
| `:info` なので診断は赤くならない／終了コードを汚さない | `check_rules.rb:1083` 「does NOT count toward `Result#error_count`」。`rule_catalog.rb:172-173` `severity_authored: :info`（ただし `strict` profile では `:error` に上がる ― 本書は profile 未言及で問題なし） | **OK** |
| 「`include Rigor::Testing` が要ります」（a3-2 §121-122 の括弧書き） | 実 Rigor は `include` 無しでも `Rigor.dump_type(x)` / `Rigor::Testing.dump_type(x)` が発火（`testing.rb:34-37,54-64`「without an `include` line」、`check_rules.rb:1142` `RIGOR_TESTING_RECEIVERS = ["Rigor","Rigor::Testing","Testing"]`、`1163-1173` レシーバ nil か上記定数なら発火）。`include` が要るのは**裸の `dump_type(x)`（暗黙 self）形のみ** | **MISLEADING** |

修正案（重点 1 の唯一の事故的乖離）：a3-2 §121-122 の
「ソースに `dump_type(式)` と書いて `check` する手があります（`include Rigor::Testing` が要ります）」を、
たとえば
「ソースに `dump_type(式)` と書いて `check` する手があります（裸の `dump_type(式)` には
`include Rigor::Testing` が要りますが、`Rigor.dump_type(式)` と完全修飾すれば include 無しでも引けます）」
程度に。やさしい文体を壊さず一文の括弧追記で済む。**断定が逆ではないので ERROR ではない**が、
「要ります」と必須に読める点だけ直すと完全に一致する。

補足（OK・再確認）：本書 a3-2 が引く実 Rigor の dump_type 例は無し（quote しているのは chibirigor 側の
出力のみ）なので、message 文字列レベルの矛盾は生じていない。実 Rigor の実メッセージは
`dump_type: <describe(:short)>`（manual 05 では `dump.type — Constant<3>` という別表記の例もあるが、
本書はこの文字列を断定していない）。

---

## 重点 2 ― `type-of` の位置づけ（a3-2 / Part1 / README）

| 本書の主張 | Rigor 実態（出典） | 判定 |
|---|---|---|
| `rigor type-of` をユーザー向けコマンドとして**提示しない**（道具は `check` と `annotate`／`dump_type`） | README.md:78・little/README.md:11 は `check`/`annotate` のみ。a3-2 §117 も「道具を `check` と `annotate` の 2 つに絞る」 | **OK（先行＝著者方針）** |
| 型を見たいユーザー手段は `annotate` と `dump_type`。hover 等ツールは低レベル API が裏で支える（a3-2 §126-127「ツール向けの低レベル API が裏で支えていて、人が直接叩くコマンドではありません」） | 実 Rigor では `rigor type-of` は **manual に載るユーザーコマンド**（`docs/manual/02-cli-reference.md:76`、`05-inspecting-types.md:66-78` ＋「Which to reach for」表 §105 に「One expression's type, from the shell → `rigor type-of`」と明記）。同時に `type_of_command.rb:21` は「thin probe over `Scope#type_of`」、`09-editor-integration.md:31,42` 「同じ binary が hover に使われる」、`05:77-78` 「same query the editor integration answers on hover」、MCP は `rigor_type_of`→`rigor type-of --format json`（`10-mcp-server.md:34,233-246`）。つまり type-of は (a) ユーザー CLI でもあり (b) hover/MCP の裏でも使われる | **先行（著者方針）** ― 乖離ではない |

所見：本書は type-of を「人が直接叩くコマンドではない＝低レベル API」と整理しているが、実 Rigor manual は
**現状 type-of をユーザーコマンドとして掲載している**（しかも「from the shell」と推奨表に載せている）。
プロンプトの注記どおり、これは**本書が manual より保守的に先行**している状態であり、事故的不正確さではない。
ただし a3-2 §126-127 の「ツール向けの低レベル API が裏で支えていて、人が直接叩くコマンドではありません」は、
**実 Rigor の type-of に当てはめると事実に反する**（type-of は人が叩けるし manual もそう載せる）。本書が
「hover が見せる型の裏には低レベル API がある」と一般論で書いている限りは OK だが、もし読者が
「実 Rigor にユーザーが型を 1 点だけ見るコマンドは無い」と読むと誤解しうる。

修正案（任意・低優先）：先行を壊さず誤読だけ防ぐなら、a3-2 §126-127 を
「エディタの hover が見せる型も同じ推論です（実 Rigor ではこれを `type-of` という低レベル probe が
裏で支えます ― 本書ではユーザー道具を `check`/`annotate`/`dump_type` に絞り、type-of は表に出しません）」
のように「実物には type-of がある／本書はあえて出さない」という*著者方針の明示*に寄せると、
先行であることが読者にも伝わり、誤読も消える。**現状でも ERROR ではない**ため必須ではない。

---

## 重点 3 ― 二段構え／erasure（a3-2・Part1 コラム・glossary）

| 本書の主張 | Rigor 実態（出典） | 判定 |
|---|---|---|
| Rigor は内部で精密な型を持ち、RBS 境界で粗い型に erasure で丸める | `docs/type-specification/rbs-erasure.md:1-19`「RBS erasure converts an internal Rigor type to a valid RBS type … MAY collapse refinements, literal unions, shapes … MUST NOT produce a narrower type」。実装上も `Type#erase_to_rbs` が存在 | **OK** |
| `annotate` が（内部の）型を見せる／実 Rigor の annotate は内部精密・境界粗の二重構造（Part1 §239-241, a3-2 §97-119） | `lib/rigor/cli/annotate_command.rb:159` は `type.describe(:short)`（＝内部型）1 つを `#=>` で見せる。`05-inspecting-types.md:44-64` も同様。erasure は別途 sig-gen 等で発生 | **OK** |
| hover も型を見せる（a3-2 §98,113／glossary §71-75） | `lib/rigor/language_server/hover_renderer.rb` が型を描画。`docs/manual/09-editor-integration.md:22`「textDocument/hover … receiver type + RBS signature / narrowed type / canonical refinement names」 | **OK** |
| 「erasure は Java の実行時型消去とは別物（静的に境界で精度を丸める）」（a3-2 §107-109・glossary §71-75） | `rbs-erasure.md` 全体・glossary の説明と整合。Java type erasure との対比も妥当（実 Rigor 文書は実行時消去を一切主張していない） | **OK** |
| 「2 つ並べて見せる」式の過剰主張が**残っていないか** | 本書は annotate を「内部型 1 つ」、hover を「型を見せる」と書くにとどめ、**並べて見せるとは言っていない**。実態はむしろ逆で、実 Rigor の `type-of`（`type_of_renderer.rb:25-26,49-50` `type:` ＋ `erased:`）と hover の一部（`hover_renderer.rb:277-278` `# Type` ＋ `# Erased`、`304-305` 既定 fallback も `type:`＋`erased:`）は **実際に内部型と erased 型を 2 段で並べて表示する**。よって本書は過剰どころか控えめ | **OK（過剰主張なし）** |

所見：重点 3 は問題なし。むしろ「実 Rigor は内部型と erased を *並べて見せることがある*（type-of・literal hover）」
という事実を本書が拾っていないだけで、これは簡略化の範囲。erasure の語義・Java 対比・二段構えの説明は
すべて一次情報と一致。**修正不要**。

---

## その他（軽い再点検 ― 既裏取り分は「OK（再確認）」）

| 本書の該当箇所 | 本書の主張 | Rigor 実態（出典＋行） | 判定 |
|---|---|---|---|
| a3-3 dispatch 5 段カスケード | ① 定数畳み込み → ② shape → ③ RBS → ④ in-source → ⑤ fallback(`Dynamic[Top]`)、上から fall through、③ が ④ に勝つ | `method_dispatcher.rb:41-58` Tiers in order「1 ConstantFolding / 2 ShapeDispatch / 3 RbsDispatch」＋ §54「ShapeDispatch runs *above* RbsDispatch」。`104-194` で RBS の下に in-source / user-class fallback、最後は `nil`→caller が `Dynamic[Top]` に widen（§88）。「user-authored RBS overrides …」も §164-167 と一致。実 dispatcher は string/data/math/time 等の fold モジュールが多数あるが、いずれも定数畳み込み族の細分で、5 段の抽象は妥当な簡略化 | **OK（再確認）** |
| a3-1 `rigor check --explain` | fail-soft 地点を `:info` 診断で地図化 | `lib/rigor/cli/check_command.rb:347` `opts.on("--explain", "Surface fail-soft fallback events as :info diagnostics")`。**`rigor explain <rule>` は別物**（rule カタログ表示・`explain_command.rb:11-20`）だが本書は `check --explain` を指しており取り違え無し | **OK** |
| a1-3 / a5-5 unreachable arm | 到達不能な分岐を検出 | `lib/rigor/analysis/check_rules/unreachable_clause_collector.rb`・`always_truthy_condition_collector.rb` 実在 | **OK（再確認）** |
| part4 4-1x Union レシーバ分配 | inference-engine.md 引用 | `docs/internal-spec/inference-engine.md` 実在。Union をメンバへ分配して畳む発想は a3-3b trace 例（§283「Union をメンバへ分配」）とも整合 | **OK（再確認）** |
| baseline / ADR-22 | baseline によるオンボーディング | `docs/adr/22-baseline-and-project-onboarding.md` 実在、`check_command.rb:360-368` に `--baseline` / `--no-baseline` / `--baseline-strict` フラグ | **OK（再確認）** |
| FactStore（glossary §40-41） | フロー感応な事実置き場 | 前編は素朴版・後編 P6 完全版という整理。実 Rigor 側 `flow_contribution.rb` 等にフロー事実機構あり。本書は「素朴版」と明示しており乖離なし | **OK** |
| a3-3b `rigor trace` | `bind`/`union`/`dispatch` をコマ送り、`--verbose`/`--format=json`/`--delay`/`--line` | `method_dispatcher.rb:87-94` `FlowTracer.dispatch` フック実在（dispatch イベント記録）。`05-inspecting-types.md:80-99` が trace の 3 種イベント・`--verbose`・`--format=json`・`--delay` を記載。本書は実物のみ `--line` と注記（§313）しており一致 | **OK（再確認）** |
| Part1 §237 / glossary §75 erasure は sig-gen で使われる | sig-gen が erasure を使う | `lib/rigor/cli/sig_gen_command.rb` が `erase_to_rbs` を使用（grep 一致） | **OK** |

---

## 総評

- 今セッションの最重点 3 項目（dump_type / type-of / 二段構え・erasure）は、**事実記述として実 Rigor と
  ほぼ完全に一致**。dump_type の `:info`・値素通し・PHPStan 相当・診断が赤くならない、はすべて
  `testing.rb`・`check_rules.rb`・`rule_catalog.rb` で裏取り済み。
- **唯一の事故的乖離**は a3-2 の「`include Rigor::Testing` が要ります」という**必須断定**（MISLEADING）。
  実物は `Rigor.dump_type(式)` 完全修飾なら include 不要。括弧一文の追記で解消できる（断定の向きが
  逆ではないので ERROR ではない）。
- `type-of` は**乖離ではなく先行（著者方針）**。実 Rigor manual は現状 type-of をユーザーコマンドとして
  掲載しているが、本書はあえて `check`/`annotate`/`dump_type` に絞っている。誤読防止のため、a3-2 §126-127 を
  「実物には type-of があるが本書は出さない」という*方針の明示*に寄せると親切（任意・低優先）。
- 重点 3 の「2 つ並べて見せる過剰主張」は**残っていない**。むしろ実 Rigor の type-of / literal hover は
  内部型と erased を*実際に並べて見せる*ため、本書はやや控えめ。erasure の語義・Java 対比も正確。
- その他の Rigor 主張（5 段カスケード・`check --explain`・unreachable arm・Union 分配・baseline/ADR-22・
  FactStore・trace）はいずれも一次情報と一致。**OK（再確認）**。
- やさしい前編の軸（gradual・脅かさない・カジュアル文体）を壊す記述は無し。提案修正はいずれも一文の
  追記/言い換えで、TAPL 並みの厳密化を強いるものではない。
