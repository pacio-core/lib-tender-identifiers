#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use wasm_bindgen::prelude::*;
//
use crypto::ed25519;
use js_sys::Uint8Array;

// #[wasm_bindgen]
// pub extern "C" fn ab(seed: &[u8]) -> Uint8Array {
//     let array: Array = seed.into_iter().map(|x| JsValue::from(*x as u8)).collect();
//     let u8a = Uint8Array::new(&array);
//     u8a
// }

//

#[wasm_bindgen]
pub extern "C" fn gen_pubKey(seed: &[u8]) -> Uint8Array {
    let (_, pubKey) = ed25519::keypair(&seed);
    unsafe { Uint8Array::view(&pubKey) }
}
#[wasm_bindgen]
pub extern "C" fn gen_privKey(seed: &[u8]) -> Uint8Array {
    let (privKey, _) = ed25519::keypair(&seed);
    unsafe { Uint8Array::view(&privKey) }
}

//

#[cfg(test)]
mod tests {
    // use super::*;
    use crypto::ed25519;
    // use ed25519_dalek::{
    //     Keypair, PublicKey, Signature, KEYPAIR_LENGTH, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH,
    // };
    // use rand::rngs::StdRng;
    // use rand::SeedableRng;
    // use rand_chacha::ChaCha20Rng;
    // use rand_core::RngCore;
    // use std::convert::TryInto;

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

    type bErr = Box<dyn std::error::Error>;

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
