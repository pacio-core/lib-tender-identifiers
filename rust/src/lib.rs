#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//
use bip39::{Language, Mnemonic as Bip39Mnemo, MnemonicType, Seed};
use blake2::{Blake2b, Digest};
use crypto::ed25519;
use js_sys::Uint8Array;

mod ts_fns;

pub struct KeyPair {
    pub privKey: [u8; 64],
    pub pubKey: [u8; 32],
}
impl KeyPair {
    pub fn from_phrase(phrase: &str) -> Self {
        // let mnemo = Bip39Mnemo::from_phrase(mnemoPhrase, Language::English).unwrap();
        // let seed = Seed::new(&mnemo, "");
        let seed = Blake2b::digest(phrase.as_bytes());
        dbg!(seed);
        let (privKey, pubKey) = ed25519::keypair(&seed);
        KeyPair { privKey, pubKey }
    }

    pub fn sign(message: &[u8], privKey: &[u8]) -> [u8; 64] {
        ed25519::signature(message, privKey)
    }
    pub fn verify(message: &[u8], pubKey: &[u8], signature: &[u8]) -> bool {
        ed25519::verify(message, pubKey, signature)
    }
}

pub fn verify(message: &[u8], pubKey: &[u8], signature: &[u8]) -> bool {
    ed25519::verify(message, pubKey, signature)
}

pub struct SeedPhrase(String);
impl SeedPhrase {
    // pub fn from_str(s: &str) -> Self {
    //     return Self(s.to_owned());
    // }
    pub fn new_random() -> Self {
        let mnemonic = Bip39Mnemo::new(MnemonicType::Words12, Language::English);
        Self(mnemonic.phrase().to_owned())
    }
    pub fn to_string(&self) -> String {
        self.0.to_owned()
    }
    // pub fn into_seed(&self) -> [u8; 64] {
    //     let seed: &[u8] = &Blake2b::digest(&self.0.as_bytes());
    //     let seed2: &[u8; 64] = seed.slice();
    // }
}

//

#[cfg(test)]
mod tests {
    use super::KeyPair;

    #[test]
    fn test_gen_keypair() {
        let phrase = String::from(
            "famous concert update chimney vicious repeat camp awful equal cash leisure stable",
        );
        // let seed = [0u8, 0, 1, 1, 2, 2];
        // let pubKey = gen_keypair(&seed);
        // let (privKey, pubKey) = ed25519::keypair(&seed);
        let kp = KeyPair::from_phrase(&phrase);
        // let pub64 = base64::encode(&pubKey);
        dbg!(kp.pubKey.as_ref());
    }

    #[test]
    fn test_gen_keypair() {
        let seed = [0u8, 0, 1, 1, 2, 2];
        // let pubKey = gen_keypair(&seed);
        let (privKey, pubKey) = ed25519::keypair(&seed);
        // let pub64 = base64::encode(&pubKey);
        dbg!(pubKey);
    }

    // fn seed_rng() -> ChaCha20Rng {
    //     // let seed: &[u8] = &[1, 2, 3, 4,5,6,7,8,9,10];
    //     let seed_str: [u8; 32] = "correct horse battery staple pad"
    //         .as_bytes()
    //         .try_into()
    //         .expect("slice with incorrect length");
    //     let rng = ChaCha20Rng::from_seed(seed_str);
    //     // let rng = Rng(ChaCha::new(&seed, &[0u8; 8]));
    //     // let mut rng = StdRng::from_seed(SeedableRng::seed_from_u64(&mut 1))
    //     rng
    // }

    // #[test]
    // fn fill_bytes_seedrng() {
    //     let mut rng = seed_rng();
    //     let mut res = [0u8; 32];
    //     rng.fill_bytes(&mut res);

    //     // dbg!(res);
    // }

    // #[test]
    // fn scrypt_js() {
    //     let default_scrypt_params = crypto::scrypt::ScryptParams::new(14, 8, 1);
    //     let mut out = [0u8; 32];
    //     let pass = "correct horse battery staple".as_bytes();
    //     let salt = [0u8; 1];

    //     crypto::scrypt::scrypt(pass, &salt, &default_scrypt_params, &mut out);

    //     dbg!(out);
    // }

    // #[test]
    // fn signs_and_verifies() {
    //     let mut rng = seed_rng();
    //     let keypair: Keypair = Keypair::generate(&mut rng);

    //     let message: &[u8] = b"This is a test of the tsunami alert system.";
    //     let signature: Signature = keypair.sign(message);
    //     assert!(keypair.verify(message, &signature).is_ok());

    //     let public_key: PublicKey = keypair.public;
    //     assert!(public_key.verify(message, &signature).is_ok());
    // }
    // #[test]
    // fn ser_deser() -> Result<(), bErr> {
    //     let mut rng = seed_rng();
    //     let keypair: Keypair = Keypair::generate(&mut rng);

    //     let keypair_bytes: [u8; KEYPAIR_LENGTH] = keypair.to_bytes();
    //     assert_eq!(keypair_bytes.len(), 64);

    //     let got_keypair = Keypair::from_bytes(&keypair_bytes)?;
    //     assert_eq!(got_keypair.public, keypair.public);
    //     assert_eq!(got_keypair.secret.to_bytes(), keypair.secret.to_bytes());

    //     Ok(())
    // }

    // #[test]
    // fn key_sigs_length() {
    //     assert_eq!(PUBLIC_KEY_LENGTH, 32);
    //     assert_eq!(SECRET_KEY_LENGTH, 32);

    //     let keypair = new_keypair();
    //     assert_eq!(keypair.public.to_bytes().len(), 32);
    //     assert_eq!(keypair.secret.to_bytes().len(), 32);

    //     let message: &[u8] = b"This is a test of the tsunami alert system.";
    //     let sig: Signature = keypair.sign(message);
    //     assert!(keypair.verify(message, &sig).is_ok());
    //     assert_eq!(sig.to_bytes().len(), 64);

    //     // dbg!(kp);
    // }

    //

    //////////////////
    // HELPERS

    // fn new_keypair() -> Keypair {
    //     // let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    //     // let mut rng = ReadRng::new(&data[..]);
    //     // println!("{:x}", rng.gen::<u32>());

    //     let mut rng = seed_rng();
    //     let keypair: Keypair = Keypair::generate(&mut rng);
    //     // dbg!(keypair.public);
    //     keypair
    // }
}
