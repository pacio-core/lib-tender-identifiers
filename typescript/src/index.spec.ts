import { KeyPair, SeedPhrase, utils, verify } from ".";
import { validateMnemonic } from "bip39-ts";

describe("gen_phrase", () => {
  it("gen_phrase", () => {
    let mnemo = SeedPhrase.new_random();
    expect(mnemo.phrase.split(" ").length).toBe(12);
    expect(validateMnemonic(mnemo.phrase)).toBe(true);
  });
});

describe("gen_keypair", () => {
  it("gen_keypair", () => {
    let phrase =
      "famous concert update chimney vicious repeat camp awful equal cash leisure stable";
    let kp = KeyPair.from_phrase(phrase);

    // prettier-ignore
    expect(kp.pubKey()).toEqual(new Uint8Array([
      73, 49, 171, 46, 173, 51, 219, 117, 97, 85, 196, 48, 227, 42, 201, 0, 38, 245, 250,
      186, 82, 194, 87, 85, 208, 148, 180, 231, 240, 204, 242, 90
    ]))
  });
});

// describe("sign and verify", () => {
//   it("signs and verifies", () => {
//     let phrase =
//       "famous concert update chimney vicious repeat camp awful equal cash leisure stable";
//     let kp = KeyPair.from_phrase(phrase);
//     let message: Uint8Array = utils.toUtf8("a short msg");

//     let sig = kp.sign(message);
//     expect(sig.length).toBe(64);

//     let isValid = kp.verify(message, sig);
//     expect(isValid).toBe(true);
//   });
// });
