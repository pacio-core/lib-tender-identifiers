package com.pacio.ed25519lib

external fun keypair_from_phrase(phrase_utf8: String): ByteArray
external fun pubKey_from_pair_bytes(keypair: ByteArray): ByteArray
external fun sign(message: ByteArray, keypair: ByteArray): ByteArray
external fun verify(message:ByteArray, pubKey: ByteArray, signature: ByteArray): Boolean
external fun seed_from_phrase(phrase_utf8: String): ByteArray

fun loadLibEd25519() {
    System.loadLibrary("ed25519xp")
}