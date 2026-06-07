# 用語集

本文で「実はこれ◯◯と呼ばれます」と後出しした用語を、引けるようにまとめます。
（前編＝The Little、後編＝The Seasoned。初出の章を併記。）

## 型と値

- **型キャリア（type carrier）**〔前編 P1〕… 型を表す Ruby オブジェクト。`Const`/`Nominal`/
  `Dynamic`/`Union`/`HashShape`/`Tuple` など。
- **`Const`（リテラル型）**〔前編 P1〕… 「その値そのもの」を表す型。例：`Const[1]`。
- **`Nominal`（名前的型）**〔前編 P1〕… 名前付きクラスを表す型。例：`Nominal[:Integer]`。
- **`Dynamic`／untyped**〔前編 P1〕… 「型を見失った」印。gradual の要。
- **`Union`（合併型）**〔前編 P4〕… 「`A` か `B` のどちらか」。例：`Integer | String`。
- **`HashShape`（レコード型）**〔前編 P5〕… キーごとの型を覚えるハッシュの型。
- **丸め／正規化（normalization）**〔前編 P1〕… 細かい型（`Const[3]`）を大ざっぱな型
  （`Integer`）に戻すこと。TAPL 12 章。

## 推論と検査

- **合成（synthesize, `⇒`）**〔後編 P1〕… 式から型を上向きに**求める**こと。前編の `type_of`。
- **照合（check, `⇐`）**〔後編 P1〕… 期待型に対して式が**合うか確かめる**こと。前編の `accepts`。
- **双方向型付け（bidirectional typing）**〔後編 P1〕… 合成と照合の 2 方向に分ける枠組み。
- **三値（trinary）`:yes`/`:no`/`:maybe`**〔前編 P6〕… 受理判定の答え。`:maybe` は罰しない。
- **ナローイング（narrowing／絞り込み）**〔前編 P4〕… 条件分岐の枝ごとに変数の型を狭めること。
- **ディスパッチ（dispatch）**〔前編 P2〕… レシーバとメソッド名から戻り型を引くこと。
- **型再構築（type reconstruction）／HM**〔後編 P3〕… 注釈から型を復元する推論。TAPL 22 章。

## 部分型と多相

- **部分型（subtype, `<:`）**〔前編 P6／後編 P2〕… 「値が期待の型の箱に入る」関係。TAPL 15 章。
- **包摂（subsumption）**〔後編 P1〕… 合成した `S` が `S <: T` なら期待型 `T` に照合できる、の規則。
- **変性（variance）**〔後編 P2〕… 構築子の引数位置での部分型の向き。返り共変・引数反変。
- **gradual consistency（整合）**〔後編 P2〕… `untyped` が絡むときの対称・非推移な関係。`<:` とは別。
- **型代入（substitution）／System F**〔後編 P6〕… 型変数に型を入れる操作。TAPL 23 章。
- **再帰型（μ型）／余帰納（coinduction）**〔後編 P4〕… 自分を参照する型と、その等価判定。TAPL 20–21 章。
- **HKT（高階型）**〔後編 P4〕… 型を取って型を返す型。`App[F, A]`。TAPL 29 章。

## 設計思想

- **gradual typing（漸進的型付け）**〔前編 P9〕… 型の付く所と付かない所を混在させる型付け。
- **never frighten working code（動くコードを脅かさない）**〔前編 P0〕… 誤検知を最上位に避ける規律。
- **robustness principle（Postel の法則）**〔前編 P6／後編 P1〕… 返りは厳密・引数は寛容。
- **健全性（soundness）＝進行＋保存**〔後編 P7〕… 型の付いたプログラムは未定義動作に陥らない、の保証。TAPL 8 章 §8.3。
- **FactStore**〔前編 P4（素朴版）／後編 P5（完全版）〕… フロー感応な「事実」の置き場。
