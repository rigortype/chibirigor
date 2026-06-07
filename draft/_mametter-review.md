# mametter（遠藤侑介）視点の批評レビュー

> 『型システムのしくみ』著者かつ TypeProf 開発者として振る舞う独立サブエージェント（opus）に
> よる、TypeProf と Rigor の概念差を主軸にした批評。TypeProf＝whole-program 抽象解釈・呼び出し元
> から引数推論・RBS 生成が主目的／Rigor＝ローカル＋カタログ・引数 untyped・診断が主目的。
> 「すべて反映する必要はなく、軸はぶれないように」。

## 総評
chibivue 流を型ツールでやり切った企画を評価。Part 6 三値・Part 8 戻り型合成は『しくみ』の二択
`subtype` と地続きで gradual 側に踏み出していて良い。著者が自分の弱点（推論の空白）を正確に
自覚している点は信用できる。**ただし「推論の空白を埋める」と言う本が、Ruby 公式の推論ツール
TypeProf を完全に黙殺しているのが最大の引っかかり。**

## 疑問・批評ポイント

### 【重大】「推論器」という語法 ― 二重に問題
1. TypeProf 作者として：Ruby 圏で「推論器」の名に最も値するのは TypeProf（呼び出し元から
   引数型を逆算する）。chibirigor の「推論」は Part 8 で**引数＝untyped に倒す**＝synthesis のみ
   ＝推論の難しい半分（逆算）をやっていない。
2. Rigor 自身の整理と矛盾：`docs/handbook/appendix-typeprof.md` は「Rigor の主役は check（バグ
   検出）／TypeProf こそ推論（RBS 生成）が主目的」と整理。なのに入門書 Part 0 は「Rigor＝推論器、
   check は副産物」と**ほぼ逆の旗**を立てる。同一プロジェクト内で appendix と入門書がツールの
   自己定義を食い違わせている。
   - **提案**：「推論器 vs チェッカー」ではなく「**合成（synthesis）はやるが逆算（call-site
     inference）はやらない**」と切る。後編 Part 3 §3-4 で既に正しく書けているのだから前編でも堂々と。

### 【重大】TypeProf 不在 ― 注釈なし Ruby ツールの「もう一方の極」を地図から消している
Steep・Sorbet・TAPL・『しくみ』は出るのに TypeProf だけ一度も出ない。とくに惜しい 2 箇所：
- **Part 1 §1-2「丸め」**：TypeProf は `1`→`Integer` に widen、Rigor は `Const[1]` を保つ。本書の
  精度思想を最も鮮やかに見せられる対比（handbook の表が既にやっている）。
- **Part 8 §8-3「untyped＝推論の穴」**：TypeProf は引数を call-site から埋めるので、本書が諦めた
  `def double(n)` を `(Integer)->Integer` まで持っていける。「引数推論＝続編」と言うなら、
  「Ruby には動く別アプローチ（TypeProf）がある」と脚注一つで触れるのが誠実。
- **提案**：Part 0 か Part 8 に TypeProf を一段落／一脚注で。handbook appendix-typeprof.md を引けばよい。

### 【中】引数推論の不在を続編に丸投げした構造的帰結
前編単独完結を謳うと、読者は「synthesis 限定の推論」を「推論」と内面化したまま閉じる。後編
Part 3 §3-4 の「なぜ全部やらないか」（決定性・速度／誤検知／RBS 境界）は Rigor の判断として正しい
（whole-program の爆発を避ける＝TypeProf 作者も同意）。**前編 Part 8 にミニ版を前借り**すべき。

### 【中〜軽】『しくみ』の引用 ― 著者本人が検証して「公正・正確」
- 「完全一致で弾く（P5）」「NG で throw して止まる（P0/P1）」「union をあえて避けた（P4）」
  「健全でも良いプログラムを弾くのは誤検知（P6 コラム）」── すべて**正確**。曲解なし。
- 細部のみ：(a) Part 4 で「union＝TAPL 11.10 バリアントが最近縁」とするが、私が避けた「一般の
  合併型」と TAPL のタグ付きバリアントは別物。地の文でやや圧縮。(b) 後編 Part 3 が TAPL 22 章を
  「HM の骨子」とするが let 多相は 23 章由来（既に `_expert-review-findings` が指摘）。(c)「薄い本」
  が二度（著者は苦笑）。
- mapping.md の章対応表は独立した読み物として優秀。本体に組み込んでもよい。

### 【軽】後編 Part 1 §1-3「合成 ⇒ は決して失敗しない」
一般定理に見える書き方。chibirigor 固有の全域化である旨を限定（既に `_expert-review-findings` MISLEADING #1）。

## 興味を引く点（mametter が「面白い」と評価）
- (a) **リテラル型 `Const[1]` を保つ精度思想**：TypeProf は最初から Integer に widen（抽象解釈の
  状態爆発を避けるため）。Rigor が局所合成＋境界カタログだから `Const[1]` を保てる、という因果が
  `1 + 2` の素朴な「丸め」から実 Rigor まで伸びている。whole-program 派が持たないカード。
- (b) **「診断は照合 `⇐` 位置でしか出ない」の形式化（後編 P1 §1-3）**：標語を subsumption の
  `S<:T` が崩れる瞬間に回収し、「①合成が untyped に全域化＋②照合が untyped を罰しない」の二段で
  "never frighten" を型付け規則の構造的帰結へ昇格。「論文の導入に使える」。
- (c) **「わざと unsound」を progress 放棄として表に（後編 P7 §7-4）**：TAPL 8.3 を gradual ツールの
  設計選択に読み替えた好例。「preservation はほぼ保てる／手放すのは主に progress」の切り分けも正確。
- (d) **chibivue 流を型ツールで**：`type_of`/`accepts`/`Scope` が実 Rigor の三点の最小版になり、
  「最小版を手で作れば本物のソースが同じ骨格として読める」出口が機能。mapping §12 の方法論的自覚。

## TypeProf 開発者としての一言（要約）
良い本。だから TypeProf を一段落でいいから登場させてほしい。黙殺は本の品質に見合わない。
(1) Part 0/Part 8 に脚注一つ（handbook appendix を引く）。(2) Part 1「丸め」に TypeProf を対置。
(3)「推論器」の看板を「局所合成ベースの誤検知を出さない診断器」へ正直に。引数を埋めないのは
弱点でなく whole-program の爆発を避けてスケールと沈黙を買う Rigor の核心トレードオフ。
『しくみ』の引用は公正だった ── その目で Ruby 公式の推論ツールを地図から消すのだけはやめてほしい。
