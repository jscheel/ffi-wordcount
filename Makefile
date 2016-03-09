ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/release/libwordcount.$(EXT)
	ruby src/main.rb

target/release/libwordcount.$(EXT): src/lib.rs Cargo.toml
	cargo build --release

clean:
	rm -rf target
