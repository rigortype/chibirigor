# chibirigor — CI チェックリスト
# make          : 全チェック（テスト + ドリフト検出 + 段スナップショット検証）
# make test     : lib テストのみ
# make drift    : 本文ドリフトチェックのみ
# make fix      : --fix モード（include ブロックを region から再生成）
# make impls        : 段スナップショット（impls/dist/partN）を steps から生成
# make impls-verify : 生成 ＋ 各段の test_stage.rb を実行
# make impls-check  : 生成し直して dist が steps と同期しているか（手編集されていないか）を検証

.PHONY: all test drift fix impls impls-verify impls-check

all: test drift impls-verify

test:
	@echo "=== lib tests ==="
	@for f in test/test_part*.rb test/test_plugin.rb; do ruby $$f; done

drift:
	@echo "=== drift check: 前編 ==="
	ruby draft/little/ja/examples/check_docs.rb
	@echo "=== drift check: 後編 ==="
	ruby draft/seasoned/ja/examples/check_docs.rb

fix:
	ruby draft/little/ja/examples/check_docs.rb --fix
	ruby draft/seasoned/ja/examples/check_docs.rb --fix

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
