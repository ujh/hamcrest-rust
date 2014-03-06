RUSTC ?= rustc
RUSTFLAGS ?= -O
TARGET = target/$(shell rustc --crate-file-name --crate-type=lib src/hamcrest/lib.rs)

SRC = $(wildcard src/hamcrest/*.rs)

all: $(TARGET)

$(TARGET): $(SRC)
	mkdir -p target
	$(RUSTC) --crate-type=lib $(RUSTFLAGS) --out-dir target $(SRC)

hamcrest-test: $(SRC)
	$(RUSTC) --test -o hamcrest-test $(SRC)

test: hamcrest-test
	./hamcrest-test

clean:
	rm -rf target
	rm hamcrest-test

.PHONY: all test clean
