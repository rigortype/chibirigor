# 【試し書き】The Little chibirigor Part 9（最終章）― gradual の哲学

> ここまでを `gradual`（漸進的型付け）の価値観で締める最終章。実装は `type.rb`・`checker.rb`
> に少しだけ反映済み。コードは実 Prism/Ruby で動作確認済み。

この章のゴール：**`untyped` の伝播を仕上げ、「chibirigor がわざと見逃している所」を総括する。**
そして、副読本『型システムのしくみ』が最後に指さした gradual typing と接続します。

---

## 9-1. `untyped` は伝染する

最後の小さなコード変更を一つ。`untyped`（`Dynamic`）が union に混じったら、**全体を
`untyped`** にします：

```ruby
def union(types)
  flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
  return Dynamic.new if flat.empty?
  return Dynamic.new if flat.any?(Dynamic)   # ★ untyped が混じれば全体 untyped
  return flat.first if flat.size == 1
  Union[flat.freeze]
end
```

```console
x = c ? 1 : foo.bar   # => untyped   （片方が未知なら、全体が未知）
x = c ? 1 : "a"       # => 1 | "a"   （未知が無ければ union のまま）
```

理屈は素直です。一部でも型を見失っているなら、その合併について断言はできない。`1 | untyped`
と中途半端に持つより、`untyped` と正直に言う。これが gradual の芯です。

---

## 9-2. chibirigor が「わざと見逃している」4 箇所

chibirigor は健全（sound）ではありません。**わざと**見逃します。それは「動くコードを
脅かさない」ためです。どこで見逃しているかを、最後にはっきりさせます：

1. **`untyped` は何でも受理する**（Part 6 の `accepts` が `:maybe`）。型を見失った境界の穴。
2. **ハッシュの未知キーは `nil`**（Part 5）。余剰キーも許す（open）。本書のように完全一致で
   弾けば、動く options ハッシュが真っ赤になる。
3. **`:maybe` を罰しない**（Part 6 の `dispatch`）。疑わしきは黙る。
4. **絞り込みは保守的**（Part 4）。読めない条件・disjoint・`Dynamic` は絞らない。

どれも「見逃し＝バグを見落とす危険」と引き換えに「誤検知＝動くコードを脅かす」を避けて
います。chibirigor（と Rigor）は、後者のコストをずっと重く見ます。

- **① 型理論**：健全性（未定義動作の排除）か、誤検知の少なさか、というトレードオフ。
- **② Ruby だと**：動的言語に後付けの型。厳しすぎると現場のコードが回らない。
- **③ Rigor だと**：「動くコードを脅かさない」を最上位の価値に。わざと見逃す。

---

## 9-3. baseline ― 既存を呑み、新規だけ見る

それでも、初めて既存コードにかけると診断が大量に出ることはあります。そこで **baseline**：
最初に出た診断を「呑んだもの」として記録し、以降は**新規の診断だけ**を見せます。

```ruby
def check(source, baseline = [])
  # …診断を集める…
  diagnostics.reject { |d| baseline.include?(d) }
end
```

```ruby
base = check(source)              # 最初の診断を baseline として保存
check(edited_source, base)        # baseline に無い新規だけが出る
```

「既存の指摘は一旦そのまま、これから書くコードだけ綺麗に」── これも「脅かさない」の一形態
です（Rigor の `.rigor-baseline.yml` の縮図）。

---

## 9-4. 本書の終着点が、chibirigor の出発点

副読本『型システムのしくみ』は、TypeScript のミニ言語に対する**静的で健全な検査器**を作り、
最後の「おわりに」で次のフロンティアとして **gradual typing**（Siek & Taha 2006 ほか）の論文を
指さして終わります。

chibirigor は、ちょうどその先から始めました：

| | 本書（静的・健全） | chibirigor（漸進的） |
|---|---|---|
| 判定 | OK / NG の二値 | `:yes`/`:no`/`:maybe` の三値 |
| 未知の型 | 無い（ミニ言語） | `untyped`（`Dynamic`）が主役 |
| NG のとき | 例外で止まる | 診断を貯めて続行、未知は黙る |
| 価値観 | 健全性 | 誤検知を出さない（脅かさない） |

本書を読んでから chibirigor を作ると、「なぜ三値なのか」「なぜ `untyped` に逃がすのか」
「なぜわざと見逃すのか」が腑に落ちます。そして chibirigor を作ってから本物の Rigor を読むと、
ここで見た一つ一つ（`Scope`・`accepts`・ナローイング・RBS・sig 推論）が、実用規模で
作り込まれているのが分かるはずです。

**本書の終着点が、chibirigor の出発点。chibirigor の出発点が、Rigor への入口。**

---

## 9-5. ここまでのまとめ（本編 全 9 章）

| Part | 足したもの |
|---|---|
| 1 | `Const`/`Dynamic`/`Nominal`、`type_of`、`check`/`annotate` |
| 2 | `Dispatch`（メソッド表）、未知は degrade |
| 3 | 不変 `Scope`、`eval_statement`（文を縫う） |
| 4 | `Union`、`if`/三項、`nil?`/`is_a?` ナローイング（dead branch は絞らない） |
| 5 | `HashShape`/`Tuple`、添字読み（未知キーは nil） |
| 6 | `accepts`（三値）、`:no` だけ報告 |
| 7 | `Rbs.load`（手書き表を RBS 由来へ differ 置換） |
| 8 | `def` の戻り型合成、RBS 風シグネチャ、`untyped` の可視化 |
| 9 | `untyped` の伝播、わざと見逃す総括、baseline |

**続編「The Seasoned chibirigor」へ**：双方向型付けの形式化、変性、再帰型、本物の型推論（引数推論）、
完全な FactStore、健全性理論、`erasure`/sig-gen 本体 ―『型システムのしくみ』の先と、Rigor の
作り込みへ。

---

> **検証メモ**
> - 最終章にふさわしく、新規コードは最小（union の 1 行＋baseline の 1 行）。残りは*総括*。○
> - 「わざと見逃す 4 箇所」は全て既出の実装を指すだけ＝伏線回収になっている。○
> - 本書との対比表で「終着点＝出発点」を締めに使えた。○
> - 全 9 章のテスト（test_part1〜9）が緑。The Little chibirigorは一通り通った。○
