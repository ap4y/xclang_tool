CLANG_LIB_DIR = /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib
RUSTC_ARGS = -L $(CLANG_LIB_DIR) -C link-args="-Wl,-rpath,$(CLANG_LIB_DIR)"

all: rclang

rclang:
	mkdir -p lib
	rustc --out-dir=lib src/rclang/lib.rs

xclang: rclang
	mkdir -p bin
	rustc -L lib $(RUSTC_ARGS) src/xclang.rs --out-dir=bin
	RUST_LOG=debug ./bin/xclang
docs:
	mkdir -p doc
	rustdoc -o doc src/lib.rs

test:
	mkdir -p tests
	rustc $(RUSTC_ARGS) --test src/rclang/lib.rs --out-dir=tests
	RUST_LOG=debug ./tests/rclang

clean:
	rm -rf bin
	rm -rf lib
	rm -rf tests
