import { KeyPair, Mnemonic, utils, verify } from ".";
// import { gen_privKey, gen_pubKey, seed_from_phrase } from ".";
import { validateMnemonic } from "bip39-ts";

// describe("gen_phrase", () => {
//   it("gen_phrase", () => {
//     let mnemo = Mnemonic.new_random();
//     expect(mnemo.phrase.split(" ").length).toBe(12);
//     expect(validateMnemonic(mnemo.phrase)).toBe(true);
//   });
// });

describe("gen_keypair", () => {
  // it("gen_privKey", () => {
  //   let phrase =
  //     "famous concert update chimney vicious repeat camp awful equal cash leisure stable";
  //   let seed = seed_from_phrase(phrase);
  //   let privKey = gen_privKey(seed);

  //   // prettier-ignore
  //   expect(privKey).toEqual(
  //     Uint8Array.from([
  //       136,  43, 140, 164,  87,   8, 104,  39, 242, 182,  44,
  //       253, 236, 253, 115,  28, 152,  43,  56,  73,  78,  26,
  //         8, 248, 146,   1,  64,  92,  38, 169,  53, 217,  77,
  //       255,   1,  87,  32, 102,  70, 203,  73, 206, 134, 120,
  //       196, 243,  98, 188, 113,  87,  70, 251,   0,  89,  80,
  //        68, 155, 200, 146,  57, 221, 152,  21, 113
  //     ])
  //   );
  // });

  it("gen_keypair", () => {
    let phrase =
      "famous concert update chimney vicious repeat camp awful equal cash leisure stable";
    let kp = KeyPair.from_phrase(phrase);
    console.log({ kp });

    // expect(kp.privKey.length).toBe(64);
    // expect(kp.pubKey.length).toBe(32);
    // prettier-ignore
    // expect(kp.privKey).toEqual(new Uint8Array([
    //   136,  43, 140, 164,  87,   8, 104,  39, 242, 182,  44,
    //   253, 236, 253, 115,  28, 152,  43,  56,  73,  78,  26,
    //     8, 248, 146,   1,  64,  92,  38, 169,  53, 217,  77,
    //   255,   1,  87,  32, 102,  70, 203,  73, 206, 134, 120,
    //   196, 243,  98, 188, 113,  87,  70, 251,   0,  89,  80,
    //    68, 155, 200, 146,  57, 221, 152,  21, 113
    // ]));
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
