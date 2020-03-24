# pledges-sig-lib-ts

Curve25519 signatures library for Rust / Typescript / Android / iOS.

### Prerequisites

- On mac/linux, make sure you have the packages `build-essentials`, `pkg-config` installed on your system (for Rust).
- In order to build for iOS, you need a mac.

Other dependencies will be installed automatically (when you run the makefile commands)

## Usage

### Rust

<details>
  <summary>Installation</summary>
  <p>
  In your project's `Cargo.toml`, add the following line under `[dependencies]`:

```toml
ed25519xp = { git="https://github.com/pacio-core/pledges-sig-lib" }
```

  </p>
</details>

<details>
  <summary>API</summary>
  <p> 
    See docs.rs
  </p>
</details>

### Typescript

<details>
  <summary>Installation</summary>
  <p>Copy `android/ed25519lib/` into your android project</p>
</details>

<details>
  <summary>API</summary>
  <p> 
    <li>generateKeyPair(seed) -> KeyPair</li>
    <li>sign(privateKey, message, [random]) -> Signature</li>
    <li>verify(publicKey, message, signature) -> true | false</li>
    <li>serializeKeyPair(keyPair) -> String</li>
    <li>deserializeKeyPair(String) -> KeyPair</li>
  </p>
</details>

### Android

<details>
  <summary>Installation</summary>
  <p>Copy `android/ed25519lib` into your android project</p>
</details>

<details>
  <summary>API</summary>
  <ul>
    <li>keypair_from_phrase(phrase_utf8: JString) -> (keyPair: JByteArray)</li>
    <li>pubKey_from_pair_bytes(keypair: JByteArray) -> (pubKey: JByteArray)</li>
    <li>sign(message: JByteArray, keypair: JByteArray) -> (signature: JByteArray)</li>
    <li>verify(message: JByteArray, pubKey: JByteArray, sig: JByteArray) -> (isValid: boolean)</li>
    <li>seed_from_phrase(phrase_utf8: JString) -> (seed_bytes: JByteArray)</li>
  </ul>
</details>

### iOS

<details>
  <summary>Installation</summary>
  <ul>
    <li>copy `ios/libs/` and `ios/include/` into the top of you folder</li>
    <li>
    In Xcode, in your project settings -> General -> Frameworks, libraries, and embedded content, <br/>
        add the file `ios/libs/libed25519xp.a` (if it doesn't appear add it a second time)
    </li>
    <li>
        In Xcode, in your project settings -> Build Settings, <br/>
        <ul>
            <li>set `Header Search Paths` to `../include`</li>
            <li>set `Library Search Paths` to `../libs`</li>
            <li>set `Objective-C Bridging Header` to `../include`</li>
        </ul>
    </li>

  </ul>
</details>

<details>
  <summary>API</summary>
  <ul>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> (keyPair: RustByteSlice)</li>
    <li>pubKey_from_pair_bytes(keypair: RustByteSlice) -> (pubKey: RustByteSlice)</li>
    <li>sign(message: RustByteSlice, keypair: RustByteSlice) -> (signature: RustByteSlice)</li>
    <li>verify(message: RustByteSlice, pubKey: RustByteSlice, sig: RustByteSlice) -> (isValid: bool)</li>
  </ul>
</details>

### Building from source

#### for typescript, run

```shell
make ts.build
```

#### for android, run

```shell
make a.build
```

#### for ios, run

```shell
make ios.build
```
