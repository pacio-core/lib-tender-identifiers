# pledges-sig-lib-ts
Curve25519 signatures library in Typescript

## Prerequisites

## Usage

### API

* generateKeyPair(seed) -> KeyPair
* sign(privateKey, message, [random]) -> Signature
* verify(publicKey, message, signature) -> true | false
* serializeKeyPair(keyPair) -> String
* deserializeKeyPair(String) -> KeyPair
