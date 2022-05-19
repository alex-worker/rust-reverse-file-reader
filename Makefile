run: build
	./rust-end-file-reader 5 ./fixtures/logfile.txt

test:
	cargo test

test-verbose:
	cargo test -- --show-output

build:
	cargo build --release
	cp ./target/release/rust-end-file-reader ./

run-example: build
	./rust-end-file-reader 5 ./fixtures/logfile.txt
