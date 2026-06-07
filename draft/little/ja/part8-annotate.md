# 【試し書き】The Little chibirigor Part 8 ― annotate を仕上げる

> `annotate` に「メソッドの推論シグネチャを RBS 風に見せる」を足す章。実装は
> `lib/chibirigor/annotator.rb`・`type_of.rb` に反映済み。コードは実 Prism/Ruby で動作確認済み。

この章のゴール：**`def` を読み、本体から戻り型を合成して RBS 風シグネチャで見せる。**
ここで `chibirigor` が「推論器」であることが一番はっきりします ― 注釈ゼロのメソッドから、
型が立ち上がってくる。

---

## 8-1. 戻り型は本体から合成できる

Ruby のメソッドには型注釈がありません。でも**戻り型は本体から分かる**ことが多い：

```ruby
def greet
  "hi".upcase   # String を返す
end
```

`"hi".upcase` の型は（Part 2/7 の表から）`String`。メソッドの戻り型は本体の最後の式の型
そのもの。だから合成できます。`type_of` に `def` を足します：

```ruby
when Prism::DefNode then type_of_def(node, scope, diagnostics)

def type_of_def(node, scope, diagnostics)
  method_return_type(node, scope, diagnostics)  # 本体を型検査（診断も集まる）
  Type::Const[node.name]                        # def 式の値はメソッド名シンボル
end

def method_return_type(node, scope, diagnostics)
  # 仮引数は untyped（本編は引数推論しない＝続編）
  body_scope = method_param_names(node).reduce(scope) { |s, n| s.with_local(n, Type::Dynamic.new) }
  type_of_body(node.body, body_scope, diagnostics)
end
```

これで **`def` の本体も型検査される**ようになりました（`check` が `def bad; 1 + "x"; end` の
中のエラーを拾う）。引数は `untyped` なので、`def ok(x); x + 1; end` は誤検知しません
（`untyped + Integer` は `:maybe` → 黙る）。

---

## 8-2. RBS 風に見せる

`annotate` は、`def` のときだけシグネチャ文字列を返します：

```ruby
def method_signature(node, scope, diagnostics)
  params = method_param_names(node).map { "untyped" }.join(", ")
  "def #{node.name}: (#{params}) -> #{method_return_type(node, scope, diagnostics)}"
end
```

```console
$ printf 'def greet\n  "hi".upcase\nend\n' | ruby exe/chibirigor annotate /dev/stdin
1: def greet: () -> String
```

`check` と `annotate` は**同じ推論器**（`type_of`/`method_return_type`）を使います。
推論が主役で、検査も表示もその副産物 ― これが Part 0 で言った「推論器」の姿です。

---

## 8-3. `untyped` がどこに出るか＝推論の弱点

引数を `untyped` にしているので、それが戻りまで流れると `untyped` が顔を出します：

```console
1: def double: (untyped) -> untyped     # n が untyped → n * 2 も untyped
1: def mystery: (untyped) -> untyped
```

この **`untyped` の出方そのものが「推論が型を見失った場所」** です。どこを直せば型が
通るようになるかが、ひと目で分かる。これは Rigor の `sig-gen`（RBS を生成する機能）の発想の
芽です ― 生成された RBS の `untyped` は「人間が型を足すべき場所」を指しています。

- **① 型理論**：本体から戻り型を合成する（注釈なしでも型が立つ）。
- **② Ruby/RBS**：メソッドに注釈は無いが、戻りは本体から分かることが多い。
- **③ Rigor 実装の問題**：合成した型を RBS 風に見せ、`untyped` で推論の穴を可視化する。

---

## 8-4. この章のまとめ

足したもの：`type_of` の `DefNode` 対応（本体検査＋戻り型合成）、`annotate` の `method_signature`。

この章の三題噺：

| | 内容 |
|---|---|
| ① 型理論（TAPL 22 章「型再構築」） | 本体から戻り型を合成（注釈ゼロでも型が立つ） |
| ② Ruby/RBS | メソッドに注釈は無いが戻りは本体から分かる |
| ③ Rigor 実装の問題 | RBS 風 sig で見せ、`untyped` で推論の弱点を可視化（sig-gen の芽） |

**続編に送ったもの**：

- **引数の推論**（本体での使われ方から `x` の型を当てる＝ capability / duck 推論）。本編は
  引数＝`untyped` 止まり。これが『しくみ』 9 章演習「正解を知らない」と言った前線（TAPL なら 22 章「型再構築」が型推論の本拠地）。
- 複数 `return`・early return・条件分岐をまたぐ戻り型の合流。
- 生成した RBS の書き出し（`erasure`／sig-gen 本体）。

**次章予告（Part 9・最終章）**：ここまでを `gradual` の哲学で締めます。`untyped` の伝播を
仕上げ、「chibirigor はわざと見逃すことで動くコードを脅かさない」を総括し、『しくみ』
『型システムのしくみ』のおわりに（gradual typing への伏線）と接続します。

---

> **検証メモ**
> - 「推論器」の手応え：`def greet: () -> String` のように、注釈ゼロから戻り型が立つのを
>   見せられた。これが『しくみ』（注釈必須の検査器）との一番の違い。○
> - FP 安全：引数 untyped なので本体の `x + 1` 等は `:maybe` で黙る。脅かさない。○
> - 複雑さ予算：新規は `type_of_def`/`method_return_type`/`method_param_names`/`method_signature`。
>   いずれも既存の `type_of_body` を使い回すだけ。○
> - 引数推論は続編に正しく逃がせた（本編の線引きどおり）。○
