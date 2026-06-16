# chibirigor — CI チェックリスト
# make          : 全チェック（テスト + ドリフト検出 + 段スナップショット検証）
# make test     : lib テストのみ
# make drift    : 本文ドリフトチェックのみ（draft 前後編 + v1 前編 + v2 前後編）
# make fix      : --fix モード（include ブロックを region から再生成）
# make impls        : 段スナップショット（impls/dist/partN）を steps から生成
# make impls-verify : 生成 ＋ 各段の test_stage.rb を実行
# make impls-check  : 生成し直して dist が steps と同期しているか（手編集されていないか）を検証
# make fmt          : 和欧境界スペースを詰める（book/v2/ja を書き換え）
# make fmt-check    : 和欧境界スペースの未整形を検出（書き換えず、あれば失敗）

.PHONY: all test drift fix impls impls-verify impls-check fmt fmt-check

all: test drift impls-verify

test:
	@echo "=== lib tests ==="
	@for f in test/test_*.rb; do ruby $$f || exit 1; done

drift:
	@echo "=== drift check: draft 前編 ==="
	ruby draft/little/ja/examples/check_docs.rb
	@echo "=== drift check: draft 後編 ==="
	ruby draft/seasoned/ja/examples/check_docs.rb
	@echo "=== drift check: v1 前編 ==="
	ruby book/v1/ja/little/examples/check_docs.rb
	@echo "=== drift check: v2 前編 ==="
	ruby book/v2/ja/little/examples/check_docs.rb
	@echo "=== drift check: v2 後編 ==="
	ruby book/v2/ja/seasoned/examples/check_docs.rb

fix:
	ruby draft/little/ja/examples/check_docs.rb --fix
	ruby draft/seasoned/ja/examples/check_docs.rb --fix
	ruby book/v1/ja/little/examples/check_docs.rb --fix
	ruby book/v2/ja/little/examples/check_docs.rb --fix
	ruby book/v2/ja/seasoned/examples/check_docs.rb --fix

impls:
	@echo "=== 段スナップショット生成 ==="
	ruby tools/gen_impls.rb

impls-verify:
	@echo "=== 段スナップショット生成 + 段テスト ==="
	ruby tools/gen_impls.rb --verify

impls-check: impls
	@echo "=== dist が steps と同期しているか（手編集検出）==="
	@git diff --exit-code -- impls/dist \
	  && echo "OK: impls/dist は steps から再生成された内容と一致" \
	  || (echo "NG: impls/dist が手編集されています。steps を直して make impls してください"; exit 1)

fmt:
	ruby tools/ja_format.rb --write book/v2/ja

fmt-check:
	ruby tools/ja_format.rb --check book/v2/ja
