
test:
	cargo test

test-verbose:
	cargo test -- --show-output

build:
	cargo build --release
	cp ./target/release/rust-end-file-reader ./

run:
	./rust-end-file-reader
