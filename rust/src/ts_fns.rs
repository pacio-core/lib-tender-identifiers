use wasm_bindgen::prelude::*;
//
use super::{KeyPair, PublicKey, SeedPhrase, Signature};
use js_sys::Uint8Array;

// #[wasm_bindgen]
// pub extern "C" fn gen_pubKey(seed: &[u8]) -> Uint8Array {
//     let (_, pubKey) = ed25519::keypair(&seed);
//     unsafe { Uint8Array::view(&pubKey) }
// }
// #[wasm_bindgen]
// pub extern "C" fn gen_privKey(seed: &[u8]) -> Uint8Array {
//     let (privKey, _) = ed25519::keypair(&seed);
//     unsafe { Uint8Array::view(&privKey) }
// }
#[wasm_bindgen]
pub extern "C" fn gen_keypair(phrase: &str) -> Uint8Array {
    let kp = KeyPair::from_phrase(phrase);
    unsafe { Uint8Array::view(&kp.to_bytes()) }
}
#[wasm_bindgen]
pub extern "C" fn sign(message: &[u8], keypair_bytes: &[u8]) -> Uint8Array {
    let kp = KeyPair::from_bytes(keypair_bytes);
    let sig_bytes = kp.sign(&message);
    unsafe { Uint8Array::view(&sig_bytes) }
}
#[wasm_bindgen]
pub extern "C" fn verify(message: &[u8], pubKey_bytes: &[u8], sig_bytes: &[u8]) -> bool {
    console_error_panic_hook::set_once();

    let pubKey = PublicKey::from_bytes(pubKey_bytes).unwrap();
    let sig = Signature::from_bytes(&sig_bytes).unwrap();
    pubKey.verify(&message, &sig).is_ok()
}

#[wasm_bindgen]
pub extern "C" fn seed_from_phrase(phrase: String) -> Uint8Array {
    console_error_panic_hook::set_once();

    let seed = SeedPhrase::from_str(&phrase).into_seed();

    unsafe { Uint8Array::view(&seed) }
}
