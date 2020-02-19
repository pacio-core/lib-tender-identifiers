"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const _1 = require(".");
const bip39_ts_1 = require("bip39-ts");
describe("gen_phrase", () => {
    it("gen_phrase", () => {
        let mnemo = _1.SeedPhrase.new_random();
        expect(mnemo.phrase.split(" ").length).toBe(12);
        expect(bip39_ts_1.validateMnemonic(mnemo.phrase)).toBe(true);
    });
});
describe("gen_keypair", () => {
    it("gen_keypair", () => {
        let phrase = "famous concert update chimney vicious repeat camp awful equal cash leisure stable";
        let kp = _1.KeyPair.from_phrase(phrase);
        expect(kp.pubKey()).toEqual(new Uint8Array([
            73, 49, 171, 46, 173, 51, 219, 117, 97, 85, 196, 48, 227, 42, 201, 0, 38, 245, 250,
            186, 82, 194, 87, 85, 208, 148, 180, 231, 240, 204, 242, 90
        ]));
        expect(kp.to_bytes()).toEqual(new Uint8Array([
            136, 43, 140, 164, 87, 8, 104, 39, 242, 182, 44,
            253, 236, 253, 115, 28, 152, 43, 56, 73, 78, 26,
            8, 248, 146, 1, 64, 92, 38, 169, 53, 217, 73,
            49, 171, 46, 173, 51, 219, 117, 97, 85, 196, 48,
            227, 42, 201, 0, 38, 245, 250, 186, 82, 194, 87,
            85, 208, 148, 180, 231, 240, 204, 242, 90
        ]));
    });
});
describe("sign and verify", () => {
    it("signs and verifies", () => {
        let phrase = "famous concert update chimney vicious repeat camp awful equal cash leisure stable";
        let kp = _1.KeyPair.from_phrase(phrase);
        let message = _1.utils.toUtf8("A short msg");
        let sig = kp.sign(message);
        expect(sig.length).toBe(64);
        expect(sig).toEqual(new Uint8Array([
            146, 245, 180, 26, 7, 213, 47, 135, 129, 130, 210, 72, 55, 84, 111, 162, 191, 204, 89,
            164, 62, 160, 4, 208, 106, 157, 198, 129, 217, 23, 73, 206, 178, 105, 25, 40, 235, 18,
            18, 49, 13, 69, 11, 163, 148, 250, 102, 70, 81, 58, 88, 101, 29, 132, 64, 206, 0, 234,
            249, 102, 106, 153, 46, 0,
        ]));
        let isValid = kp.verify(message, sig);
        expect(isValid).toBe(true);
    });
});
describe("ser and deser", () => {
    it("ser and deser", () => {
        let phrase = "famous concert update chimney vicious repeat camp awful equal cash leisure stable";
        let kp = _1.KeyPair.from_phrase(phrase);
        let ser = kp.to_bytes();
        expect(ser.length).toBe(64);
        let deser = _1.KeyPair.from_bytes(ser);
        expect(deser).toEqual(kp);
    });
});
