CLANG_LIB_DIR = /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib

all: rclang

rclang:
	mkdir -p lib
	rustc --out-dir=lib src/lib.rs

docs:
	mkdir -p doc
	rustdoc -o doc src/lib.rs

test:
	mkdir -p tests
	rustc -L $(CLANG_LIB_DIR) --test src/lib.rs --out-dir=tests
	RUST_LOG=debug DYLD_LIBRARY_PATH=$(CLANG_LIB_DIR) ./tests/rclang:0.1


clean:
	rm -rf bin
