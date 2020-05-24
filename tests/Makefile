ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/librust_robotstxt.$(EXT)
	g++ src/simple_test.cpp -L ./target/debug/ -lrust_robotstxt -o simple_test
	LD_LIBRARY_PATH=./target/debug/ ./simple_test

target/debug/librust_robotstxt.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf run
