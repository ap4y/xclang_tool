CLANG_LIB_DIR = /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib
RFSEVENTS_LIB_DIR = /Users/arthurevstifeev/github/xclang_tool/lib
RUSTC_ARGS = -L $(CLANG_LIB_DIR) -C link-args="-Wl,-rpath,$(CLANG_LIB_DIR)"

all: rclang rfsevents xclang

rclang:
	mkdir -p lib
	rustc --out-dir=lib src/rclang/lib.rs

rfsevents:
	mkdir -p lib
	clang -dynamiclib -std=gnu99 src/rfsevents/lib.c -current_version 1.0 -compatibility_version 1.0 -o $(RFSEVENTS_LIB_DIR)/libRFSEvents.dylib -framework CoreServices
	rustc -L lib --out-dir=lib src/rfsevents/lib.rs

xclang: rclang rfsevents
	mkdir -p bin
	rustc -L lib $(RUSTC_ARGS) src/xclang.rs --out-dir=bin

docs:
	mkdir -p doc
	rustdoc -o doc src/lib.rs

test:
	mkdir -p tests
	# rustc $(RUSTC_ARGS) --test src/rclang/lib.rs --out-dir=tests
	# RUST_LOG=debug ./tests/rclang
	rustc -L lib $(RUSTC_ARGS) --test src/xclang.rs --out-dir=tests
	RUST_LOG=debug ./tests/xclang

clean:
	rm -rf bin
	rm -rf lib
	rm -rf tests
