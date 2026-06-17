---
title: Part 3　ローカル変数と不変 Scope
description: "`x = 1` で型を覚え `x` を読めるようにする。不変 Scope で「上書きしない」設計を体得する。"
sidebar:
  order: 4
---

# The Little chibirigor Part 3　ローカル変数と不変Scope

この章のゴールは、`x = 1`で型を覚え、あとで`x`を読めるようにすることです。そのために「変数名 → 型」の対応である **型環境（Scope）** を導入し、文から文へと縫って渡します。

> [!NOTE]
> 『しくみ』3、4章（『TAPL』 9章＋11章 §11.5「let束縛」）に対応します。同書は型環境を`tyEnv`（変数名→型の対応）と呼び、`{ ...tyEnv, x: 型 }`とコピーして引き回しました。私たちの`Scope`は同じものです。

---

## 3-1. 変数を覚える場所（Scope）

`x = 1`と書いたら「`x`は`Integer`（正確には`1`）」と覚え、あとで`x`を読んだらその型を返したいです。覚えておく場所が要ります。それが**Scope**（型環境）です。ただの「変数名 → 型」の対応です。

```ruby
class Scope
  def initialize(locals = {})
    @locals = locals.freeze
  end

  def local(name)        # その名前の型（未束縛なら nil）
    @locals[name]
  end

  def with_local(name, type)   # 束縛を 1 つ足した「新しい」Scope を返す
    Scope.new(@locals.merge(name => type))
  end
end
```

これは、あなたがコードを読むとき頭の中でやっていることと同じです。`x = 1`の行を見たら「`x`は数だな」と覚え、数行下で`x`が出てきたら「さっきの数」と思い出します。その手元のメモをプログラムが持てるデータにしたのがScopeです。型チェッカーも人間と同じで、変数を見たら「これは何だっけ」と引けるメモが要ります。

ポイントは**不変**であることです。`with_local`は元のScopeを*変えず*、束縛を足した新しいScopeを返します。『しくみ』が`tyEnv`を破壊せず`{ ...tyEnv, x: 型 }`とコピーしたのと同じ作法で、これをオブジェクトの形にしました。

なぜわざわざ*不変*にするのでしょうか。ふつうの`Hash`を`@locals[name] = type`と書き換えても動きそうです。理由は少し先取りになります。Part 4、5で`if`の枝ごとに「この枝の中でだけ`x`は`Integer`」という*別々の*メモを持ちたくなるからです。

ここで言うメモは**型チェッカーが内部で持つもの**で、Rubyの実行時の変数スコープとは別の話です。その内部の`Hash`を1個そのつど書き換えていたら、枝の中で足したメモが枝の外の*検査*にまで残ってしまいます。新しいScopeを返す不変設計なら、「その枝だけのメモ」を元を汚さずに作って渡せます。いまは恩恵が見えにくいですが、この「足しても元は変わらない」性質が、後の章の絞り込み（ナローイング）で効いてきます。

- **① 型理論**：変数の型を覚える対応は型環境tyenvです（『しくみ』 3、4章）。
- **② Rubyだと**：ローカル変数は当たり前に使います。`x = ...; ...x...`。
- **③ Rigorだと**：Scopeは不変です。束縛を足すと新しいScopeを返します（本物のRigorも同じ不変設計）。

---

## 3-2. 変数を読む

`type_of`に1行足すだけです。ローカル変数の読みは、Scopeから型を引きます。

```ruby
when Prism::LocalVariableReadNode then scope.local(node.name) || Type::Dynamic.new
```

`type_of`にはscopeを渡すようにします。受け手や引数の型を求める再帰にも一緒に渡します。未束縛なら`Dynamic`です。型エラーを出して脅かしません。

> [!NOTE]
> ちょっとしたRubyの機微：**代入していない裸の名前（`y`）は、変数ではなくメソッド呼び出し（`self.y`）** です。だから「まだ代入していない`y`を使った」場合、それは未束縛変数の参照ではなく*未知メソッド*の呼び出しとしてPart 2のディスパッチに流れ、黙って`Dynamic`にdegradeします。怒りません。

---

## 3-3. 文を縫う

ここまでの`1 + 2`や`foo.bar`は、評価すると**型が出てくるだけ**でした。でも`x = 1`は違います。型（`1`）が出てくるのに加えて、「以後`x`が使える」という効果をあとに残します。こういう、値を出すだけでなく**スコープを増やす**ものを「文」と呼んでいます。この「あとに効く」分を取りこぼさないために、文を1つ評価して`[その文の型, 更新後のスコープ]`を返す関数を作ります。

