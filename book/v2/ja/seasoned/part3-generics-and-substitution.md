---
title: Part 3　ジェネリクスと型代入
description: 前編でなぞった型変数の置換を本式に組み直し、α 同値、変数捕獲、erasure を形式で扱う。
sidebar:
  order: 13
---

# The Seasoned chibirigor Part 3　ジェネリクスと型代入

> [!TIP]
> 参考書（任意）：TAPL 22章「型再構築」、23章「全称型（System F）」、『しくみ』9章。
> 前編Part 8のRBSでなぞった型変数の置換を、本式に組み直す章です。

前章（Part 2）では、`Cat <: Animal`のような**部分型**で「ある型を別の型の代わりに使える」関係を見ました。
この章はもう一段進んで、型そのものに穴をあけ、中身をあとから差し替えられるようにします。
前編Part 8で、RBSから`Array[Elem]`のような型を読みました。
しかし「`Elem`に`String`を入れて`Array[String]`にする」置換は、ごく簡単な所しか触れていません。
この章は、その**型代入（substitution）**を作ります。
山場は、置換が静かに壊れる2つの落とし穴（**シャドーイング**と**変数捕獲**）です。

---

## 3-1. 型抽象と型適用

ジェネリクスは、型を抽象し（穴を開け）、使う所で適用する（穴を埋める）仕組みです。

```ruby
def select(cond, a, b) = cond ? a : b
# 型: <T>(bool, T, T) -> T      ← <T> が型抽象（型の穴）
# select<Integer>(...)          ← 型適用（穴を Integer で埋める）
```

`<T>`を付けるのが**型抽象（type abstraction）**、`select<Integer>`のように具体型を入れるのが**型適用（type application）**です。
TAPL 23章のSystem Fの用語そのものです。
型適用の中身は`<T>`を外し、本体の`T`をすべて具体型で置き換えること、つまり**型代入**です。

---

## 3-2. 素朴な`subst`と、その落とし穴

`subst(ty, X, repl)`（型`ty`の中の型変数`X`を`repl`で置換）を素朴に書くと：

```
subst(Nominal[C, args], X, repl) = Nominal[C, args.map { subst(_, X, repl) }]
subst(TypeVar[name],    X, repl) = name == X ? repl : TypeVar[name]
subst(TypeAbs[params, body], X, repl) = TypeAbs[params, subst(body, X, repl)]   # ← ここが罠
```

最後の`TypeAbs`（内側の`<...>`）を無条件に潜るのが間違いです。
落とし穴が2つあります。

**(1)シャドーイング**：内側の`<T>`が外側の`T`を隠すとき、内側の`T`は別物なので置換してはいけません。
まず型適用は2段階であることを分けて見ます（混線しやすい所です）：

> [!NOTE]
> 記法の断り：この先の例では、関数（ラムダ）を`=> 本体`で書きます（TypeScriptのアロー関数と同じ）。
> §3-1の`->`が関数の型、`=>`が関数そのものの矢印で、どちらもRubyのHashの`=>`（ロケット）とは無関係です。
> `(arg1: T, …) => true`は「引数を取り`true`を返すラムダ」と読みます。

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

段1（適用で外側を剥がす）と段2（本体への`subst`）を分けると、規則と例が一致します。
段2の停止条件はこれです。
`TypeAbs.params`が置換対象`X`を含むなら、その抽象の中は置換せず返します。

**(2)変数捕獲（variable capture）**：置換する型`repl`が自由変数を含み、それが内側の束縛変数とたまたま同じ名前だと、別物が一つに捕獲されてしまいます。

```ruby
# foo = <T>(arg1: T, arg2: <U>(x: T, y: U) => bool) => true
# bar = <U>() => foo<U>     ← T := U を適用
# 素朴: (arg1: U, arg2: <U>(x: U, y: U) => bool)  ← bar の U と arg2 の U が混線！
```

`bar`由来の`U`と、`arg2`の束縛`U`が、同じ`U`に潰れてしまいます。
これが捕獲バグです。

---

## 3-3. fresh変数で捕獲を避ける

捕獲の修正は、置換の前に内側の束縛変数を新品の名前に付け替えること（α 変換）です。

