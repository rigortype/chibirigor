# L1 真・Rigorフィデリティ 査読（2026-06-09）

査読観点：本書の「実 Rigor では…／Rigor の中では…」という**事実記述**だけを、
Rigor チェックアウト（`/Users/megurine/repo/ruby/rigor/`、read-only）の一次情報と
突き合わせて一致／不一致を判定する。型理論の正しさ・日本語・読み味は別レンズ担当。

判定基準：**実 Rigor が X なのに本書が（簡略化の断りなく）¬X と断言**している
*事故的不正確さ*のみを「乖離(ERROR)」とする。各章「続編／付録に送ったもの」に
記録した意図的簡略化は乖離としない。

一次情報：`docs/type-specification/`・`docs/internal-spec/`・`docs/adr/`・
`docs/handbook/`・`docs/manual/`・`lib/rigor/`。

---

## 重大度サマリ

- **乖離(ERROR)**：3 件
  1. `fall-soft` ← 実 Rigor の正式語は **`fail-soft`**（用語の事故的誤記。本書内でも不統一）
  2. `int<m, n>` を Rigor の組み込み refinement 名として列挙 ← 実 Rigor は**明示的に拒否**し
     `Integer[1..10]`／`IntegerRange` を使う
  3. a4-4 で **ADR-46 を「フロー事実をどう持つか」「後編 Part 6（FactStore）」** に紐付け ←
     ADR-46 は**増分依存グラフ（cross-file incremental）**であり FactStore とは無関係
- **要確認**：2 件（a2-3 のエスケープ例 `Enumerator`／`Proc#curry`、a2-6「二層構成」の語）
- **OK（簡略化／検証済み一致）**：多数。とくに `Dynamic[Top]` 表記・6 バケツ名・dispatch 段順・
  `erasure`・Difference/Refined/IntegerRange 三分・ADR 番号と Status・baseline キー・
  正規表現キャプチャ・unreachable arm/ADR-47 は一次情報と一致。

---

## 乖離(ERROR)

