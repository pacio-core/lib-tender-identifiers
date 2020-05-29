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
	#
a.example.build:
	./ExampleAndroidApp/gradlew build
	rm -rf .gradle/
ios.build:
	mkdir -p ios/include && cbindgen rust/src/lib.rs -l c > ios/include/rustylib.h
	cargo lipo --release
	mkdir -p ios/libs && cp target/universal/release/libed25519xp.a ios/libs

# DEPS
deps: installs
	@rustc --version | grep -E 'nightly.*2020-05-07' $s || rustup override set nightly-2020-05-07
	@cargo ndk --version | grep 0.4.1 $s || cargo install cargo-ndk --version 0.4.1
	@cargo-lipo --version $s || cargo install cargo-lipo
	@cbindgen --version $s || cargo install cbindgen
	-@rustup target add wasm32-unknown-unknown aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386 $s
	@test -d node_modules || npm install
installs: 		# install manually: build-essential, pkg-config
	@rustup --version $s || curl https://sh.rustup.rs -sSf | sh -s -- -y
	@xcode-select --install $s || true
s = &>/dev/null

rm.cache:
	rm -rf package-lock.json node_modules/ target/ rust/target/
