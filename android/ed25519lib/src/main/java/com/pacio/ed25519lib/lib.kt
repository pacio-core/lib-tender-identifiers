package com.pacio.ed25519lib

external fun keypairFromPhrase(phrase_utf8: String): ByteArray
external fun pubKeyFromPairBytes(keypair: ByteArray): ByteArray
external fun sign(message: ByteArray, keypair: ByteArray): ByteArray
external fun verify(message:ByteArray, pubKey: ByteArray, signature: ByteArray): Boolean
external fun seedFromPhrase(phrase_utf8: String): ByteArray

fun loadLibEd25519() {
    System.loadLibrary("ed25519xp")
}