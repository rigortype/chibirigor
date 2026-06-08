# impls ― 各章末の到達段階スナップショット

chibivue の [`book/impls`](https://github.com/chibivue-land/chibivue/tree/main/book/impls)
にならい、本文の各 Part を読み終えた時点で手元のコードが**どこまで来ているか**を、丸ごと動く
ツリーとして展開します。ただし chibivue が全コピーを手で維持するのに対し、ここは
**「変わったファイルだけ」を真実の源にして、完全ツリーは機械生成**します（事故＝ドリフトを減らすため）。

## 仕組み（単一ソース → 生成物 → 検証ゲート）

```
impls/
├── steps/            ← 真実の源（手で編集する）
│   ├── part1/
│   │   ├── lib/...           その章で *新規/置換した* ファイルだけ
│   │   └── test_stage.rb     その段の到達挙動を固定するスモークテスト
│   └── part2/
│       ├── lib/chibirigor/dispatch.rb   （新規）
│       ├── lib/chibirigor/type.rb       （置換：Type:: 名前空間へ）
│       ├── lib/chibirigor/type_of.rb    （置換：Dispatch へ委譲）
│       ├── lib/chibirigor.rb            （置換：require 追加）
│       └── test_stage.rb
└── dist/             ← 生成物（手で編集しない。閲覧用に commit する）
    ├── part1/lib/... 完全ツリー（part1 を読み終えた状態）
    └── part2/lib/... 完全ツリー（part1 + part2 の差分を重ねた状態）
```

`tools/gen_impls.rb` が `steps/part1..partN` の `lib/` を**順に重ねて**（同じパスは後の段が
上書き＝置換）、各到達段階の**完全ツリー**を `dist/partN/` に出力します。

```console
$ make impls          # dist を生成
$ make impls-verify   # 生成 + 各段の test_stage.rb を実行（段ごとの挙動が緑か）
$ make impls-check    # 生成し直して dist が手編集されていないか（steps と同期か）を検証
```

## なぜ「patch 直列」ではなく「ファイル単位の前方 compose」か

- **patch 直列**（`0N.patch` を累積 apply）は位置依存で、早い段の変更が後続 patch を
  カスケード衝突させる。本書は章を組み替える（reorder した実績がある）ので相性が悪い。
- **ファイル単位の前方 compose** は、章を入れ替えても**ディレクトリ名を振り直すだけ**。
  変更は「その段で変わったファイル」に局所化され、衝突しない。
- **最終 `lib/` を後ろから削る逆射影**は、最終 lib が全 Part 織り込み済み（`checker.rb` が
  baseline/rbs を、`annotator.rb` が sig 合成を既に持つ）なので困難。前方 compose なら、
  各段の*教えるコードそのもの*が源になる。

## 事故を減らす 3 点

1. **単一の真実**：各段で変わったファイルは `steps/` の 1 箇所だけ（N 個のコピーを手維持しない）。
2. **スナップショットは生成物**：`dist/` は機械生成。手で触らない（`make impls-check` が検出）。
3. **段ごとの検証ゲート**：`test_stage.rb` が「Part N の到達挙動」を固定（例：Part 1 は加算を
   **丸めて Integer**、Part 2 も dispatch 経由で Integer）。`make all` に組み込み済み。

## 本文との接続（次の一歩）

本文のコードブロックを `<!-- include: ../../impls/dist/partN/lib/... #region -->` で
**スナップショットから直接引く**ようにすれば、本文 ↔ スナップショット ↔ 段テストが
一つの源に束ねられ、ドリフトが原理的に消えます（既存の `check_docs.rb` と同じ発想を段に拡張）。

## 現状

- **Part 1・Part 2 の試作のみ**（仕組みの実証）。Part 3 以降は `steps/partN/` に
  「その章で変わったファイル＋ test_stage.rb」を足していけば、`make impls` が自動で
  完全ツリーを生成します。最終段は `lib/`（＝完成形）と一致させるのが目標。