```
freshTypeAbs(params, body):
  各 param を一意な新名 param@n に付け替え（subst で body も更新）
  → 衝突しようがない名前にしてから、外側の置換を行う
```

`@`（や`#`）のようなプログラマが書けない文字にカウンタを組み合わせて、ぶつからない名前を作ります。
シャドーイングと捕獲回避まで入れた`subst`を動くRubyで書くとこうなります（`TypeAbs`が型抽象`<...>`）：

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

`TypeAbs`の節がこの章の肝です。
①`params.include?(x)`ならシャドーイングなので置換せず返す。
②そうでなければ束縛変数を`fresh_name`（`:"#{p}@#{n}"`）で付け替える。
③そのうえで外側を置換する。
単体で走る設計スケッチ[`examples/subst.rb`](examples/subst.rb)で、3つの肝が**緑**になります：

<!-- run: subst.rb -->
```text
PASS: shadowing leaves the inner T untouched
PASS: non-shadowing substitutes T and freshens U
PASS: capture is avoided (inner U becomes U@1, distinct from the substituted U)
```

検証されている挙動を具体例で言うと：`subst(<T>(T)->Bool, T:=Num)`は`<T>(T) -> Bool`のまま（①シャドーイング）、`subst(<U>(T,U)->Bool, T:=Num)`は`<U@1>(Num, U@1) -> Bool`（②Uをfresh化）、捕獲例`subst(foo_body, T:=U)`は`(U, <U@1>(U, U@1) -> Bool) -> Bool`です。
先頭の`U`（`bar`由来）と内側の`U@1`が別物に保たれます（③捕獲回避）。

最後のケースが捕獲回避そのものです。
`bar`由来の`U`と、`arg2`の内側`<U>`を`U@1`に付け替えた別物が、混線せずに保たれています。

> [!TIP]
> **参考書メモ**：『しくみ』9章は、まず間違った`subst`（`poly_bug.ts`）を見せ、シャドーイングと捕獲を具体例で炙り出し、`freshTypeAbs`で直します。
> この章の構成をそっくり追っています。
> TAPL 23章がSystem Fの代入と α 変換の理論を与えます。

---

## 3-4. 型変数下の等価判定（α 同値）

`<A>(x: A) => A`と`<B>(x: B) => B`は同じ型です（束縛変数の名前が違うだけ）。
これを**α 同値（alpha-equivalence）**と呼びます。
等価判定は、名前の対応表を引き回して解きます：

```
typeEq(TypeAbs[p1, b1], TypeAbs[p2, b2], map):
  p1[i] と p2[i] を対応づけて map に足し、b1 と b2 を比較
typeEq(TypeVar[n1], TypeVar[n2], map):
  map[n1] == n2     # n1 を対応表で翻訳してから比べる
```

「束縛変数の名前対応をmapで持つ」この技法は、次章Part 4の再帰型の等価判定でも同じ形で出てきます。
α 同値も再帰型の α 同値も、根は一つです。

---

## 3-5. erasure（型を消してRBSに戻す）

型適用は実行時には何もしません。
`select<Integer>`は実行時にはただの`select`です。
TAPL 23.7のerasure（消去）定理は「型注釈と型適用を消しても実行結果は変わらない」を保証します。
Javaのジェネリクスの「型消去」とは別物で、あちらは生成コードから型を落とす実装手法、こちらは意味論の定理です。

Rigorにとってerasureはもう一つ意味があります。
内部の豊かな型を、保守的にRBSへ戻す操作です。
`HashShape`はRBSのrecordか`Hash[K,V]`へ、リテラルunionは基底クラスへ、`Dynamic[T]`は`untyped`へ戻します。
**より広くはなっても、決して狭くならない**（健全な近似）。
「RigorはRBSのスーパーセットで、いつでもRBSに書き戻せる」の中身です。

---

## 3-6. Rigorの中では

- **型代入**：`RbsTypeTranslator.translate(..., type_vars:)`が正道の`subst`。`type_vars[:Elem]=String`で`Array[Elem]` → `Array[String]`。RBS由来なので、『しくみ』9章ほど一般のネスト型抽象にさらされず、捕獲の露出面は小さいです（思想は同じ α 変換）。
- **有界量化（`X extends T`）**：TAPL 26章。Rigorでは構造契約（interface/capability role）への適合がその役割の一部を担う。
- **erasure**：`Type#erase_to_rbs`。export時にRBSへ保守的変換。

