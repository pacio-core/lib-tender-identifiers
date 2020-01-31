import { sign, signMessage, generateKeyPair, verify, str2u8a32 } from "./index";

describe("str2u8a32", () => {
  it("string to Uint8Array", () => {
    let inputStr = "correct horse battery staple";
    let u8a: Uint8Array = str2u8a32(inputStr);
    expect(u8a.slice(0, 5)).toEqual(Uint8Array.from([99, 111, 114, 114, 101]));
    expect(u8a.length).toBe(32);
  });
});

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
    let msg = str2u8a32("very message, many long, wow");
    let sig = sign(keyPair.private, msg);
    let verif = verify(keyPair.public, msg, sig);
    expect(verif).toBe(true);
  });
});

///
// HELPERS

function newKeyPair() {
  let seedString = "correct horse battery staple";
  let seedU8a: Uint8Array = str2u8a32(seedString);
  return generateKeyPair(seedU8a);
}

// function bytesToSring(bytes: Uint8Array) {
//   var chars = [];
//   for (var i = 0, n = bytes.length; i < n; ) {
//     chars.push(((bytes[i++] & 0xff) << 8) | (bytes[i++] & 0xff));
//   }
//   return String.fromCharCode.apply(null, chars);
// }

// function stringToBytes(str: string) {
//   var bytes = [];
//   for (var i = 0, n = str.length; i < n; i++) {
//     var char = str.charCodeAt(i);
//     bytes.push(char >>> 8, char & 0xff);
//   }
//   return bytes;
// }
