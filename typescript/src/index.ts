import {
  verify as lib25519verify,
  sign,
  seed_from_phrase,
  gen_keypair,
  pubKey_from_pair_bytes
} from "ed25519-sigs";
import { getMnemonic } from "bip39-ts";

export { seed_from_phrase } from "ed25519-sigs";

export class KeyPair {
  bytes: Uint8Array;
  // privKey: Uint8Array;
  // pubKey: Uint8Array;
  // constructor(privKey: Uint8Array, pubKey: Uint8Array) {
  //   this.privKey = privKey;
  //   this.pubKey = pubKey;
  // }
  constructor(bytes: Uint8Array) {
    this.bytes = bytes;
  }

  static from_phrase(phrase: string): KeyPair {
    // let seed1 = utils.copyUint8Array(seed_from_phrase(phrase));
    // let seed2 = utils.copyUint8Array(seed_from_phrase(phrase));
    // let privKey = utils.copyUint8Array(gen_privKey(seed1));
    // let pubKey = utils.copyUint8Array(gen_pubKey(seed2));
    let kp_bytes: Uint8Array = utils.copyUint8Array(gen_keypair(phrase));
    return new KeyPair(kp_bytes);
  }

  pubKey(): Uint8Array {
    return utils.copyUint8Array(pubKey_from_pair_bytes(this.bytes));
  }

  // sign returns the signature only
  sign(message: Uint8Array): Uint8Array {
    return utils.copyUint8Array(sign(message, this.bytes));
  }

  verify(message: Uint8Array, signature: Uint8Array): boolean {
    return lib25519verify(message, this.pubKey(), signature);
  }
}

export function verify(
  message: Uint8Array,
  pubKey: Uint8Array,
  signature: Uint8Array
) {
  return lib25519verify(message, pubKey, signature);
}

export class SeedPhrase {
  phrase: string;
  constructor(phrase: string) {
    this.phrase = phrase;
  }

  static new_random(): SeedPhrase {
    let phrase: string = getMnemonic();
    return new SeedPhrase(phrase);
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
