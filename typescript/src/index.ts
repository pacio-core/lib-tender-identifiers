import {
  verification as lib25519verify,
  signature as sign,
  mnemonic_phrase_to_entropy as seed_from_phrase,
  random_new_keypair as gen_keypair,
  new_pubkey as pubKey_from_pair_bytes
} from "ed25519-sigs";
import { getMnemonic } from "bip39-ts";

export { mnemonic_phrase_to_entropy as seed_from_phrase } from "ed25519-sigs";

export class KeyPair {
  bytes: Uint8Array;
  constructor(bytes: Uint8Array) {
    this.bytes = bytes;
  }

  static from_phrase(phrase: string): KeyPair {
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
  // if valid signature, returns true
  verify(message: Uint8Array, signature: Uint8Array): boolean {
    return lib25519verify(message, this.pubKey(), signature);
  }

  to_bytes(): Uint8Array {
    return this.bytes;
  }
  static from_bytes(bytes: Uint8Array): KeyPair {
    return new KeyPair(bytes);
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

  // u8a2str(u8a: Uint8Array): string {
  //   return String.fromCharCode.apply(null, Array.from(u8a));
  // }
  // str2u8a(str: string): Uint8Array {
  //   let strLen = str.length;
  //   const buf = new ArrayBuffer(strLen);
  //   const bufView = new Uint8Array(buf);
  //   for (let i = 0; i < strLen; i++) {
  //     bufView[i] = str.charCodeAt(i);
  //   }
  //   return bufView;
  // }
  // str2u8a32(str: string): Uint8Array {
  //   var buf = new ArrayBuffer(32);
  //   var bufView = new Uint8Array(buf);
  //   for (var i = 0, strLen = str.length; i < strLen; i++) {
  //     bufView[i] = str.charCodeAt(i);
  //   }
  //   return bufView;
  // }
}
