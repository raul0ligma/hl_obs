.PHONY: build test clean

build:
	cargo build --release --bin test_open

test: build
	mkdir -p /tmp/test/{node_order_statuses_by_block,node_raw_book_diffs_by_block,node_fills_by_block}
	LD_PRELOAD=../target/release/libpreload.so ../target/release/test_open

clean:
	cargo clean
	rm -rf /tmp/test