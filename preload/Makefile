.PHONY: build test clean

UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
    LIB_NAME = libpreload.so
    PRELOAD_VAR = LD_PRELOAD
    TEST_CMD = $(PRELOAD_VAR)=$(shell pwd)/../target/release/$(LIB_NAME) ../target/release/test_open
else ifeq ($(UNAME), Darwin)
    LIB_NAME = libpreload.dylib
    PRELOAD_VAR = DYLD_INSERT_LIBRARIES
    TEST_CMD = $(PRELOAD_VAR)=$(shell pwd)/../target/release/$(LIB_NAME) DYLD_FORCE_FLAT_NAMESPACE=1 ../target/release/test_open
endif

build:
	cargo build --release --bin test_open
	@if [ "$(UNAME)" = "Darwin" ] && [ -f ../target/release/libpreload.so ]; then \
		mv ../target/release/libpreload.so ../target/release/libpreload.dylib; \
	fi

test: build
	@echo "Platform: $(UNAME)"
	@echo "Library: $(LIB_NAME)"
	mkdir -p /tmp/test/{node_order_statuses_by_block,node_raw_book_diffs_by_block,node_fills_by_block}
	$(TEST_CMD)

clean:
	cargo clean
	rm -rf /tmp/test