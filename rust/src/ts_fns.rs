use wasm_bindgen::prelude::*;
//
// use bip39::{Language, Mnemonic, MnemonicType, Seed};
use blake2::{Blake2b, Digest};
use crypto::ed25519;
use js_sys::Uint8Array;

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
#[wasm_bindgen]
pub extern "C" fn sign(message: &[u8], privKey: &[u8]) -> Uint8Array {
    let sig = ed25519::signature(message, privKey);
    unsafe { Uint8Array::view(&sig) }
}
#[wasm_bindgen]
pub extern "C" fn verify(message: &[u8], pubKey: &[u8], signature: &[u8]) -> bool {
    ed25519::verify(message, pubKey, signature)
}

// #[wasm_bindgen]
// pub extern "C" fn new_mnemophrase() -> String {
//     console_error_panic_hook::set_once();
//     let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
//     mnemonic.phrase().to_owned()
// }
#[wasm_bindgen]
pub extern "C" fn seed_from_phrase(phrase: String) -> Uint8Array {
    console_error_panic_hook::set_once();

    // let mnemo = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    // let mut hasher = Blake2b::new(64);
    // let bytes = phrase.as_bytes();
    let seed = Blake2b::digest(&phrase.as_bytes());

    // let seed = Seed::new(&phrase, "");
    unsafe { Uint8Array::view(&seed) }
}
