#![cfg(target_os = "ios")]
// use c_vec::{CSlice, CVec};
use libc::size_t;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;
use std::str;
//
use crate::{KeyPair, PublicKey, SeedPhrase, Signature};

#[repr(C)]
pub struct RustByteSlice {
    pub bytes: *const u8,
    pub len: size_t,
}

#[no_mangle]
pub extern "C" fn get_string_from_rust() -> RustByteSlice {
    let s = "This is a string from Rust.";
    RustByteSlice {
        bytes: s.as_ptr(),
        len: s.len() as size_t,
    }
}

#[no_mangle]
pub unsafe extern "C" fn hello(to: *const c_char) -> *mut c_char {
    let c_str = CStr::from_ptr(to);
    let recipient = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => "you",
    };

    CString::new(format!("Hello from Rust: {}", recipient))
        .unwrap()
        .into_raw()
}

fn print_byte_slice_as_utf8(bytes: &[u8]) {
    match str::from_utf8(bytes) {
        Ok(s) => println!("got {}", s),
        Err(err) => println!("invalid UTF-8 data: {}", err),
    }
}

#[no_mangle]
pub extern "C" fn utf8_bytes_to_rust(bytes: *const u8, len: size_t) {
    let byte_slice: &[u8] = unsafe { slice::from_raw_parts(bytes, len as usize) };
    print_byte_slice_as_utf8(byte_slice);
}

// #[no_mangle]
// pub unsafe extern "C" fn cvec() -> *mut CVec<u8> {
//     let phrase = String::from("hello world this is a secure passphrase");
//     let kp = KeyPair::from_phrase(&phrase);
//     let mut kp_ptr = kp.to_bytes().as_mut_ptr();
//     &mut CVec::new(kp_ptr, 1)
// }

#[no_mangle]
pub unsafe extern "C" fn hello_release(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    CString::from_raw(s);
}
