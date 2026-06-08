# chibirigor — CI チェックリスト
# make          : 全チェック（テスト + ドリフト検出）
# make test     : lib テストのみ
# make drift    : 本文ドリフトチェックのみ
# make fix      : --fix モード（include ブロックを region から再生成）

.PHONY: all test drift fix

all: test drift

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
