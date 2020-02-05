import axlsign from "axlsign";
import { getMnemonic, toSeed, validateMnemonic } from "bip39-ts";

export type Bytes = Uint8Array;
export type Msg = Uint8Array;
export type PubKey = Uint8Array;
export type PrivKey = Uint8Array;
export type Seed = Uint8Array;
export type Sig = Uint8Array;
export type SignedMsg = Uint8Array;

export class KeyPair {
  private: PrivKey; // 32-byte private key
  public: PubKey; // 32-byte public key

  public constructor(privKey: PrivKey, pubKey: PubKey) {
    this.private = privKey;
    this.public = pubKey;
  }

  public ser(): string {
    return u8a2str(this.private) + u8a2str(this.public);
  }

  static fromString(str: string): KeyPair {
    let u8a = str2u8a(str);
    let [privKey, pubKey] = [u8a.slice(0, 32), u8a.slice(32, 64)];
    return new KeyPair(privKey, pubKey);
  }
}

export function generateKeyPair(seed: Seed): KeyPair {
  let pair = axlsign.generateKeyPair(seed);
  return new KeyPair(pair.private, pair.public);
}

// returns the signature only
export function sign(prvKey: PrivKey, msg: Msg, rnd?: Bytes): Sig {
  return axlsign.sign(prvKey, msg, rnd);
}

// returns the signature concatenated with the message
export function signMessage(prvKey: PrivKey, msg: Msg, rnd?: Bytes): SignedMsg {
  return axlsign.signMessage(prvKey, msg, rnd);
}

export function verify(pubKey: PubKey, msg: Msg, sig: Sig): Boolean {
  return axlsign.verify(pubKey, msg, sig);
}

export function newMnemonic() {
  const mnemonic: string = getMnemonic();
  console.log(validateMnemonic(mnemonic) === true);
  const seed: Buffer = toSeed(mnemonic);
  // const seedHex: string = toSeedHex(mnemonic);
}

//

//////////////////////
// HELPERS

// interface U8a<L extends number> extends Uint8Array {
//   length: L;
// }

export function u8a2str(u8a: Uint8Array): string {
  return String.fromCharCode.apply(null, Array.from(u8a));
}
export function str2u8a(str: string): Uint8Array {
  const buf = new ArrayBuffer(str.length);
  const bufView = new Uint8Array(buf);
  for (let i = 0; i < str.length; i++) {
    bufView[i] = str.charCodeAt(i);
  }
  return bufView;
}
export function str2u8a32(str: string): Uint8Array {
  var buf = new ArrayBuffer(32);
  var bufView = new Uint8Array(buf);
  for (var i = 0, strLen = str.length; i < strLen; i++) {
    bufView[i] = str.charCodeAt(i);
  }
  return bufView;
}
