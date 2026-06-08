---
title: "Part 9（最終章）― gradual の哲学"
description: "`untyped` の伝播を仕上げ、chibirigor がわざと見逃す 4 箇所と gradual typing の哲学を整理する。"
sidebar:
  order: 9
---

# The Little chibirigor Part 9（最終章）― gradual の哲学

この章のゴール：**`untyped` の伝播を仕上げ、「chibirigor がわざと見逃している所」を総括する。**
そして、漸進的型付け（gradual typing）という大きな流れと、その先の Rigor へ接続します。

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
2. **ハッシュの未知キーは `nil`**（Part 5）。余剰キーも許す（open）。『しくみ』のように完全一致で
   弾けば、動く options ハッシュが真っ赤になる。
3. **`:maybe` を罰しない**（Part 6 の `dispatch`）。疑わしきは黙る。
4. **絞り込みは保守的**（Part 4）。読めない条件・disjoint・`Dynamic` は絞らない。

どれも「見逃し＝バグを見落とす危険」と引き換えに「誤検知＝動くコードを脅かす」を避けて
います。chibirigor（と Rigor）は、後者のコストをずっと重く見ます。

> いま標語として挙げたこの 4 つは、後編 **The Seasoned chibirigor** Part 7 で、
> **健全性（progress ＋ preservation）を*わざと*破る穴**として、形式の言葉で言い直します。
> 「なぜ unsound でよいのか」に、理論からの答えが付きます。

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

> **コラム：「なぜ報告されないのか」を調べる `rigor check --explain`**
>
> Rigor を使っていると「このバグを見落としているのでは？」という疑問が生じることがあります。
> そのときの調査コマンドが `rigor check --explain` です。
>
> 通常の `rigor check` は証明できた問題だけを報告し、`Dynamic[Top]`（`untyped`）に
> *fall-soft* した箇所は黙っています。`--explain` を付けると、その**fall-soft した全箇所が
> `:info` 診断として現れます** ― 「ここで型を見失いました」という地図が出力されます。
>
> 「なぜ診断が出ないか」の答えは大抵「その式が `untyped` になっているから」です。
> `--explain` の出力を手がかりに「どこで型が消えたか」を遡ると、
> RBS の不足・プラグイン未設定・型注釈の抜け漏れを発見できます。
> chibirigor の「知らなければ黙る」は誤検知を防ぎますが、それは同時に
> 「静かに見逃す」でもあります ― `--explain` はその沈黙を可視化する道具です。

---

## 9-4. ここから先 ― gradual typing と Rigor へ

chibirigor は、最初から一貫してこの立場で作ってきました：

| | 静的・健全なチェッカー | chibirigor（漸進的） |
|---|---|---|
| 判定 | 適合・不適合の二値 | `:yes`/`:no`/`:maybe` の三値 |
| 未知の型 | 無い（注釈必須） | `untyped`（`Dynamic`）が主役 |
| 不適合のとき | 例外で止まる | 診断を貯めて続行、未知は黙る |
| 価値観 | 健全性 | 誤検知を出さない（脅かさない） |

「型を見失っても止まらない」「わからない所は黙る」「動くコードを脅かさない」── この漸進的型付け
は型システム研究の一つの到達点で、Siek & Taha の gradual typing（2006）などが
土台にあります。chibirigor は、その入口を*自分の手で*なぞったものです。

> **参考書メモ（任意）**：『しくみ』や TAPL をお持ちなら ― あの本は静的で健全なチェッカーを
> 作り、最後に「次のフロンティア」として gradual typing を指さして終わります。chibirigor は、
> ちょうどその先から始めた、と読めます。併読すると「なぜ三値か」「なぜ `untyped` に逃がすか」が
> 腑に落ちやすいでしょう（無くても本書だけで完結します）。健全性の本式は TAPL 8 章 §8.3「安全性 = 進行 + 保存」、gradual はその先（TAPL 後の研究）です。

そして chibirigor を作り終えたいま、本物の Rigor を覗くと、ここで見た一つ一つ
（`Scope`・`accepts`・ナローイング・RBS・sig 推論）が、実用規模で作り込まれているのが
分かります。**chibirigor は、Rigor への入口です。**

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
| 7 | `Rbs.load`（手書き表を RBS 由来へ差し替え：differ 置換） |
| 8 | `def` の戻り型合成、RBS 風シグネチャ、`untyped` の可視化 |
| 9 | `untyped` の伝播、わざと見逃す総括、baseline |

## 前編を読み終えたあなたができること

数百行の Ruby だけで、ここまで来ました：

- Ruby のソースから**型を推論**し（リテラル・メソッド・変数・分岐・ハッシュ/配列）、
- 矛盾を**診断**として報告し（しかも止まらず、わからない所は黙る）、
- 推論した型と**メソッドのシグネチャ**を `annotate` で見せ、
- 「**動くコードを脅かさない**」を、`untyped`・三値・open shape・保守的ナローイングという
  具体的な仕組みとして実装した。

これは本物の Rigor の最小版です。後編では、この一つ一つに**理論の裏打ち**を与えます。

## 演習

1. `c ? 1 : foo.bar` が `untyped` になることを確かめ、`union` のどの行が効いたか指せ。
2. `check` に baseline を渡し、既存の診断が消え、新規の診断だけが出ることを確かめよ。
3. 「わざと見逃す 4 箇所」のうち 1 つ（例：未知キー→nil）を、あえて厳しく（エラーに）したら、
   どんな*動くコード*が脅かされるか、具体例を 1 つ挙げよ。

---

**続編「The Seasoned chibirigor」へ**：双方向型付けの形式化、変性、再帰型、本物の型推論（引数推論）、
完全な FactStore、健全性理論、`erasure`/sig-gen 本体 ―『型システムのしくみ』の先と、Rigor の
作り込みへ。

