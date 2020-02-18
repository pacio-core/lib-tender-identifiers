test:
	cargo test -- --nocapture
	# yarn test
build-w:
	sh copy.sh
w.test: build-w
	yarn test
