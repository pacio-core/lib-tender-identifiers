# pledges-sig-lib-ts
Curve25519 signatures library in Typescript

## Prerequisites

## Usage

### API

* generateKeyPair(seed) -> keyPair
* sign(privateKey, message, [random]) -> signature
* verify(publicKey, message, signature) -> true | false
* serializeKeyPair(keyPair) -> String
* deserializeKeyPair(String) -> keyPair
