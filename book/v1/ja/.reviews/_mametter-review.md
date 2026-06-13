# mametter レビュー（前編 v1）／2026-06-13 L1真・dump_type/TypeProf 対比 監査

査読者：遠藤侑介（mametter）＝『型システムのしくみ』著者・TAPL 共訳・TypeProf 開発者の目線。
重点：dump_type（付録 a3-2）／type-of を低レベル API とした整理／引数の call-site 推論の不在（Part 8 §8-6・Part 0 コラム）。辛口で。軸（やさしい前編・gradual・脅かさない・カジュアル）は壊さない。framing 不可逆点（推論を土台にした型チェッカー／check は副産物ではない）は尊重。

---

## 総評（先に）

今回の dump_type 追加は、前編の「道具は 2 つ」という骨格を壊さずに `dump_type` を**式**として滑り込ませた処理が巧い。`:info` に乗せる説明も正しく、PHPStan の `dumpType()` を引くのは概念として妥当。TypeProf 開発者として一番うれしいのは、§8-6 と Part 0 コラムの引数 call-site 推論の扱いが、過去の懸念（TypeProf を「諦めるツール」扱いしていないか）を解消して、**設計選択の対称な対比**として書けている点。TypeProf を矮小化していない。`rigor/docs/handbook/appendix-typeprof.md` の公式対比とも軸がズレていない。

突くべき点は 3 つに集約される。(1) dump_type の「PHPStan と同じ発想」は正しいが、TypeProf 開発者として見ると**型を覗く道具の系譜で TypeProf 自身（`rigor type-of`／LSP hover／TypeProf の assert_type コメント）が引かれていない**のは、a3 という「実 Rigor の道具」付録の中ではやや片手落ち。(2) `dump_type` の実 Rigor 側の事実（`include Rigor::Testing` 要件）が一次資料と細部でズレる疑い。(3) 「hover は低レベル API」という整理は Rigor 内部では正確だが、読者が「型を覗く UX は Rigor 独自に整備されている」と読み違える余地が少しある（TypeProf/Steep/Sorbet も同じ UX を持つ）。いずれも軸を壊さず直せる範囲で、(2) 以外は据え置きも可。

---

## 指摘表

