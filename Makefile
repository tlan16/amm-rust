clean:
	rm -rf ./dist
	mkdir ./dist

build:
	cargo build --release
	cp ./target/release/amm-rust ./dist/amm-rust

install:
	cp ./dist/amm-rust /usr/local/bin/amm-rust
