import axlsign from "axlsign";

// export { sign, verify, generateKeyPair } from "axlsign";

export type Bytes = Uint8Array;
export type Msg = Uint8Array;
export type PubKey = Uint8Array;
export type PrivKey = Uint8Array;
export type Seed = Uint8Array;
export type Sig = Uint8Array;
export type SignedMsg = Uint8Array;

export interface KeyPair {
  private: PrivKey; // 32-byte private key
  public: PubKey; // 32-byte public key
}

export function generateKeyPair(seed: Seed): KeyPair {
  return axlsign.generateKeyPair(seed);
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

// export function sum(a: number, b: number) {
//   return a + b;
// }

// HELPERS

// function u8a2str(buf: ArrayBuffer) {
//   return String.fromCharCode.apply(null, new Uint8Array(buf).);
// }

interface U8a<L extends number> extends Uint8Array {
  length: L;
}

export function str2u8a32(str: string): Uint8Array {
  //   var buf = new ArrayBuffer(str.length * 2); // 2 bytes for each char
  var buf = new ArrayBuffer(32);
  var bufView = new Uint8Array(buf);
  for (var i = 0, strLen = str.length; i < strLen; i++) {
    bufView[i] = str.charCodeAt(i);
  }
  return bufView;
}
