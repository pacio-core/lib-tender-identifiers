import {
  sign,
  signMessage,
  generateKeyPair,
  verify,
  str2u8a32,
  u8a2str,
  str2u8a,
  KeyPair,
  newMnemonic,
  toSeed
} from "./index";
import { validateMnemonic } from "bip39-ts";

describe("generateKeyPair", () => {
  it("generates a valid keyPair", () => {
    let seedString = "correct horse battery staple";
    let seedU8a: Uint8Array = str2u8a32(seedString);
    let keyPair = generateKeyPair(seedU8a);

    expect(keyPair.public.length).toEqual(32);
    expect(keyPair.private.length).toEqual(32);
    expect(keyPair.public.slice(0, 5)).toEqual(
      Uint8Array.from([139, 234, 20, 123, 234])
    );
    expect(keyPair.private.slice(0, 5)).toEqual(
      Uint8Array.from([96, 111, 114, 114, 101])
    );
  });
});
describe("sign and verify", () => {
  it("signs a message and verifies", () => {
    let keyPair = newKeyPair();
    let msg = str2u8a("very message, many short, wow");
    let sig = sign(keyPair.private, msg);
    let verif = verify(keyPair.public, msg, sig);

    expect(msg.length).toBe(29);
    expect(sig.length).toBe(64);
    expect(verif).toBe(true);
  });
  it("signs a long message and verifies", () => {
    let keyPair = newKeyPair();
    let msg = str2u8a(
      "very message, many loooooooooooooooooooooooooooooooooong, so more than 32 bytes, wow"
    );
    let sig = sign(keyPair.private, msg);
    let verif = verify(keyPair.public, msg, sig);

    expect(msg.length).toBe(84);
    expect(sig.length).toBe(64);
    expect(verif).toBe(true);
  });
});
describe("signMessage and verify", () => {
  it("signs a message and verifies", () => {
    let keyPair = newKeyPair();
    let msg = str2u8a32("very message, many short, wow");
    let signedMessage = signMessage(keyPair.private, msg);
    let verif = verify(keyPair.public, msg, signedMessage.slice(0, 64));
    expect(verif).toBe(true);
  });
});

describe("serialize, deserialize KeyPair", () => {
  it("serializes then deserializes a KeyPair", () => {
    let keyPair = newKeyPair();
    let ser = keyPair.ser();

    expect(ser.length).toBe(64);

    let deser = KeyPair.fromString(ser);

    expect(deser.private.length).toBe(32);
    expect(deser.public.length).toBe(32);
    expect(deser.private).toEqual(keyPair.private);
    expect(deser.public).toEqual(keyPair.public);
  });
});

describe("newMnemonic", () => {
  it("generates a valid new mnemonic", () => {
    let mnem = newMnemonic();
    expect(validateMnemonic(mnem)).toBe(true);
  });
});

describe("toSeed", () => {
  it("transforms a string to the right seed", () => {
    let inputStr = "correct horse battery staple pad";
    let seed = toSeed(inputStr);
    // console.log({ seed });
    expect(seed).toEqual(seed);
  });
});

////////////////////////
// HELPERS

function newKeyPair() {
  let seedString = "correct horse battery staple";
  let seedU8a: Uint8Array = str2u8a32(seedString);
  return generateKeyPair(seedU8a);
}

describe("str2u8a", () => {
  it("string to Uint8Array and back", () => {
    let inputStr =
      "very message, many loooooooooooooooooooooooooooooooooong, so more than 32 bytes, wow";
    let u8a: Uint8Array = str2u8a(inputStr);

    expect(u8a.length).toEqual(inputStr.length);
    expect(u8a.slice(0, 5)).toEqual(Uint8Array.from([118, 101, 114, 121, 32]));

    let outStr = u8a2str(u8a);

    expect(outStr.length).toEqual(inputStr.length);
    expect(outStr).toEqual(inputStr);
  });
});
describe("str2u8a32", () => {
  it("string to Uint8Array", () => {
    let inputStr = "correct horse battery staple";
    let u8a: Uint8Array = str2u8a32(inputStr);

    expect(u8a.length).toBe(32);
    expect(u8a.slice(0, 5)).toEqual(Uint8Array.from([99, 111, 114, 114, 101]));
  });
});
