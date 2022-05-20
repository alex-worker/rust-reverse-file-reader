default: run-example

build:
	cargo build --release
	cp ./target/release/rust-end-file-reader ./

run: build
	./rust-end-file-reader 5 ./fixtures/logfile.txt

clean:
	cargo clean
	rm ./rust-end-file-reader

test:
	cargo test

test-v:
	cargo test -- --show-output

run-example:
	cargo build
	./target/debug/rust-end-file-reader 5 ./fixtures/logfile_2.txt
