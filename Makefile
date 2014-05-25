# Root of the project
ROOT = $(dir $(firstword $(MAKEFILE_LIST)))

# Path to rustc executable
RUSTC ?= rustc

# Flags to pass rustc
RUSTC_FLAGS ?= -O

TARGET ?= $(ROOT)target

LIBHAMCREST = $(TARGET)/libhamcrest.timestamp

SRC = $(wildcard $(ROOT)src/hamcrest/*.rs) \
			$(wildcard $(ROOT)src/hamcrest/matchers/*.rs)

all: $(LIBHAMCREST)

$(TARGET):
	@mkdir -p $@

$(LIBHAMCREST): $(SRC) | $(TARGET)
	$(RUSTC) --crate-type=lib $(RUSTFLAGS) --out-dir target $(ROOT)src/hamcrest/lib.rs
	touch $(TARGET)

hamcrest-test: $(SRC) | $(TARGET)
	$(RUSTC) --test -o $(TARGET)/hamcrest-test $(ROOT)src/hamcrest/lib.rs

test: hamcrest-test | $(TARGET)
	TEST_EXISTS_FILE=$(ROOT)README.md \
	TEST_EXISTS_DIR=$(TARGET) \
	TEST_EXISTS_NONE=$(ROOT)zomg.txt \
	$(TARGET)/hamcrest-test

clean:
	rm -f $(TARGET)/libhamcrest*
	rm -f $(TARGET)/hamcrest-test

.PHONY: all test clean