```ruby
def eval_statement(node, scope, diagnostics)
  case node
  when Prism::LocalVariableWriteNode
    type = type_of(node.value, scope, diagnostics)   # 右辺の型を求めて…
    [type, scope.with_local(node.name, type)]        # …その名前に束縛した新スコープを返す
  else
    [type_of(node, scope, diagnostics), scope]       # 代入以外はスコープを変えない
  end
end
```

`check`と`annotate`は、文の列を上から評価しながら**スコープを縫って**いきます。前の文で更新したスコープを次の文へ手渡していきます。コードを上から読んで「ここまでで何が定義済みか」を覚えながら進むのと同じ動きです。

```ruby
scope = Scope.new
program.statements.body.each do |stmt|
  _type, scope = eval_statement(stmt, scope, diagnostics)   # scope を更新して次へ
end
```

これで「上で定義した変数を、下で使う」が型でも追えます。

```ruby
check("x = 1\nx + 2")        # OK
check("x = \"a\"\nx + 1")    # ["expected String but got 1"]
```

- **① 型理論**：文を順に評価し、環境を育てながら進みます（『しくみ』 4章の逐次実行）。
- **② Rubyだと**：上から下へ、定義した変数が後ろで見えます。
- **③ Rigorだと**：`[型, スコープ]`を返して縫います。スコープは不変なので「どこで何が見えるか」がはっきりします。

---

## 3-4. 再代入で型が変わる

Rubyでは同じ変数に違う型を入れ直せます。`with_local`は単に束縛を上書きするので、これも自然に追えます。

```ruby
# annotate("x = 1\nx\nx = \"a\"\nx\n")
1: 1       # x は 1
2: 1       # x を読む → 1
3: "a"     # x に "a" を再代入
4: "a"     # x を読む → "a"（型が変わった）
```

```ruby
check("x = 1\nx = \"a\"\nx + 1")   # 再代入後 x は String → ["expected String but got 1"]
```

『しくみ』はここで「同一ブロックでの再定義をエラーにするか」を論点にしましたが、学習の本筋ではないとしてシャドーイング処理を省きました。私たちも同じです。再代入は素直に型の差し替えとして扱います。

---

## 3-5. この章のまとめ

足したものは、`Scope`（不変の型環境）と`eval_statement`（文を縫う）です。`check`/`annotate`はスコープを引き回すようになりました。

動かすとこうなります。

```ruby
Chibirigor.annotate("x = 1\nx\n").each { |a| puts "#{a[:line]}: #{a[:type]}" }
puts Chibirigor.check("x = \"a\"\nx + 1").map { |d| d[:message] }.first
```

<!-- run: examples/part3.rb -->
```text
1: 1
2: 1
expected String but got 1
```

`x = 1`の型`1`が次の行の`x`にそのまま運ばれ（`1: 1` → `2: 1`）、`"a"`を再代入したあとの`x + 1`は`String#+`に`1`（Integer）を渡すので型エラーになります。

この章の三つの視点：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 3、4章 / 『TAPL』 9、11章） | 変数の型を覚える型環境tyenv、文を縫う逐次実行 |
| ② Ruby/RBS | 再代入で型が変わる、裸の名前は代入が無いとメソッド呼び出し |
| ③ Rigor実装の問題 | 不変Scopeで「どこで何が見えるか」を明確化、再代入は型差し替え |

**続編に送ったもの**：

- ブロックが外側のローカルを**捕獲**したときの事実無効化（実RigorのFactStoreの機微）
- 複合代入`x += 1`（`LocalVariableOperatorWriteNode`）、多重代入
- インスタンス変数、定数、グローバル変数などローカル以外の束縛

## 演習

1. `x = 1\ny = x\ny + 2`が通ることを確かめ、型がどう運ばれたかを追え。
2. 再代入`x = 1\nx = "a"`の前後で`x`の型がどう変わるかを`annotate`で観察せよ。
3. 複合代入`x += 1`（Prismでは`LocalVariableOperatorWriteNode`）は今のコードでどう扱われるか。
   対応させるには`eval_statement`に何を足せばよいか考えよ。

---

**次章予告（Part 4とPart 5）**：`if`で型が枝分かれする場合の型（`Union`）をPart 4で、`x.nil?`のような条件で型を絞る「ナローイング」をPart 5で作ります。Scopeがここで本領を発揮します。この「再代入で束縛を差し替える」不変スコープの発想は、後の章の絞り込みでも同じように効いてきます。

---

> **この章の実装（演習の答え合わせにも）** → [`impls/dist/part3/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part3/lib)
