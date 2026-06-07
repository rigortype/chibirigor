# 【試し書き】The Little chibirigor Part 3 ― ローカル変数と不変 Scope

> 変数を導入し、「型環境＝Scope」を登場させる章。実装は `lib/chibirigor/scope.rb` と
> `evaluator.rb` に反映済み。コードは実 Prism/Ruby で動作確認済み。

この章のゴール：**`x = 1` で型を覚え、あとで `x` を読めるようにする。** そのために
「変数名 → 型」の対応＝**型環境（Scope）**を導入し、文から文へと縫って渡します。

> 『しくみ』3・4 章に対応します。あの本は型環境を `tyEnv`（変数名→型の対応）
> と呼び、`{ ...tyEnv, x: 型 }` とコピーして引き回しました。私たちの `Scope` は同じものです。

---

## 3-1. 変数を覚える場所 ― Scope

`x = 1` と書いたら「`x` は `Integer`（正確には `1`）」と覚え、あとで `x` を読んだらその型を
返したい。覚えておく場所が要ります。それが **Scope**（型環境）です ― ただの「変数名 → 型」の
対応：

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

ポイントは **不変**であること。`with_local` は元の Scope を*変えず*、束縛を足した新しい Scope を
返します。『しくみ』が `tyEnv` を破壊せず `{ ...tyEnv, x: 型 }` とコピーしたのと同じ作法で、これを
オブジェクトの形にしました。

- **① 型理論**：変数の型を覚える対応＝型環境 tyenv（『しくみ』 3・4 章）。
- **② Ruby だと**：ローカル変数は当たり前に使う。`x = ...; ...x...`。
- **③ Rigor だと**：Scope は不変。束縛を足すと新しい Scope を返す（本物の Rigor も同じ不変設計）。

---

## 3-2. 変数を読む

`type_of` に 1 行足すだけ。ローカル変数の読みは、Scope から型を引きます：

```ruby
when Prism::LocalVariableReadNode then scope.local(node.name) || Type::Dynamic.new
```

`type_of` には scope を渡すようにします（受け手や引数の型を求める再帰にも一緒に渡す）。
未束縛なら `Dynamic`（脅かさない）。

> ちょっとした Ruby の機微：**代入していない裸の名前（`y`）は、変数ではなくメソッド呼び出し
> （`self.y`）**です。だから「まだ代入していない `y` を使った」場合、それは未束縛変数の参照では
> なく*未知メソッド*の呼び出しとして Part 2 のディスパッチに流れ、黙って `Dynamic` に
> degrade します。怒りません。

---

## 3-3. 文を縫う

`x = 1` は「式」ではなく「文」で、**スコープを増やす**働きがあります。そこで、文を 1 つ評価して
**`[その文の型, 更新後のスコープ]`** を返す関数を作ります：

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

`check` と `annotate` は、文の列を上から評価しながら**スコープを縫って**いきます：

```ruby
scope = Scope.new
program.statements.body.each do |stmt|
  _type, scope = eval_statement(stmt, scope, diagnostics)   # scope を更新して次へ
end
```

これで「上で定義した変数を、下で使う」が型でも追えます：

```ruby
check("x = 1\nx + 2")        # OK
check("x = \"a\"\nx + 1")    # ["String が必要ですが 1 が渡されました"]
```

- **① 型理論**：文を順に評価し、環境を育てながら進む（『しくみ』 4 章の逐次実行）。
- **② Ruby だと**：上から下へ、定義した変数が後ろで見える。
- **③ Rigor だと**：`[型, スコープ]` を返して縫う。スコープは不変なので「どこで何が見えるか」が
  はっきりする。

---

## 3-4. 再代入で型が変わる

Ruby では同じ変数に違う型を入れ直せます。`with_local` は単に束縛を上書きするので、これも
自然に追えます：

```ruby
# annotate("x = 1\nx\nx = \"a\"\nx\n")
1: 1       # x は 1
2: 1       # x を読む → 1
3: "a"     # x に "a" を再代入
4: "a"     # x を読む → "a"（型が変わった）
```

```ruby
check("x = 1\nx = \"a\"\nx + 1")   # 再代入後 x は String → ["String が必要ですが 1 が渡されました"]
```

『しくみ』はここで「同一ブロックでの再定義をエラーにするか」を論点にしましたが、学習の本筋では
ないとしてシャドーイング処理を省きました。私たちも同じ。再代入は素直に型の差し替えとして
扱います。

---

## 3-5. この章のまとめ

足したもの：`Scope`（不変の型環境）と `eval_statement`（文を縫う）。`check`/`annotate` は
スコープを引き回すようになりました。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（『しくみ』 3・4 章） | 変数の型を覚える型環境 tyenv、文を縫う逐次実行 |
| ② Ruby/RBS | 再代入で型が変わる、裸の名前は代入が無いとメソッド呼び出し |
| ③ Rigor 実装の問題 | 不変 Scope で「どこで何が見えるか」を明確化、再代入は型差し替え |

**続編に送ったもの**：

- ブロックが外側のローカルを**捕獲**したときの事実無効化（実 Rigor の FactStore の機微）。
- 複合代入 `x += 1`（`LocalVariableOperatorWriteNode`）、多重代入。
- インスタンス変数・定数・グローバル変数などローカル以外の束縛。

**次章予告（Part 4）**：`if` で型が枝分かれする場合の型（`Union`）と、`x.nil?` のような条件で
型を絞る「ナローイング」を作ります。Scope がここで本領を発揮します。

---

> **検証メモ**
> - 複雑さ予算：新規は `Scope` と `eval_statement` の 2 つ。`type_of` は scope 引数が増えただけ
>   （ロジックは 1 行追加）。1 step 1 難所を維持。○
> - 回帰なし：Part 1・2 のテストは全て緑（公開 API `check`/`annotate` は不変）。○
> - 三題噺：③「不変 Scope」が①（tyenv のコピー作法）の素直な実体化として出た。○
> - 余録：②の「裸の名前＝メソッド呼び出し」を本文に拾えた。これが Part 4 のナローイング
>   （局所変数にしか効かない）の伏線になる。
