import {
  sign as lib25519sign,
  verify as lib25519verify,
  seed_from_phrase,
  gen_privKey,
  gen_pubKey
} from "ed25519-sigs";
import { getMnemonic } from "bip39-ts";

export { gen_privKey, gen_pubKey, seed_from_phrase } from "ed25519-sigs";

export class KeyPair {
  privKey: Uint8Array;
  pubKey: Uint8Array;
  constructor(privKey: Uint8Array, pubKey: Uint8Array) {
    this.privKey = privKey;
    this.pubKey = pubKey;
  }

  static from_phrase(phrase: string): KeyPair {
    let seed1 = utils.copyUint8Array(seed_from_phrase(phrase));
    let seed2 = utils.copyUint8Array(seed_from_phrase(phrase));
    let privKey = utils.copyUint8Array(gen_privKey(seed1));
    let pubKey = utils.copyUint8Array(gen_pubKey(seed2));
    return new KeyPair(privKey, pubKey);
  }

  // sign returns the signature only
  sign(message: Uint8Array): Uint8Array {
    return utils.copyUint8Array(lib25519sign(message, this.privKey));
  }

  verify(message: Uint8Array, signature: Uint8Array): boolean {
    return lib25519verify(message, this.pubKey, signature);
  }
}

export function verify(
  message: Uint8Array,
  pubKey: Uint8Array,
  signature: Uint8Array
) {
  return lib25519verify(message, pubKey, signature);
}

export class Mnemonic {
  phrase: string;
  constructor(phrase: string) {
    this.phrase = phrase;
  }

  static new_random(): Mnemonic {
    let phrase: string = getMnemonic();
    return new Mnemonic(phrase);
  }

  into_seed(): Uint8Array {
    return utils.copyUint8Array(seed_from_phrase(this.phrase));
  }
}

export class utils {
  static toUtf8(data: string): Buffer {
    const nor: string = data.normalize("NFKD");
    return Buffer.from(nor, "utf8");
  }
  static copyUint8Array(src: Uint8Array): Uint8Array {
    let dst = new Uint8Array(src.length);
    dst.set(src);
    return dst;
  }
}
