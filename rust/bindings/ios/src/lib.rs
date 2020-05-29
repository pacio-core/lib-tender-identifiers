#![cfg(target_os = "ios")]
// use c_vec::{CSlice, CVec};
use libc::size_t;
// use std::ffi::{CStr, CString};
// use std::os::raw::c_char;
use std::slice;
use std::str;
//
use crate::{KeyPair, PublicKey, Signature};

#[repr(C)]
pub struct RustByteSlice {
    pub start: *const u8,
    pub len: size_t,
}

#[no_mangle]
pub unsafe extern "C" fn keypair_from_phrase(phrase_utf8: RustByteSlice) -> RustByteSlice {
    let phrase_slice: &[u8] = slice::from_raw_parts(phrase_utf8.start, phrase_utf8.len as usize);
    let phrase_str = String::from(str::from_utf8(phrase_slice).unwrap()); //TODO no unwrap

    let kp_bytes = KeyPair::from_phrase(&phrase_str).to_bytes();

    RustByteSlice {
        start: kp_bytes.as_ptr(),
        len: kp_bytes.len() as size_t,
    }
}

#[no_mangle]
pub unsafe extern "C" fn pubKey_from_pair_bytes(keypair: RustByteSlice) -> RustByteSlice {
    let keypair_slice: &[u8] = slice::from_raw_parts(keypair.start, keypair.len as usize);
    let pubKey_bytes = KeyPair::from_bytes(keypair_slice).pubkey().to_bytes();

    RustByteSlice {
        start: pubKey_bytes.as_ptr(),
        len: pubKey_bytes.len() as size_t,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sign(message: RustByteSlice, keypair: RustByteSlice) -> RustByteSlice {
    let kp = KeyPair::from_bytes(slice::from_raw_parts(keypair.start, keypair.len as usize));
    let sig_bytes = kp.sign(slice::from_raw_parts(message.start, message.len as usize));

    RustByteSlice {
        start: sig_bytes.as_ptr(),
        len: sig_bytes.len() as size_t,
    }
}

#[no_mangle]
pub unsafe extern "C" fn verify(
    message: RustByteSlice,
    pubKey: RustByteSlice,
    sig: RustByteSlice,
) -> bool {
    let message_slice: &[u8] = slice::from_raw_parts(message.start, message.len as usize);
    let pubKey =
        PublicKey::from_bytes(slice::from_raw_parts(pubKey.start, pubKey.len as usize)).unwrap();
    let sig = Signature::from_bytes(slice::from_raw_parts(sig.start, sig.len as usize)).unwrap();
    pubKey.verify(message_slice, &sig).is_ok()
}

/////////////////

// #[no_mangle]
// pub extern "C" fn get_string_from_rust() -> RustByteSlice {
//     let s = "This is a string from Rust.";
//     RustByteSlice {
//         start: s.as_ptr(),
//         len: s.len() as size_t,
//     }
// }

// #[no_mangle]
// pub extern "C" fn utf8_bytes_to_rust(bytes: *const u8, len: size_t) {
//     let byte_slice: &[u8] = unsafe { slice::from_raw_parts(bytes, len as usize) };
//     print_byte_slice_as_utf8(byte_slice);
// }
// fn print_byte_slice_as_utf8(bytes: &[u8]) {
//     match str::from_utf8(bytes) {
//         Ok(s) => println!("got {}", s),
//         Err(err) => println!("invalid UTF-8 data: {}", err),
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn hello(to: *const c_char) -> *mut c_char {
//     let c_str = CStr::from_ptr(to);
//     let recipient = match c_str.to_str() {
//         Ok(s) => s,
//         Err(_) => "you",
//     };

//     CString::new(format!("Hello from Rust: {}", recipient))
//         .unwrap()
//         .into_raw()
// }
// #[no_mangle]
// pub unsafe extern "C" fn hello_release(s: *mut c_char) {
//     if s.is_null() {
//         return;
//     }
//     CString::from_raw(s);
// }
