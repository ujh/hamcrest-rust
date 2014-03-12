RUSTC ?= rustc
RUSTFLAGS ?= -O
TARGET = target/timestamp

SRC = $(wildcard src/hamcrest/*.rs)

all: $(TARGET)

$(TARGET): $(SRC)
	mkdir -p target
	$(RUSTC) --crate-type=lib $(RUSTFLAGS) --out-dir target $(SRC)
	touch $(TARGET)

hamcrest-test: $(SRC)
	$(RUSTC) --test -o hamcrest-test $(SRC)

test: hamcrest-test
	./hamcrest-test

clean:
	rm -rf target
	rm -f hamcrest-test

.PHONY: all test clean
