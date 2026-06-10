---
title: The Seasoned chibirigor Part 3 ― ジェネリクスと型代入
description: 前編でなぞった型変数の置換を本式に組み直し、α 同値・変数捕獲・erasure を形式で扱う。
sidebar:
  order: 13
---

# The Seasoned chibirigor Part 3 ― ジェネリクスと型代入

> 参考書（任意）：TAPL 22 章「型再構築」・23 章「全称型（System F）」／『しくみ』9 章。
> 前編 Part 8 の RBS で*なぞった*型変数の置換を、本式に組み直す章です。

前章（Part 2）では、`Cat <: Animal` のような**部分型**で「ある型を別の型の代わりに使える」関係を
見ました。この章はもう一段すすんで、型そのものに**穴をあける** ― 中身をあとから差し替えられる
ようにします。前編 Part 8 で、RBS から `Array[Elem]` のような型を読みました。でも「`Elem` に
`String` を入れて `Array[String]` にする」置換は、ごく簡単な所しか触れていません。この章は、その
**型代入（substitution）** を正面から作ります。山場は、置換が静かに壊れる 2 つの落とし穴 ―
**シャドーイング**と**変数捕獲** ― です。

---

## 3-1. 型抽象と型適用

ジェネリクスは、型を**抽象**し（穴を開け）、使う所で**適用**する（穴を埋める）仕組みです。

```ruby
def select(cond, a, b) = cond ? a : b
# 型: <T>(bool, T, T) -> T      ← <T> が型抽象（型の穴）
# select<Integer>(...)          ← 型適用（穴を Integer で埋める）
```

`<T>` を付けるのが**型抽象（type abstraction）**、`select<Integer>` のように具体型を入れるのが
**型適用（type application）**。TAPL 23 章の System F の用語そのものです。型適用の中身は ―
`<T>` を外し、本体の `T` を*すべて*具体型で置き換える、つまり**型代入**です。

---

## 3-2. 素朴な `subst` と、その落とし穴

`subst(ty, X, repl)`（型 `ty` の中の型変数 `X` を `repl` で置換）を素朴に書くと：

```
subst(Nominal[C, args], X, repl) = Nominal[C, args.map { subst(_, X, repl) }]
subst(TypeVar[name],    X, repl) = name == X ? repl : TypeVar[name]
subst(TypeAbs[params, body], X, repl) = TypeAbs[params, subst(body, X, repl)]   # ← ここが罠
```

最後の `TypeAbs`（内側の `<...>`）を**無条件に**潜るのが間違いです。落とし穴が 2 つ：

**(1) シャドーイング**：内側の `<T>` が外側の `T` を*隠す*とき、内側の `T` は別物なので置換しては
いけない。まず**型適用は 2 段階**であることを分けて見ます（混線しやすい所）：

```ruby
# 型適用 (<T>BODY)<Integer> は 2 段階：
#   段1【適用】 外側の <T> を剥がし、「BODY に T:=Integer を代入する」と決める。
#   段2【代入】 subst(BODY, T, Integer) を実行する。
#
# BODY = (arg1: T, arg2: <T>(x: T) => bool) => true  のとき段2は：
#   - arg1 の T          → Integer に置換（外側の T）
#   - arg2 の内側 <T> … → ここで停止。内側 <T> は *別の* T（シャドーイング）なので中は触らない
# 結果: (arg1: Integer, arg2: <T>(x: T) => bool) => true
```

段1（適用で外側を剥がす）と段2（本体への `subst`）を分けると、規則と例が一致します。
段2 の停止条件はこれ ―
→ **`TypeAbs.params` が置換対象 `X` を含むなら、その抽象の中は置換せず返す。**

**(2) 変数捕獲（variable capture）**：置換する型 `repl` が*自由変数*を含み、それが内側の束縛変数と
*たまたま同じ名前*だと、別物が一つに**捕獲**されてしまう。

```ruby
# foo = <T>(arg1: T, arg2: <U>(x: T, y: U) => bool) => true
# bar = <U>() => foo<U>     ← T := U を適用
# 素朴: (arg1: U, arg2: <U>(x: U, y: U) => bool)  ← bar の U と arg2 の U が混線！
```

`bar` 由来の `U` と、`arg2` の束縛 `U` が、同じ `U` に潰れてしまう。これが捕獲バグです。

---

## 3-3. fresh 変数で捕獲を避ける

捕獲の修正は、**置換の前に、内側の束縛変数を*新品の名前*に付け替える**（α 変換）こと：

