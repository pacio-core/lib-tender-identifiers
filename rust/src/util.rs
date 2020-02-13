use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Bytes {
    offset: *const u8,
    size: usize,
}

#[wasm_bindgen]
impl Bytes {
    pub fn new(bytes: &[u8]) -> Bytes {
        Bytes {
            offset: bytes.as_ptr(),
            size: bytes.len(),
        }
    }

    pub fn offset(&self) -> *const u8 {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.size
    }    // pub fn u8Arr(&self) -> Uint8Array {
    //     use std::convert::TryInto;
    //     Uint8Array::new_with_byte_offset_and_length(
    //         &JsValue::NULL,
    //         (*self.offset()).into(),
    //         self.size().try_into().unwrap(),
    //     )
    // }
}
