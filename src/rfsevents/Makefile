fsevents:
	clang -dynamiclib -std=gnu99 lib.c -o libRFSEvents.dylib -framework CoreServices

fsevents-test: fsevents
	clang test.c libRFSEvents.dylib -o test
	./test

rfsevents:
	rustc --out-dir=. lib.rs

rfsevents-test: fsevents rfsevents
	rustc -L . test.rs
	./test
