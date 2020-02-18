test:
	cargo test -- --nocapture
	# yarn test
build-w:
	# cargo build --target wasm32-unknown-unknown --release
	# wasm-gc target/wasm32-unknown-unknown/release/sig_lib.wasm -o .cache/sig_lib.min.wasm
	# wasm2js .cache/sig_lib.min.wasm -o .cache/sig_lib.min.js
	wasm-pack build rust --target no-modules --out-dir ../.cache/ed25519-sigs/dist/no-modules/
	wasm-pack build rust --target browser --out-dir ../.cache/ed25519-sigs/dist/browser/
	wasm-pack build rust --target nodejs --out-dir ../.cache/ed25519-sigs/dist/node/
	sh copy.sh
w.test: build-w
	npx jest typescript/deps.spec.ts
