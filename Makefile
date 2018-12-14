build:
	cargo +nightly build --target wasm32-unknown-unknown --release
	mkdir -p dist
	wasm-bindgen ./target/wasm32-unknown-unknown/release/node_lcs_img_diff.wasm --out-dir ./dist --nodejs