| # | 該当箇所 | 本書の記述 | ドメイン著者としての指摘 | 修正案 | 重大度（真の弱点 / 好み） |
|---|---|---|---|---|---|
| 1 | a3-2 L124 | 「`dump_type` は… `check` したときにその位置の推論型を `:info` 診断として印字します（PHPStan の `dumpType()` と同じ発想で…）」 | **概念対比としては公正**。PHPStan の `dumpType()` はまさに「式の位置に推論型を診断として吐く」道具で、Rigor の `dump_type` の発想元として正しい（実 Rigor 自身 CHANGELOG が "PHPStan-style typing helpers" と呼ぶ）。ただし TypeProf 開発者の目で見ると、**「型を覗く道具」の引き先が PHPStan 一択**なのが惜しい。Ruby 同梱の TypeProf にも `# assert_type:` コメントや LSP hover という同系の「型を覗く」手段があり、この付録は他でもないその TypeProf を Part 8/Part 0 で対比相手にしている。PHPStan に寄せること自体は不公正ではない（実 Rigor が命名対応で PHPStan に寄せているのは事実）が、片側だけ引いて TypeProf の同等手段に触れないのは、a3 という「実物の道具棚」では収まりが悪い。 | 「PHPStan の `dumpType()` と同じ発想」はそのまま残す。任意で半文「（TypeProf の型注釈コメントや LSP hover も同じ『式の型を覗く』系統）」を添えると、道具系譜が片寄って見えない。必須ではない。 | 好み（fairness の収まり） |
| 2 | a3-2 L122–123 | 「ソースに `dump_type(式)` と書いて `check` する手があります（`include Rigor::Testing` が要ります）。」 | **一次資料との細部のズレ疑い（要確認）**。実 Rigor の CHANGELOG（0.0.x）は API を `Rigor::Testing.dump_type` と記し、`dump_type`/`assert_type` ルールは call-site の `self_type` が `Rigor`/`Rigor::Testing` のとき抑制、と書く。一次資料が前面に出すのは `Testing.dump_type(value)` の**レシーバ付き**呼び出し。`include Rigor::Testing` して `dump_type(式)` を裸で呼ぶ書き味は実装上ありうるが、「`include` が要る」と**断定**するとレシーバ付きで使う実態とズレかねない。chibirigor 側が「`include` も要らず」と対比で強調しているぶん、実 Rigor 側の要件記述が甘いと対比の土台が崩れる。 | 一次資料（rigor 本体の Testing モジュール定義／handbook）で `dump_type` の正準の呼び出し形を確認し、「`include Rigor::Testing` で裸呼び」か「`Rigor::Testing.dump_type(式)`」かを実態に合わせる。曖昧なら「`Rigor::Testing` 経由で呼びます」と緩める。 | 真の弱点（事実精度・要一次確認） |
| 3 | a3-2 L126–128 | 「エディタの hover が見せる型も同じ推論ですが、そちらはツール向けの低レベル API が裏で支えていて、人が直接叩くコマンドではありません。」 | **Rigor 内部としては正確**（hover は `rigor type-of` のコア＝`Scope#type_of` を LSP が叩く構造で、一次資料 design/language-server と整合）。ただし「型を見るユーザー手段は annotate と dump_type、hover 等は低レベル API」という整理を**そのまま読むと**、型を覗く UX が Rigor 独自に設計されているかの含みが出る。TypeProf も `--lsp`/hover、Steep も LSP hover、Sorbet も `T.reveal_type`／LSP hover を持ち、「hover の裏に低レベル type-at-position API」という構造は**この界隈の標準形**であって Rigor 固有ではない。過度に独自と見せてはいないが、ひと言で標準だと分かると公正。 | 「人が直接叩くコマンドではありません」の後に任意で「（hover の裏に型問い合わせ API を置く作りは Steep・Sorbet・TypeProf でも同じ）」を半文。独自性の誤読を防ぐだけで、本文の正確さは保てる。必須ではない。 | 好み（誤読予防） |
| 4 | Part 8 §8-6 L254–259 / Part 0 L65–71 | 「TypeProf なら… `double(3)` のように呼ばれている場所を見つけて `n` を `Integer` まで逆算し… chibirigor（と Rigor）はあえてそれをしません… その方がスケールするし、誤検知も出ない」 | **TypeProf 開発者として満足。不当な矮小化はない**。「呼び出し元から逆算」は TypeProf の whole-program 抽象解釈の核を正しく言い当てており、それを「弱点ではなく設計選択」と両建てで書いているのも公正（`appendix-typeprof.md` の "the trade is real and intentional" と同趣旨）。唯一の微調整は「その方がスケールする」の含意 ― TypeProf 2/`--lsp` は incremental 化を進めているので、「全体実行は大きなコードベースで爆発しがち」（Part 0 L70）は **TypeProf 1 系の像**にやや寄る。前編はやさしさ優先で踏み込まなくてよいが、後編 Part 5 で扱うときは TypeProf 2 の改善に一言あると今日的。 | Part 8/Part 0 本文は変更不要。後編 Part 5（本物の推論）側で TypeProf 2 の incremental 化に半文触れると、前編コラムの「爆発しがち」像が古びない。前編は据え置き可。 | 好み（時制の鮮度・後編向け） |
| 5 | Part 8 §8-6 L251 / dump_type 追加の波及 | 「この `untyped` の出方そのものが…『推論が型を見失った場所』… Rigor の `sig-gen` の発想の芽」 | **dump_type 追加で文脈はズレていない**（確認結果）。§8-6 は「引数を untyped に倒す→sig-gen の芽」という縦糸で、a3-2 の dump_type は「型を覗く道具」という別軸。両者が干渉していないのは良い。TypeProf 開発者として一点だけ：§8-6 で「untyped＝人間が型を足すべき場所」と言うのは Rigor の robustness principle（引数は寛容）と整合するが、TypeProf は**逆に observed-narrow な引数を埋める**。前編はそこまで要らないが、ここが Part 0 コラムの「TypeProf は逆算する」と対になる ―「untyped を残す Rigor」vs「観測から埋める TypeProf」の対称が読者の頭で結べると美しい。現状の本文でも壊れてはいない。 | 変更不要。強いて言えば §8-6 の脚注で「TypeProf はこの untyped を観測から埋めてくる（=Part 0 コラムの逆算）」と一言結ぶと縦糸が締まる。任意。 | 好み（編まれ方） |
| 6 | a3-2 L97–98 / L113 / L326 | 「実 Rigor も `annotate`（本書と同じ名前の道具です）やエディタの hover で型を見せます」 | **事実確認 OK**。実 Rigor に `rigor annotate`（ソースに型コメントを付す）は実在（ADR-29 browser-playground、ADR-32 が `annotate` の戻り型表示に言及）。本書の `annotate` と同名で対応づけるのは正確。問題なし（褒め）。 | なし | （正確・指摘なし） |
| 7 | Part 0 L13・L43–57（framing） | 「chibirigor は、型を自分で推論する、最小限の Ruby 型チェッカー」「推論はチェックと別物の前段ではなく、チェックを成り立たせている土台」 | **「推論器」という語法の監査結果：本文は健全**。Part 0 は一貫して「推論を土台にした型チェッカー」と書き、「推論器」という名詞で道具をラベルしていない（「推論器」が出るのは Part 9 L205・seasoned P1 L15 の比喩的言及のみで、framing を侵していない）。TypeProf 開発者として、TypeProf を「推論ツール（inference）」、Rigor/chibirigor を「推論を土台にしたチェッカー」と書き分けているのは正確で公平 ― 両者の到達点（RBS 生成 vs 診断）の違いをラベルが反映している。不可逆 framing は守られている。 | なし | （framing 健全・指摘なし） |
| 8 | a3-2 全体（位置づけ） | 「chibirigor も `dump_type` を基本機能として持っています。道具は `check` と `annotate` の 2 つに絞ったまま ― これはコマンドではなく、`check` が認識する式」 | **学習体験としての位置づけは妥当**。「コマンドを増やさず、式として型を覗く」整理は、Part 0 の「道具は 2 つ」の約束を破らずに型確認の学習動作を足す、教育的に良い設計。`dump_type(x)` の型が `x` のままという説明（L143）も `:info` の非破壊性（exit 0）も正しい。TypeProf 開発者の目で見て、`reveal_type` 系（Sorbet `T.reveal_type`／PHPStan `dumpType`／mypy `reveal_type`）の系譜に正しく乗っている。 | なし（任意で #1 の系譜追記を兼ねられる） | （妥当・好み余地のみ） |

