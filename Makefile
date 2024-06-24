clean:
	rm -rf ./dist
	mkdir ./dist

build:
	cargo build --release
	cp ./target/release/amm-rust ./dist/amm-rust
	upx -9 ./dist/amm-rust
