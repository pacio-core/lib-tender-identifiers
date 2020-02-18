// #[wasm_bindgen]
// pub struct KeyPairJS {
//     pub privKey: Bytes,
//     pub pubKey: Bytes,
// }

// #[wasm_bindgen]
// impl KeyPairJS {
//     pub fn privKey(&self) -> Uint8Array {
//         unsafe { Uint8Array::view(&self.privKey) }
//     }
//     pub fn pubKey(&self) -> Uint8Array {
//         self.pubKey
//     }
// }

// unsafe {
//     KeyPairJS {
//         // privKey: &Uint8Array::view(&privKey),
//         pubKey: &Uint8Array::view(&pubKey),
//     }
// }

// let pubArr: Array = pubKey.iter().map(|x| JsValue::from(*x as u8)).collect();
// let privArr: Array = privKey.iter().map(|x| JsValue::from(*x as u8)).collect();
// KeyPairJS {
//     privKey: &Uint8Array::new(&privArr),
//     pubKey: &Uint8Array::new(&pubArr),
// }
