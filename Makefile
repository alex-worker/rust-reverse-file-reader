
run:
	cargo build --release
	cp ./target/release/rust-end-file-reader ./
	./rust-end-file-reader