---

## 重点 3 問への結論

1. **dump_type の PHPStan 引用は公正か／学習体験の位置づけは妥当か**
   公正かつ妥当。`dumpType()` は発想元として正しく、`:info` 非破壊・式扱いの説明も正確。唯一、a3 という「実 Rigor の道具棚」の中で PHPStan だけ引いて TypeProf の同系手段（assert_type コメント・hover）に触れないのは収まりが惜しい（#1、好み）。事実誤りではない。

2. **type-of をユーザー非露出＝低レベル API とした整理は公正・正確か**
   Rigor 内部としては正確（hover＝`type-of` コアを LSP が叩く、で一次資料と整合）。ただし「hover の裏に type-at-position API」という構造は Steep/Sorbet/TypeProf 共通の標準形なので、Rigor 独自と読み違える余地を半文で潰すと公正（#3、好み）。過度に独自とは見せていない。

3. **引数推論の不在の対比は TypeProf を矮小化していないか／dump_type で文脈がズレていないか**
   矮小化していない。§8-6・Part 0 コラムとも「逆算は TypeProf の強み、ローカル維持は設計選択」の対称対比で、`appendix-typeprof.md` と軸が一致。dump_type 追加による文脈のズレもなし（別軸として独立、#5 で確認）。唯一、「全体実行は爆発しがち」が TypeProf 1 系の像にやや寄るので、後編 Part 5 で TypeProf 2 incremental 化に触れると鮮度が保てる（#4、好み・後編向け）。

## 唯一の「真の弱点」

**#2 の `include Rigor::Testing` 要件**だけは一次資料（CHANGELOG は `Rigor::Testing.dump_type` のレシーバ付きを前面に出す）と細部がズレる疑いがあり、要確認。chibirigor 側が「include も要らず」と対比で強調しているぶん、実 Rigor 側の要件が甘いと対比の土台が崩れる。ここだけ事実精度の問題。残りはすべて好み（fairness の収まり・誤読予防・時制の鮮度・編まれ方）で、前編の軸を一切壊さずに据え置きも可能。

## 自著（『しくみ』）引用の公正さ ― 監査結果

Part 0 L77–82・L102（「『しくみ』のチェッカーは型エラーで例外を投げて止まる／chibirigor は止まらない」）、L174–177（『しくみ』が扱わなかった領域＝chibirigor が先に進めた部分）。**公正**。『しくみ』を「型注釈付きミニ言語のチェッカーを作る入門書」と正しく要約し、TAPL のエッセンスを蒸留した位置づけ・日本語のみという但し書きも正確。「裏表の関係」という対比（注釈前提 vs 注釈なし推論土台）は誇張でも貶めでもなく、両書の設計差を素直に言い当てている。自著を持ち上げすぎても貶めすぎてもいない。