| # | 本書の記述（原文ママ） | 一次情報での実態（出所） | 判定 | 修正案 |
|---|---|---|---|---|
| 1 | a3-1 見出し「`rigor check --explain` ― **fall-soft** の地図を出す」／本文・表・a1-1・part9 でも一貫して「**fall-soft**」 | 実 Rigor の語は **`fail-soft`**（`docs/manual/02-cli-reference.md:42`「`--explain` Surface **fail-soft** fallback events as info diagnostics」、`docs/internal-spec/inference-engine.md` で `fail-soft` 多数。リポジトリ全体で `fail-soft` 135 件、`fall-soft` 0 件）。さらに本書内でも seasoned/part5-real-inference.md:241 だけは正しく「**fail-soft**」と書いており**不統一** | **ERROR**（用語の事故的誤記） | `fall-soft`→`fail-soft` に統一置換。該当：a3-tooling.md（見出し含む 6 箇所）、a1-special-types.md:45、little/part9:102、_reorg-proposal.md:181。`--explain` が fail-soft fallback を `:info` で出す挙動の説明自体は正しい |
| 2 | a2-6 脚注「`positive-int`/**`int<m,n>`** のような範囲整数は `IntegerRange` で表します」／a2-6 表に Rigor 列「**`int<m, n>`**｜範囲指定の整数」 | 実 Rigor は `int<m,n>` を**組み込み名として拒否**：「Integer ranges use Rigor's range notation, such as `Integer[1..10]`. PHPStan-style `int<1, 10>` MUST NOT be added as an alias initially」（`docs/type-specification/imported-built-in-types.md:18`）。範囲整数の Rigor 表記は `Integer[1..]`／`IntegerRange`（`rigor-extensions.md:12`、ADR-3:67・112） | **ERROR**（Rigor 名でないものを Rigor 名として断言） | 表の Rigor 列を `Integer[1..n]`（または `IntegerRange`）に。`int<m, n>` は **PHPStan 列のみ**に置く（PHPStan 側の名称としては正しい）。脚注の例も `positive-int`/`Integer[m..n]` に |
| 3 | a4-4 表「ADR-46｜増分依存グラフ（**フロー事実をどう持つか**）｜後編 **Part 6（FactStore）**」 | ADR-46 は「**Incremental analysis via a cross-file dependency graph**」（`docs/adr/46-incremental-dependency-graph.md:1`）。内容は `rigor check --incremental` の cross-file 増分再解析で、FactStore／フロー事実とは無関係。FactStore の一次情報は `docs/internal-spec/inference-engine.md`（専用 ADR は無い） | **ERROR**（ADR の主題の取り違え） | 「フロー事実をどう持つか」「後編 Part 6（FactStore）」の対応を削除。ADR-46 は「増分依存グラフ＝cross-file 増分再解析（`--incremental`）」と説明し、後編で増分／キャッシュに触れる箇所（Part 8 §8-2 の工学 ADR 群）に紐付ける。FactStore に対応 ADR は無い旨を明記してよい |

---

## 要確認

| # | 本書の記述（原文ママ） | 一次情報での実態（出所） | 判定 | 備考 |
|---|---|---|---|---|
| 4 | a2-3「対象パターン：`Thread.new`・`define_method`・**`Proc#curry`・`Enumerator`** など『ブロックをオブジェクトとして保存／後で呼ぶ』パターン」 | 実 Rigor `ClosureEscapeAnalyzer` の `:escaping` カタログは `Module/Class#define_method`・`Thread.new/start`・`fork`・`Fiber.new`・`Proc.new`（`docs/internal-spec/inference-engine.md:504`）。**`Enumerator` は `:non_escaping` に分類**されており、本書が escaping 例に挙げるのは Rigor の分類と逆。`Proc#curry` はカタログ外（`:unknown` 扱い）。`Thread.new`・`define_method` は一致 | **要確認**（後編予習の例示だが、`Enumerator` は実分類と矛盾） | `Enumerator` を例から外す（実 Rigor では非エスケープ）。`Proc#curry`→`Proc.new`／`Fiber.new` に差し替えると一次情報と整合。なお「迷ったら消す（`:unknown` を `:escaping` と同等に保守的に扱う）」方針は spec と一致 |
| 5 | a2-6「実 Rigor は**二層構成**（ADR-3）で…（Difference／Refined／IntegerRange の 3 キャリアを列挙）」 | ADR-3 の Working Decision は「Option C（**two-tier** hybrid: point-removal `Difference`, predicate-subset `Refined`）」（`docs/adr/3-type-representation.md:105`）。IntegerRange は「structurally a Difference against the complementary half-line」で**専用 carrier として併存**（同:112）。つまり「二層（点除去／述語部分集合）」は正しいが、本文は carrier を**3 つ**列挙しており「二層」と数が合わない | **要確認**（nitpick 寄り。骨格は正しい） | 「二層（点除去＝`Difference`／述語部分集合＝`Refined`）。範囲整数は構造的には Difference の特例だが専用 carrier `IntegerRange` を併用」と書けば数の食い違いが消える |

---

## OK（検証して一致を確認した主な事実記述）

| 本書の記述 | 一次情報での実態（出所） | 判定 |
|---|---|---|
| `untyped` の内部表記 = **`Dynamic[Top]`**（`Top` に `Dynamic` マーカー）。a1-1・a3 各所・part2:234 | 実装形は `Dynamic[Top]`（`lib/rigor/type/top.rb` の `Rigor::Type::Top`、リポジトリ全体で `Dynamic[Top]` 303 件＞`Dynamic[top]` 194 件で優勢）。type-spec は surface 名を小文字 `top`/`Dynamic[top]` で書くが、実装・内部表記は `Top`。`Dynamic[T]` は「dynamic 境界を越えた」＋「static facet `T`」の 2 事実の合成（`special-types.md:36`）。本書の「`Top`＋`Dynamic` マーカー」「どこで黙ったか構造に残る」は spec の provenance 説明と一致 | **OK** |
| FactStore の **6 バケツ**：`local_binding`／`captured_local`／`object_content`／`global_storage`／`relational`／`dynamic_origin`（a2 冒頭） | 「recognised buckets are `local_binding`, `captured_local`, `object_content`, `global_storage`, `dynamic_origin`, and `relational`」（`docs/internal-spec/inference-engine.md:56`）。6 個・名称すべて一致 | **OK** |
| dispatch カスケードの**段の相対順序**：定数畳み込み → shape → RBS → in-source → fallback（a3-3・part2:221） | 実ディスパッチャは tier 順「precise tiers（constant-folding, shape-aware, …）→ plugin → HKT/static-refinement → **RBS** → macro → patched → dependency-source → **discovered-method(cross-file user def＝in-source)** → ancestor fallback」（`docs/internal-spec/inference-engine.md:156`）。本書の 5 段は多数 tier の**圧縮**だが、定数畳み込み/ shape が RBS より前・in-source が RBS の後・最後に degrade、という骨格順序は一致。各 part が「全貌は付録」「続編に送る」と断っており**意図的簡略化** | **OK（簡略化）** |
| ③ RBS が ④ in-source に勝つ＝宣言が本体推論に優先（a3-3） | 「The first tier that returns a non-`nil` wins」、RBS tier が discovered-method tier より前（同:156）。一致 | **OK** |
| `rigor type-of FILE:LINE:COL` で位置指定の推論型を引く（a3-2・part1:239） | `rigor type-of FILE:LINE:COL`（`docs/manual/02-cli-reference.md:85`、`05-inspecting-types.md`）。一致。※「内部精密型＋境界の保守型の **2 つを並べて見せる**」は manual に明記が無く（type-of は単一型表示・hover と同等）、本書独自の説明。ただし erasure による二重構造の存在自体は事実 → 「2 つ並べて表示」は要確認寄りだが付録の橋渡し説明として許容範囲 | **OK（要確認の含み）** |
| 精密内部型→RBS の粗い型に落とす境界操作を **`erasure`** と呼ぶ（a3-2） | 「RBS **erasure** converts an internal Rigor type to a valid RBS type … MAY collapse refinements, literal unions, shapes, dynamic-origin provenance」（`docs/type-specification/rbs-erasure.md:1-3`）。Java の実行時型消去とは別、という注記も spec の「Rigor→RBS は not lossless」と整合 | **OK** |
| refinement carrier の三分：点除去＝`Difference`／述語部分集合＝`Refined`／範囲整数＝`IntegerRange`（a2-6 脚注） | ADR-3 Option C：`non-empty-string`=`Difference[String,""]`、`lowercase-string`=`Refined[String,:lowercase]`、`numeric-string`=`Refined[String,:numeric]`、`positive-int` 等は `IntegerRange`（`docs/adr/3-type-representation.md:107-117`、v0.0.4 実装済み:132）。マッピングは完全一致（数の数え方のみ #5 で要確認） | **OK** |
| `non-empty-string`／`numeric-string`／`literal-string`／`positive-int`／`negative-int`／`non-zero-int`／`non-negative-int`／`non-empty-array`／**`non-empty-hash`**／`lowercase-string`／`uppercase-string`（a2-6 表 Rigor 列） | すべて実在の組み込み名（`imported-built-in-types.md:24-42,63`、`non-empty-hash[K,V]`=`Hash[K,V]-{}` は ADR-3:111・132 で実装確認）。PHPStan 語彙対応も一致。※`int<m,n>` のみ #2 で誤り | **OK**（`int<m,n>` を除く） |
| 正規表現の名前付きキャプチャがマッチ成功側でローカルに **`String`** 事実を足し、`if` 外は `String\|nil`（a2-2） | 「captured locals are bound to `String \| nil` after the match, and narrowed to `String` in the truthy branch」（`docs/handbook/03-narrowing.md:218-221`、実装 `lib/rigor/builtins/regex_refinement.rb`）。`decimal-int-string` への更なる絞りは follow-up なので「String」が現行挙動として正しい | **OK** |
| エスケープ検知でブロック捕獲変数の narrowing を**保守的に無効化**、「迷ったら消す」（a2-3） | `ClosureEscapeAnalyzer` が `:escaping`/`:unknown` で捕獲 outer local を `Dynamic[Top]` に落とし `local_binding` 事実を無効化、`:unknown` は `:escaping` と同等に保守的扱い（`inference-engine.md:504-505`）。方針一致。※具体例 `Enumerator`/`Proc#curry` のみ #4 で要確認 | **OK**（例示を除く） |
| 再代入が `local_binding` 事実をリセット、メソッド呼び出しは対象の `object_content` を疑う（a2-5） | 再代入は `local_binding` 無効化（`inference-engine.md:505` の `with_local` が local_binding 事実を invalidate）。stability/バケツ別無効化は spec の `stability` フィールドと整合 | **OK** |
| `void`＝⊤の別名・「値は返るが見るな」・`-> nil` と違い BC break にならない（a1-2） | 「`void` is a result marker for expressions whose return value should not be used」「RBS treats `void`, `boolish`, and `top` equivalently」（`special-types.md:66-70`）。value context で `void`→`top` に materialize（同:89）。本書の格子位置・契約説明と整合 | **OK** |
| `never`／⊥ の Rigor 名は **`bot`**（内部 `Bot`）、`Bot <: T`、到達不能/`raise` 経路（a1-3） | 「`bot` is the empty type … unreachable branches, methods that always raise, exits, failed pattern matches, contradictory refinements」「`T \| bot = T`」（`special-types.md:13-19`）。`never`/`NoReturn` は **alias にしない**方針（`imported-built-in-types.md:17,98`）＝本書が Rigor 名を `bot` とするのは正しい | **OK** |
| ボトム型を**型として作らず診断で扱う**＝unreachable arm（**ADR-47**）（a1-3） | ADR-47「Narrowing-driven clause reachability（`flow.unreachable-clause`）」。subject が `Type::Bot` に narrowing された clause を dead として `:info`/`:warning` 報告（`docs/adr/47-narrowing-driven-clause-reachability.md:1-5`）。番号・主題とも一致 | **OK** |
| unreachable arm（到達不能）と Java/C# の missing arm（網羅性）は**逆向き**、Rigor は動くコードに黙る（a5-5） | ADR-47 は exhaustiveness（missing arm）を**追わない**と明記し、unreachable のみ報告（同:58-62）。FP envelope（gradual/Dynamic は除外、loop/block 内は skip）も「動くコードを脅かさない」価値観と一致。※ADR-47 の着想元は Elixir v1.20。本書 a5-5 は Java/C# を対比に使うが ADR 番号は出さないので問題なし | **OK** |
| baseline はデフォルトで**ルール ID** で照合・**行番号はキーに含めない**・列も見ない（ADR-22）（part9:96） | 「rule-ID by default」「A rule-ID row defines bucket `(file, rule)`」（`docs/adr/22-…:124,156`、`keyed on (file, rule)`:338）。行も列もキー外、で一致。ADR-22 Status「fully implemented across v0.1.7–v0.1.9」 | **OK** |
| ADR 番号と Status：ADR-0 基盤・設計原則／ADR-4 推論エンジン／ADR-5 ロバストネス／ADR-20 軽量 HKT（**実装済み**）／ADR-41 推論予算（**Proposed・未実装**）／ADR-14 sig-gen／ADR-25 プラグイン RBS／ADR-32 inline RBS（a4-4） | ADR-0「Foundation and Core Architecture」/ADR-4「型推論エンジン」/ADR-5「Robustness」/ADR-20 Status「Accepted (partial implementation)」（fuel 還元は実装済み→「実装済み」許容）/ADR-41 Status「**Proposed, 2026-06-03**」（本書「Proposed・未実装」と一致）/ADR-14「rbs-sig-generation」/ADR-25「plugin-contributed-rbs」/ADR-32「rbs-inline-comment-ingestion」。すべて一致 | **OK** |
| `Constant<3>` のリテラル精度（part2:234） | `dump_type(1 + 2) # → Constant<3>`（`docs/manual/05-inspecting-types.md:19`、`assert_type("Constant<3>", 1+2)`:35）。表示形と一致 | **OK** |
| 構造的ハッシュ型の系譜 Hack `shape(...)` → PHPStan `array{...}` → Rigor（open 採用）（a5-3） | Rigor は Python `TypedDict` の語彙（required/optional/read-only/open-closed）を採用し PHPStan からも import（`imported-built-in-types.md:3,70`）。Hack→PHPStan→Rigor の系譜・open 採用の方向性は spec と矛盾なし | **OK** |

---

## 総評

付録 a1–a5 と本文 Part 0–9 の「実 Rigor」言及は、**設計の骨格・用語・ADR 対応の大半が
一次情報と正確に一致**している。とくに `Dynamic[Top]`・FactStore 6 バケツ・dispatch 段の
相対順序・`erasure`・Difference/Refined/IntegerRange の三分・正規表現キャプチャ narrowing・
unreachable arm/ADR-47・baseline キー・各 ADR の番号と Status は、突き合わせても破綻が無い。
直近改修で増えた付録群の精度は高い。

要修正は事故的不正確さ 3 件に集約される：

1. **`fall-soft`→`fail-soft`**（最重要・最も明白）。実 Rigor は一貫して `fail-soft` であり、
   本書内でも seasoned/part5 だけが正しく `fail-soft` と書いていて不統一。機械的な置換で解消。
2. **`int<m, n>` を Rigor 名として列挙**。実 Rigor は `int<m,n>` を**明示的に拒否**して
   `Integer[1..n]`／`IntegerRange` を使う。PHPStan 列に限定すれば正しい。
3. **a4-4 の ADR-46 紐付け**。ADR-46 は cross-file 増分解析であり FactStore とは無関係。
   主題の取り違えなので説明文と対応 Part を差し替える。

要確認 2 件（a2-3 のエスケープ例 `Enumerator`/`Proc#curry`、a2-6「二層」の語数）は
骨格は正しく、例示・語の精度を一次情報に寄せれば解消する nitpick 寄り。いずれも本書の
論旨・コード挙動には影響しない。

---

# L1 真・Rigorフィデリティ 査読・第2回（2026-06-10／対象＝今回*追加された*発展ノート）

査読観点は第1回と同じ（本書の「実 Rigor では…／Rigor の中では…」という**事実記述**だけを
一次情報と突き合わせる。意図的簡略化は乖離としない）。**対象は今回追加された発展ノートに
限定**する：

- 付録 a1 §a1-3x（ADR-47＝`flow.unreachable-clause`／FP envelope）
- 付録 a3 §a3-1x（`--explain` が*あらゆる* fail-soft を一覧化）・§a3-2x（type-of 2 段・erasure・dispatch）
- 付録 a2 §a2-6x（`non-empty-array` が `first` を非 nil に絞る／`unless arr.empty?` から生成・無効化）
- 後編 part5 §5-6／5-6x（周辺の「Rigor の中では」記述との整合）

一次情報：`/Users/megurine/repo/ruby/rigor/`（read-only）。

## 重大度サマリ（第2回）

- **乖離(ERROR)**：0 件。
- **要確認**：1 件（§a2-6x の `[0]` ― `first` は spec の動機例で裏付くが、`[0]` の非 nil 化は
  一次情報に明示が無い。骨格に影響しない nitpick）。
- **OK（簡略化／検証済み一致）**：今回の主張はすべて一次情報と一致。とくに 4 つの裏取り依頼
  （ADR-47 正式名・FP envelope／`non-empty-array#first`／§a3-1x の「あらゆる fail-soft」／
  `Dynamic[Top]`・erasure・dispatch）はいずれも spec・実装・ADR で裏付き。
- 第1回の ERROR #1（`fall-soft`）・#2（`int<m,n>`）は**本文で既に是正済み**を確認（a3 は
  `fail-soft` に統一、a2-6 表の `int<m, n>` は PHPStan 列のみ）。

## 要確認（第2回）

| # | 本書の記述（原文ママ） | 一次情報での実態（出所） | 判定 | 備考 |
|---|---|---|---|---|
| A | §a2-6x「実 Rigor の `non-empty-array` リファインメントが `first`（**または `[0]`**）を `Elem`（非 nil）に絞る」 | `non-empty-array[T]` の意味は「`Array[T]` with at least one element」＝`Difference[Array[T], Tuple[]]`（`docs/type-specification/imported-built-in-types.md:63`・`docs/handbook/02-everyday-types.md:219`・`lib/rigor/type/difference.rb:17,137`）。`first` が**非 nil** になることは spec の動機例「'Array' is not enough to prove `arr.first.upcase` is safe; `non-empty-array[String]` is」で裏付く（`02-everyday-types.md:44-48`）。spec が明文化する projection は `size`/`length`/`count`→`positive-int`（`control-flow-analysis.md:69`）で、**`[0]` の添字読みを非 nil 化する明文規則は見当たらない**（`first` の非 nil 化は carrier 意味論からの含意、`[0]` も同様だが個別の文書化は無し） | **要確認**（nitpick） | `first` は OK。`[0]` は括弧での添え物・「効きは同じ」の例示であり骨格に影響しないが、厳密には一次情報に明示が無い。気になるなら `（または `[0]`）` を外し `first` 一本にすると確実。残すなら「`first`（や添字読み）」程度の含み表現で十分 |

## OK（今回・検証して一致を確認した主な事実記述）

| 本書の記述 | 一次情報での実態（出所） | 判定 |
|---|---|---|
| §a1-3x「実 Rigor の **ADR-47**（`flow.unreachable-clause`）の縮図。実物は narrowing が subject を `bot` に絞った節を dead と判定」 | ADR-47 タイトル「Narrowing-driven clause reachability (`flow.unreachable-clause`)」、「a clause is unreachable exactly when its computed `body_scope` narrows the subject to `bot`」（`docs/adr/47-…:1,3`、README 索引 ADR-47 行、`47-…:74-83`）。番号・正式名・主題すべて一致 | **OK** |
| §a1-3x「ループ・ブロック・gradual(Dynamic) を除外する FP envelope を持つ」 | ADR-47 の FP envelope：「Skip inside loops / blocks」「never on `Dynamic[T]` … the gradual guarantee holds」「Fire on `Dynamic[T]` subjects — rejected」（`47-…:122,128,187-190`；WD1 でも「never `Dynamic`（gradual guarantee）… clauses inside loops/blocks skipped」`47-…:5`）。**ループ・ブロック・gradual の 3 除外がそのまま一致** | **OK** |
| §a1-3x「chibirigor は閉じた既知型限定・葉クラスの互いに素・opt-in で最小化」（実物の縮図という位置づけ） | 実 Rigor も「concrete narrowed carrier を要求、`Dynamic` では撃たない」＝閉じた型限定の発想（`47-…:126-128`）。本書は最小版だと明記しており意図的簡略化。実物との対応づけは正確 | **OK（簡略化）** |
| §a2-6x「実 Rigor の `non-empty-array` リファインメントが `first` を `Elem`（非 nil）に絞る」（`first` 部分） | `non-empty-array[T]` は実在の組み込み refinement（point-removal `Difference[Array[T], Tuple[]]`、`docs/adr/3-type-representation.md:110,132`・`imported-built-in-types.md:63`・`lib/rigor/type/difference.rb:17`・CHANGELOG `[0.0.4]`）。`first` が非 nil で読めることは handbook の動機例で明示（`02-everyday-types.md:44-48`）。`Tuple` 由来の chibirigor 版と「効きは同じ・出自は違う」とする本書の整理は正確 | **OK** |
| §a2-6x「実 Rigor は `unless arr.empty?` のようなフロー事実から `non-empty-array` carrier を*生成*し、再代入やエスケープで*消す*」 | `Array#empty?`（偽辺）/`any?`（真辺）/`none?`（偽辺）が `Array[T]`→`non-empty-array[T]` に narrow（`docs/type-specification/control-flow-analysis.md:69`、`docs/handbook/03-narrowing.md:199`、ROADMAP:201）。handbook 各言語付録も「Rigor produces it from `unless arr.empty?`」と明記（`appendix-java-csharp.md:314` ほか）。再代入・エスケープでの無効化は FactStore の一般則（第1回 a2-5/a2-3 で確認済み）と整合 | **OK** |
| §a3-1x「実 Rigor は `Dynamic[Top]` の `Dynamic` マーカーを構造に保持して*あらゆる* fail-soft（RBS 不足・未注釈引数・プラグイン未設定…）を一覧化」 | `--explain`＝「Surface **fail-soft** fallback events as `info` diagnostics」（`docs/manual/02-cli-reference.md:42`）。fail-soft policy は「every fail-soft fallback … MUST be recorded into the tracer」（`docs/internal-spec/inference-engine.md:79`）＝*あらゆる* fallback を記録、で一致。挙げた 3 例（RBS 不足・未注釈引数・プラグイン未設定）はいずれも実在の fail-soft 経路（RBS ミス→`391`、未注釈 param 既定 `Dynamic[Top]`→`391`、プラグイン未解決→`305`）。※ただし **Dynamic *レシーバ*が opaque call を伝播する分は「fail-soft ではない」として tracer に*記録しない*** 明示例外あり（`inference-engine.md:218`）。本書は「fail-soft を一覧化」と限定して書いており、この例外は対象外なので**過大主張ではない** | **OK** |
| §a3-2x「type-of は内部精密型＋境界保守型の2段／境界の操作を **erasure** と呼ぶ」 | 「RBS **erasure** converts an internal Rigor type to a valid RBS type … MAY collapse refinements, literal unions, shapes」（`docs/type-specification/rbs-erasure.md:1-3`）。type-of の「2 段表示」は manual に明示が無く本書独自の橋渡し説明だが、erasure による二重構造の存在自体は事実（第1回でも「要確認の含みあるが許容」判定）。今回の §a3-2x も同じ整理で、過大主張なし | **OK（要確認の含み・第1回で既出）** |
| §a3-3「dispatch 5 段カスケード（定数畳み込み→shape→RBS→in-source→fallback）」「③ RBS が ④ in-source に勝つ」 | 実ディスパッチャの tier 順（precise tiers→…→RBS→…→discovered-method→ancestor fallback、「first tier that returns non-`nil` wins」）と骨格順序が一致（`docs/internal-spec/inference-engine.md:156`）。多数 tier の圧縮は本書が明記する意図的簡略化。第1回で確認済みの事実が §a1-3x/§a3 でも保たれている | **OK（簡略化）** |
| part5 §5-6/5-6x「ブロックの戻り型から型変数を解く」「解けない型変数は `Dynamic[Top]` に degrade」「capability role `_ToS`/`_Each[T]`」 | ジェネリクス具体化・HKT/capability-role は ADR-20（軽量 HKT、Status: partial implementation）の射程で、ブロック経由の型変数束縛・未解決時の `Dynamic[Top]` degrade は fail-soft policy（`inference-engine.md:63-79`）と整合。今回追加の付録 §a1-3x/§a2-6x/§a3 と**矛盾しない**（part5 は `fail-soft` 綴りも正しい） | **OK** |

## 総評（第2回）

今回*追加された*発展ノートの「実 Rigor」事実記述は、**事故的不正確さ（乖離 ERROR）ゼロ**。
裏取り依頼の 4 点はいずれも一次情報で裏付いた：

1. **ADR-47** ― 正式名 `flow.unreachable-clause`・主題（subject を `bot` に絞った節を dead 判定）・
   **FP envelope の「ループ／ブロック／gradual(Dynamic) 除外」** がそのまま ADR 本文と一致
   （`docs/adr/47-…:1,3,5,74-83,122,128,187-190`）。
2. **`non-empty-array`** ― 実在の組み込み point-removal refinement で、`first` の非 nil 化は
   spec の動機例で裏付き、`unless arr.empty?`（`empty?`/`any?`/`none?`）からの生成も明文（
   `control-flow-analysis.md:69`・`03-narrowing.md:199`・handbook 各付録）。再代入・エスケープでの
   無効化も FactStore 一般則と整合。
3. **§a3-1x の「*あらゆる* fail-soft を一覧化」は過大主張でない** ― `--explain` が fail-soft
   fallback を `:info` 化（`manual/02-cli-reference.md:42`）、かつ engine は「every fail-soft
   fallback … MUST be recorded」（`inference-engine.md:79`）。本書が「fail-soft」に限定して書く
   ため、Dynamic レシーバ伝播の非記録例外（`:218`）にも抵触しない。
4. **`Dynamic[Top]`・erasure・dispatch カスケード** ― 第1回の確認済み事実と整合、矛盾なし。

唯一の **要確認は §a2-6x の `[0]`**（`first` は OK、`[0]` の非 nil 化は一次情報に明示が無い
含意レベル）で、括弧内の添え物ゆえ骨格・コード挙動に影響しない nitpick。意図的簡略化
（chibirigor は `Tuple` の副産物で `non-empty-array` carrier 自体は続編送り、等）はいずれも
各ノートに断りがあり **OK**。第1回の ERROR #1/#2 が本文で是正済みであることも併せて確認した。
