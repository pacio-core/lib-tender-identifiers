"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const ed25519_sigs_1 = require("ed25519-sigs");
const bip39_ts_1 = require("bip39-ts");
var ed25519_sigs_2 = require("ed25519-sigs");
exports.seed_from_phrase = ed25519_sigs_2.mnemonic_phrase_to_entropy;
class KeyPair {
    constructor(bytes) {
        this.bytes = bytes;
    }
    static from_phrase(phrase) {
        let kp_bytes = utils.copyUint8Array(ed25519_sigs_1.random_new_keypair(phrase));
        return new KeyPair(kp_bytes);
    }
    pubKey() {
        return utils.copyUint8Array(ed25519_sigs_1.new_pubkey(this.bytes));
    }
    sign(message) {
        return utils.copyUint8Array(ed25519_sigs_1.signature(message, this.bytes));
    }
    verify(message, signature) {
        return ed25519_sigs_1.verification(message, this.pubKey(), signature);
    }
    to_bytes() {
        return this.bytes;
    }
    static from_bytes(bytes) {
        return new KeyPair(bytes);
    }
}
exports.KeyPair = KeyPair;
function verify(message, pubKey, signature) {
    return ed25519_sigs_1.verification(message, pubKey, signature);
}
exports.verify = verify;
class SeedPhrase {
    constructor(phrase) {
        this.phrase = phrase;
    }
    static new_random() {
        let phrase = bip39_ts_1.getMnemonic();
        return new SeedPhrase(phrase);
    }
    into_seed() {
        return utils.copyUint8Array(ed25519_sigs_1.mnemonic_phrase_to_entropy(this.phrase));
    }
}
exports.SeedPhrase = SeedPhrase;
class utils {
    static toUtf8(data) {
        const nor = data.normalize("NFKD");
        return Buffer.from(nor, "utf8");
    }
    static copyUint8Array(src) {
        let dst = new Uint8Array(src.length);
        dst.set(src);
        return dst;
    }
}
exports.utils = utils;
