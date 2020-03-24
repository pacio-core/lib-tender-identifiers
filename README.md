# pledges-sig-lib-ts

Curve25519 signatures library for Rust / Typescript / Android / iOS.

## Prerequisites

## Usage

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
  <p>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
  </p>
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
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
    <li>keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice</li>
  </ul>
</details>

## Building from source

- for typescript, run

```shell
make ts.build
```

- for android, run

```shell
make a.build
```

- for ios, run

```shell
make ios.build
```