---

## 3-6x. 発展：要素型の読みはlibに入った（generics 5a）

> [!IMPORTANT]
> genericsのlib化は3段に分けて進めます。**5a＝要素型の読み、5b＝ブロック仮引数への押し下げ、5c＝戻り多相**。本節は**5a**、続く5bと5cは後編Part 5「5-6x」で扱います。

ここまでは「`Array[Elem]`の`Elem`をどう置換するか」を概念とスケッチ（`subst.rb`）で見てきました。
その入口、既知の配列から`Elem`を読み出す所は、chibirigor本体に昇格済みです（`lib/chibirigor/type_of.rb`の`element_read`）。
特別なフラグも要らず`annotate`/`check`で効きます：

```console
$ printf '[1, 2, 3].first\n{ a: 1, b: 2 }.values\n[].first\n' | ruby exe/chibirigor annotate /dev/stdin
1: Integer
2: Integer
3: untyped
```

`[1,2,3].first`が`Integer`、`{a:1,b:2}.values`が`Integer`で、要素型`Elem`を読めています（リテラル精度`Const`はここでclassに丸めます。`1`でなく`Integer`、つまり「要素型」の抽象）。
非リテラル添字`a[i]`も同じく要素型を返します（位置が不明なため）。
一方、リテラル添字`a[0]`は前編Part 5の位置どおりの精度（Tuple読み）を保ちます。
位置で読むか要素型で読むかの使い分けです。

誤検知ゼロも守ります。
空配列`[].first`や未知レシーバ`foo.first`は`untyped`です（埋まらなければuntyped）。
読んだ要素型はチェックにも流れます。
`a = [1,2]; a.first + true`は「can't add true to an Integer」を1件出します。

ここでlibが獲得したのはgenericsの読み（5a）です。
続く押し下げ（5b）、要素型をブロック仮引数へ流し込む（`map { |x| ... }`の`x`を`Elem`にする）もlibに入りました。
詳細とworked example、そして「なぜ単一化でなく直接代入で済むのか」は後編Part 5「5-6x」へ。
本章の主役だった型代入`subst`と、要素が未知の型変数の一般ケースを解く本格的な単一化（[`examples/unification.rb`](examples/unification.rb)）は、設計スケッチのまま残してあります。

## 3-7. まとめ

- ジェネリクスは型抽象（`<T>`）に型適用（穴を埋める）を組み合わせた仕組みで、型代入（System F、TAPL 23章）として定式化されます。
- 素朴な`subst`は**シャドーイング**（内側の同名は別物）と**変数捕獲**（自由変数の衝突）で壊れます。
- 修正方針：シャドーイングは「含むなら置換せず返す」、捕獲は**fresh変数で α 変換してから置換**します。
- 型変数下の等価は**α 同値**（名前対応表）で、再帰型と同じ技法を使います。
- **erasure**：型適用は実行時に消えます。Rigorは内部型を保守的にRBSへ戻せます。

## 演習

1. **捕獲をトレース**：`subst.rb`の捕獲例`subst(foo_body, T:=U)`を、fresh変数を使わずに手でたどり、どの段階で`bar`の`U`と`arg2`の`U`が同じ名前に潰れるかを示せ。
2. **α 同値**：`<A>(x: A) -> A`と`<B>(x: B) -> B`が等しいことを、名前対応表`map`を使って1ステップずつ示せ（`map`に何を足すか）。
3. **シャドーイングの2段階**：`(<T>(arg1: T, arg2: <T>(x:T)=>bool)=>true)<Integer>`を、§3-2の「段1【適用】→段2【代入】」に沿って評価し、結果のどこが`Integer`になりどこが`T`のままかを書け。

**次章（Part 4）**：型に自分自身が現れる**再帰型**を、μ（不動点）と余帰納で扱います。
等価判定にこの章と同じ α 同値の技法が再登場します。

---

> **この章の設計スケッチ** → [`examples/subst.rb`](examples/subst.rb)（`ruby subst.rb`で自己チェック）
