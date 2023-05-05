build: build.rs src/codegen.proto src/main.rs
	cargo build --release --target wasm32-wasi

pack:
	cp target/wasm32-wasi/release/sqlc-gen-json-wasm.wasm ./res.wasm
	shasum -a 256 ./res.wasm