```
freshTypeAbs(params, body):
  各 param を一意な新名 param@n に付け替え（subst で body も更新）
  → 衝突しようがない名前にしてから、外側の置換を行う
```

`@`（や `#`）のような*プログラマが書けない文字*＋カウンタで、ぶつからない名前を作ります。
動く Ruby で、シャドーイング・捕獲回避まで入れた `subst` はこうなります（`TypeAbs` が型抽象
`<...>`）：

<!-- include: subst.rb#subst -->
```ruby
# 型 ty の中の型変数 x を repl で置換する。
def subst(ty, x, repl)
  case ty
  in Symbol then ty
  in Var then ty.name == x ? repl : ty
  in Arrow then Arrow.new(ty.params.map { subst(it, x, repl) }, subst(ty.ret, x, repl))
  in TypeAbs
    return ty if ty.params.include?(x) # シャドーイング → その抽象の中は置換しない

    body = ty.body
    new_params = ty.params.map do |p| # 束縛変数を fresh に α 変換してから…
      np = fresh_name(p)
      body = subst(body, p, Var.new(np))
      np
    end
    TypeAbs.new(new_params, subst(body, x, repl)) # …外側の置換（捕獲が起きない）
  end
end
```

`TypeAbs` の節がこの章の肝です ―（①）`params.include?(x)` ならシャドーイングなので置換せず返す、
（②）そうでなければ束縛変数を `fresh_name`（`:"#{p}@#{n}"`）で付け替えてから、（③）外側を置換する。
単体で走る設計スケッチ [`examples/subst.rb`](examples/subst.rb) で、3 つの肝が**緑**になります：

<!-- run: subst.rb -->
```text
PASS: shadowing leaves the inner T untouched
PASS: non-shadowing substitutes T and freshens U
PASS: capture is avoided (inner U becomes U@1, distinct from the substituted U)
```

検証されている挙動を具体例で言うと：`subst(<T>(T)->Bool, T:=Num)` は `<T>(T) -> Bool` の*まま*
（①シャドーイング）、`subst(<U>(T,U)->Bool, T:=Num)` は `<U@1>(Num, U@1) -> Bool`（②U を fresh 化）、
そして捕獲例 `subst(foo_body, T:=U)` は `(U, <U@1>(U, U@1) -> Bool) -> Bool` ― 先頭の `U`（`bar`
由来）と内側の `U@1` が別物に保たれます（③捕獲回避）。

最後のケースが捕獲回避そのものです ― `bar` 由来の `U` と、`arg2` の内側 `<U>` を `U@1` に
付け替えた別物が、混線せずに保たれています。

> **参考書メモ**：『しくみ』9 章は、まず*間違った* `subst`（`poly_bug.ts`）を見せ、シャドーイングと
> 捕獲を具体例で炙り出し、`freshTypeAbs` で直す ― この章の構成をそっくり追っています。
> TAPL 23 章が System F の代入と α 変換の理論を与えます。

---

## 3-4. 型変数下の等価判定 ― α 同値

`<A>(x: A) => A` と `<B>(x: B) => B` は**同じ型**です（束縛変数の名前が違うだけ）。これを
**α 同値（alpha-equivalence）** と呼びます。等価判定は、**名前の対応表**を引き回して解きます：

```
typeEq(TypeAbs[p1, b1], TypeAbs[p2, b2], map):
  p1[i] と p2[i] を対応づけて map に足し、b1 と b2 を比較
typeEq(TypeVar[n1], TypeVar[n2], map):
  map[n1] == n2     # n1 を対応表で翻訳してから比べる
```

この「束縛変数の名前対応を map で持つ」技法は、次章 Part 4 の再帰型の等価判定でも*同じ形*で
出てきます ― α 同値も再帰型の α 同値も、根は一つです。

---

## 3-5. erasure ― 型を消して RBS に戻す

型適用は**実行時には何もしません**。`select<Integer>` は実行時にはただの `select`。TAPL 23.7 の
*erasure（消去）定理*は「型注釈・型適用を消しても実行結果は変わらない」を保証します（Java の
ジェネリクスの「型消去」とは別物 ― あちらは生成コードから型を落とす実装手法、こちらは
*意味論* の定理です）。

