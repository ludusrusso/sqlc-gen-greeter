build: build.rs src/codegen.proto 
	cargo build --release --target wasm32-wasi

pack:
	cp target/wasm32-wasi/release/sqlc-gen-crud.wasm ./crud.wasm
	cp target/wasm32-wasi/release/sqlc-gen-proto.wasm ./proto.wasm
	shasum -a 256 ./crud.wasm
	shasum -a 256 ./proto.wasm