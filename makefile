.DEFAULT_GOAL=a.build
test: deps
	cargo test -- --nocapture
	node node_modules/.bin/jest
w.build: deps
	rm -rf typescript/libs/ed25519**
	wasm-pack --verbose build rust --out-name index --out-dir ../typescript/libs/ed25519xp

ts.build: deps
	node node_modules/.bin/webpack
a.build: deps $(eval min_ver=28) $(eval jniLibs=./android/ed25519lib/src/main/jniLibs) $(eval libName=libed25519xp.so)
	cargo ndk --target aarch64-linux-android --android-platform ${min_ver} -- build --release
	cargo ndk --target armv7-linux-androideabi --android-platform ${min_ver} -- build --release
	cargo ndk --target i686-linux-android --android-platform ${min_ver} -- build --release
	cargo ndk --target x86_64-linux-android --android-platform ${min_ver} -- build --release
	#
	rm -rf ${jniLibs} && mkdir -p ${jniLibs}/arm64-v8a ${jniLibs}/armeabi-v7a ${jniLibs}/x86 ${jniLibs}/x86_64
	#
	@cp target/aarch64-linux-android/release/${libName} ${jniLibs}/arm64-v8a/${libName}
	@cp target/armv7-linux-androideabi/release/${libName} ${jniLibs}/armeabi-v7a/${libName}
	@cp target/i686-linux-android/release/${libName} ${jniLibs}/x86/${libName}
	@cp target/x86_64-linux-android/release/${libName} ${jniLibs}/x86_64/${libName}

# DEPS
deps: install-rust
	@rustc --version | grep -E 'nightly.*2019-12-14' $s || rustup override set nightly-2019-12-14
	@cargo ndk --version | grep 0.4.1 $s || cargo install cargo-ndk --version 0.4.1
	-@rustup target add wasm32-unknown-unknown aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386 $s
	@test -d node_modules || npm install
install-rust: 		# install manually: build-essential, pkg-config
	@rustup --version $s || curl https://sh.rustup.rs -sSf | sh -s -- -y
s = 2>&1 >/dev/null

rm.cache:
	rm -rf package-lock.json node_modules/ target/ rust/target/