Rigor にとって erasure はもう一つ意味があります ― **内部の豊かな型を、保守的に RBS へ戻す**
操作です。`HashShape` は RBS の record か `Hash[K,V]` へ、リテラル union は基底クラスへ、
`Dynamic[T]` は `untyped` へ。**より広くはなっても、決して狭くならない**（健全な近似）。これが
「Rigor は RBS のスーパーセットで、いつでも RBS に書き戻せる」の中身です。

---

## 3-6. Rigor の中では

- **型代入**：`RbsTypeTranslator.translate(..., type_vars:)` が正道の `subst`。`type_vars[:Elem]=String`
  で `Array[Elem]` → `Array[String]`。RBS 由来なので、`『しくみ』`9 章ほど一般のネスト型抽象に
  さらされず、**捕獲の露出面は小さい**（が、思想は同じ α 変換）。
- **有界量化（`X extends T`）**：TAPL 26 章。Rigor では構造契約（interface/capability role）への
  適合がその役割の一部を担う。
- **erasure**：`Type#erase_to_rbs`。export 時に RBS へ保守的変換。

---

## 3-6x. 発展：要素型の読みは lib に入った（generics 5a）

ここまでは「`Array[Elem]` の `Elem` をどう置換するか」を概念とスケッチ（`subst.rb`）で見てきました。
その**入口** ― 既知の配列から `Elem` を*読み出す*ところ ― は、chibirigor 本体に昇格済みです
（`lib/chibirigor/type_of.rb` の `element_read`）。特別なフラグも要らず `annotate`/`check` で効きます：

```console
$ printf '[1, 2, 3].first\n{ a: 1, b: 2 }.values\n[].first\n' | ruby exe/chibirigor annotate /dev/stdin
1: Integer
2: Integer
3: untyped
```

`[1,2,3].first` が `Integer`、`{a:1,b:2}.values` が `Integer` ― **要素型 `Elem` を読めて**います
（リテラル精度 `Const` はここで class に丸めます。`1` でなく `Integer` ＝「要素型」の抽象）。
非リテラル添字 `a[i]` も同じく要素型を返します（位置が不明だから）。一方、リテラル添字 `a[0]` は
前編 Part 5 の**位置どおりの精度**（Tuple 読み）を保ちます ― *位置で読む*か*要素型で読む*かの
使い分けです。

誤検知ゼロも守ります：**空配列 `[].first` や未知レシーバ `foo.first` は `untyped`**（埋まらねば
untyped）。読んだ要素型はちゃんとチェックにも流れます ― `a = [1,2]; a.first + true` は
「Integer に true は足せません」を 1 件出します。

ここで lib が獲得したのは generics の**読み（5a）**までです。本章の主役だった**型代入 `subst`**と、
要素型をブロック仮引数へ押し下げる**単一化**（`map { |x| ... }` の `x` を `Elem` にする＝
[`examples/unification.rb`](examples/unification.rb)）は、まだ設計スケッチのまま ― これらを lib に
織り込む **5b** が、generics を一本につなぐ続きの一歩です。

## 3-7. まとめ

- ジェネリクス＝型抽象（`<T>`）＋型適用（穴を埋める）＝型代入（System F、TAPL 23 章）。
- 素朴な `subst` は **シャドーイング**（内側の同名は別物）と **変数捕獲**（自由変数の衝突）で壊れる。
- 直し：シャドーイングは「含むなら置換せず返す」、捕獲は **fresh 変数で α 変換してから置換**。
- 型変数下の等価は **α 同値**（名前対応表）― 再帰型と同じ技法。
- **erasure**：型適用は実行時に消える／Rigor は内部型を保守的に RBS へ戻せる。

## 演習

1. **捕獲をトレース**：`subst.rb` の捕獲例 `subst(foo_body, T:=U)` を、**fresh 変数を使わずに**
   手でたどり、どの段階で `bar` の `U` と `arg2` の `U` が同じ名前に潰れるかを示せ。
2. **α 同値**：`<A>(x: A) -> A` と `<B>(x: B) -> B` が等しいことを、名前対応表 `map` を使って
   1 ステップずつ示せ（`map` に何を足すか）。
3. **シャドーイングの 2 段階**：`(<T>(arg1: T, arg2: <T>(x:T)=>bool)=>true)<Integer>` を、§3-2 の
   「段1【適用】→段2【代入】」に沿って評価し、結果のどこが `Integer` になりどこが `T` のまま
   かを書け。

**次章（Part 4）**：型に*自分自身*が現れる**再帰型**を、μ（不動点）と余帰納で扱います。等価判定に
この章と同じ α 同値の技法が再登場します。
