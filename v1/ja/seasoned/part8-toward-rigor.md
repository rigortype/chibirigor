---
title: "Part 8（最終章）― 本物の Rigor へ"
description: "二巻を締める章。chibirigor と本物の Rigor の対応を整理し、次に読むべき ADR への道を示す。"
sidebar:
  order: 18
---

# The Seasoned chibirigor Part 8 ― 本物の Rigor へ（最終章）

> 参考書（任意）：Rigor の ADR 群・spec（型理論の教科書には対応章なし ― ここは「学習用最小版」
> から「実用ツール」への橋）。後編、そしてチュートリアル全体を締める章です。

ここまで、前編で *動く*最小版を作り、後編でその裏にある理論（双方向・部分型と変性・ジェネリクス・
再帰型・推論・FactStore・健全性）を回収しました。最後に、chibirigor の最小版と、本物の
**Rigor** の*作り込み*の間に橋を架けます。「ここから先、実用ツールは何を足すのか」を見渡して
締めます。

---

## 8-1. 私たちが作ったもの／作らなかったもの

chibirigor（前編）は、Prism ＋ 数百行で、こう積み上げました：

| 部品 | 前編での最小版 | Rigor の本物 |
|---|---|---|
| 式の型付け | `type_of`（合成 `⇒`） | `ExpressionTyper`（純粋・非破壊） |
| 受理判定 | `accepts`（三値） | `accepts`＋`AcceptsResult`（三値＋理由） |
| 型環境 | `Scope`（Hash） | `Scope`＋`FactStore`（6 バケツ・stability） |
| メソッド解決 | 手書き表→ミニ RBS | `MethodDispatcher`（多段ティア・RBS・継承） |
| 型キャリア | 7 種 | 多数（`Refined`/`Intersection`/`App`/…） |

最小版は「**骨格を忠実に、肉は最小限**」でした。骨格（双方向・gradual・フロー・RBS 境界）は
本物と同じ。ここから実用ツールになるには、何が要るか。

---

## 8-2. 実用ツールが足すもの

理論ではなく**工学**の層です。chibirigor は意図的に切り捨てました。ここからは
「**読むなら、どの ADR から**」を具体的な次の一歩として示します（ADR ＝ Rigor の設計判断記録。
リポジトリの `docs/adr/`）：

| 足すもの | 何を解く工学か | 入口の ADR |
|---|---|---|
| **プラグイン機構** | Rails・RSpec・dry-rb の DSL/マクロ/open class を、コアを汚さず外から型付け | ADR-2（拡張 API）→ ADR-37（インターフェース分離）→ ADR-9（プラグイン間連携） |
| **マクロ／DSL 展開** | `attr_accessor` や `Data.define` 等の生成メソッドを型付け | ADR-16（マクロ展開）／ADR-48（`Data`/`Struct` 折りたたみ） |
| **キャッシュ／インクリメンタル** | 大規模コードを毎回全解析しない | ADR-6（永続キャッシュ）→ ADR-45（unchanged 高速パス）→ ADR-46（増分解析） |
| **LSP（エディタ統合）** | 型エラーのリアルタイム表示・補完・ホバー | ADR-19（Language Server 同梱） |
| **CI 連携** | SARIF/GitHub/GitLab 等、各 CI が読める出力 | ADR-51（CI 出力形式）／ADR-27（配布） |
| **baseline／オンボーディング** | 既存コードへの後付け導入（前編 Part 9 の baseline の本式） | ADR-22（baseline ＋ オンボーディング） |
| **性能** | アロケーション削減・並行解析で実用的な速さ | ADR-44（アロケーション）／ADR-15（Ractor/fork）／ADR-50（性能ゲート） |

これらは「**型システムそのもの**」ではなく「型システムを*応用*した機能」です（『しくみ』も
おわりにで、エディタ支援は型システムの特性ではなく応用だと釘を刺しています）。chibirigor が
これらを非対象にしたのは、最小版の主旨を守るためでした。読み進める順は、**興味のある行の
ADR を 1 本開く**のがいちばん早い ― どれも「なぜそう設計したか」が物語として書かれています。

---

### 8-2-a 発展ノート ― 極小プラグインフック（`Chibirigor.register_method`）

上の表の「プラグイン機構」だけを chibirigor に**最小実装**してみましょう。
本物の Rigor ADR-2 が「拡張 API をコアから分離する」という設計判断をした理由が、
腹落ちするはずです。

**何をするか**　`Dispatch::METHODS` は `Rbs.load` で生成される読み取り専用のカタログです。
ここに直接書き込むと、テスト間の干渉・ライブラリ固有の型情報の混入など、
_コアを汚す_ 問題が起きます。代わりに、**外から合成するレジストリを 1 つ**用意します。

```ruby
Chibirigor.register_method(
  :String, :shout,
  params:  [],
  returns: Chibirigor::Type::Nominal[:String]
)

Chibirigor.annotate('"hello".shout')
#=> 最後の式の型は String（登録した戻り型が効いている）
```

**仕組み（`lib/chibirigor/plugin.rb` 抜粋）**

```ruby
module Plugin
  @registry = {}

  module_function

  def register_method(klass, name, params:, returns:)
    @registry[[klass, name]] = { params: Array(params), returns: returns }
  end

  def registry = @registry
  def reset! = @registry.clear
end
```

`Dispatch#dispatch` は `Plugin.registry[key] || METHODS[key]` の順で参照します。
これだけで「プラグインが優先、コアは不変」が成り立ちます。

