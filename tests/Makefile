ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/librust_robotstxt.$(EXT)
	g++ src/robotstxt_test.cpp -L ./target/debug/ -lrust_robotstxt -o run
	LD_LIBRARY_PATH=./target/debug/ ./run

target/debug/librust_robotstxt.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf run