**FP ゼロの保証**　`Plugin.registry` にないメソッドは従来通り `METHODS` に委ねます。
`METHODS` にもなければ `Dynamic`（untyped）に倒れる ― 前編の「知らなければ静かに untyped」
という約束はここでも守られます。

**実 Rigor との対比**　chibirigor の「1 点のレジストリ」は、Rigor ADR-2 の拡張 API の
骨格に対応します。本物は DSL・ライフサイクル・プラグイン間連携（ADR-9・37）が加わりますが、
「コアのカタログとプラグイン由来の情報を分けて持つ」という設計軸は同じです。

---

## 8-3. 設計判断は ADR に残る

実用ツールは、無数の**設計判断**の積み重ねです。Rigor はそれを **ADR（Architecture Decision
Record）** として記録します ― 「なぜ open class を許すか」「なぜ Ractor でなく fork か」
「なぜ baseline をこう設計したか」。

chibirigor を作り終えたいま、Rigor の ADR を読むと、その多くが**前編・後編で出会った緊張**の
具体的な解決だと分かります：

- **まず全体像をつかむなら** ADR-0（基盤・設計原則の出発点）→ ADR-4（型推論エンジンの仕組み）。
  この 2 本が、あとの全 ADR の地図です。
- 「誤検知をどう避けるか」← 前編 Part 4・6、後編 Part 7（わざと unsound）。**読むなら**
  ADR-5（ロバストネス原則）／ADR-22（baseline）。
- 「再帰・推論をどう止めるか」← 後編 Part 4（fuel）・Part 7（予算）。**読むなら**
  ADR-20（軽量 HKT）／ADR-41（推論予算の設計・未実装）。
- 「RBS とどう付き合うか」← 前編 Part 8、後編 Part 3（erasure）。**読むなら**
  ADR-14（sig-gen）／ADR-25・32（プラグイン RBS・inline RBS）。
- 「フロー事実をどう持つか」← 後編 Part 6（FactStore）。**読むなら**
  ADR-46（増分依存グラフ）と内部仕様 `inference-engine.md`。

理論で「なぜそう作るか」が腑に落ちた目で実装を読むと、ADR の一つ一つが*物語*として読めます。

---

## 8-4. ここまでの全体像

チュートリアルの二巻を、一枚に：

- **前編 The Little chibirigor** … 動く最小版。Prism ＋ 数百行で、check と annotate が動くまで。
  易しさ・複雑さ予算・FP 規律を、最小実装で*体感*する。
- **後編 The Seasoned chibirigor** … その裏の理論。双方向・部分型と変性・ジェネリクス・再帰型・
  推論・FactStore・健全性を、用語と形式で*回収*する。

そして本物の Rigor は、この骨格に、プラグイン・キャッシュ・LSP・性能・ADR という*作り込み*を
重ねた、実用規模の漸進的型チェッカーです。

---

## 8-5. 締め ― 三つの入口

最後に、この旅の地図を：

1. **『しくみ』／TAPL の終着点が、chibirigor の出発点。** 静的で健全なチェッカーの*その先* ―
   gradual typing から、私たちは始めました（必須ではないが、併読すると地続きに読めます）。
2. **chibirigor の出発点が、Rigor への入口。** 最小版で骨格を手で作ったいま、本物の Rigor の
   実装は「拡大した同じ骨格」として読めます。
3. **そして Rigor の入口は、Ruby の型付けという、まだ続く道の入口。** 漸進的型付けは研究も実装も
   現在進行形です。

ここまで作り切ったなら ― あとは、本物の Rigor のコードを開いて、`ExpressionTyper` の最初の
`case` を読んでみてください。前編で書いた `type_of` の、見慣れた形がそこにあるはずです。

---

## 8-6. まとめ（後編 全 8 章）

| Part | 回収した理論 |
|---|---|
| 1 | 双方向型付け（`type_of`＝`⇒`／`accepts`＝`⇐`、診断は照合位置のみ） |
| 2 | 部分型と変性（width/depth、戻り共変・引数反変、gradual consistency） |
| 3 | ジェネリクスと型代入（α 同値・変数捕獲・fresh・erasure） |
| 4 | 再帰型（μ・余帰納 ↔ HKT/fuel。HKT の根拠は TAPL 29 章） |
| 5 | 本物の型推論（capability/duck・制約ベース、自明な範囲だけ） |
| 6 | 完全な FactStore（6 バケツ・stability・クロージャ捕獲・join） |
| 7 | 健全性・正規化、そして「わざと unsound」＋gradual guarantee |
| 8 | 実用ツールへの橋（プラグイン・キャッシュ・LSP・性能・ADR） |

## 演習

1. **次の ADR を 1 本選ぶ**：§8-2 の表から、自分がいちばん気になる行を 1 つ選び、その入口 ADR を
   開いて「どの設計判断が、後編のどの章の緊張に対応するか」を 1 つ書き出せ。
2. **骨格の対応づけ**：§8-1 の対応表（chibirigor ↔ Rigor）から 1 組選び、前編で自分が書いた
   コード（例：`accepts`）が、本物の Rigor で何という部品に育っているかを述べよ。
3. **地図を描き直す**：この二巻で学んだことを「①前編で作った ②後編で読み解いた ③Rigor で
   作り込まれている」の 3 段で、自分の言葉で 3 行にまとめよ。

---

**The Little chibirigor** で動かし、**The Seasoned chibirigor** で腑に落ち、そして **Rigor** へ。
ここで地図は閉じます。
