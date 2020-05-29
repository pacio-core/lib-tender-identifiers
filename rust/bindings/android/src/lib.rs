#![cfg(target_os = "android")]
// #![allow(non_snake_case)]
use jni::objects::{JClass, JString};
use jni::sys::{jbyteArray, jstring};
use jni::JNIEnv;
//
use crate::{KeyPair, PublicKey, SeedPhrase, Signature};

// NOTE: RustyKt references the name rusty.kt, which will be the kotlin file exposing the functions below.
// Remember the JNI naming conventions.

#[no_mangle]
pub extern "system" fn Java_com_pacio_ed25519lib_LibKt_keypairFromPhrase(
    env: JNIEnv,
    _: JClass,
    input: JString, // phrase: String
) -> jbyteArray {
    let phrase: String = env
        .get_string(input)
        .expect("Couldn't get Java string for arg 'phrase'!")
        .into();
    let kp = KeyPair::from_phrase(&phrase);
    let output = env.new_direct_byte_buffer(&mut kp.to_bytes()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_pacio_ed25519lib_LibKt_pubKeyFromPairBytes(
    env: JNIEnv,
    _: JClass,
    input: jbyteArray,
) -> jbyteArray {
    let pair_bytes_vec: Vec<u8> = env.convert_byte_array(input).unwrap();
    let kp = KeyPair::from_bytes(&pair_bytes_vec.as_ref());
    let mut pubKey_bytes = kp.pubkey().to_bytes();
    let output = env.new_direct_byte_buffer(&mut pubKey_bytes).unwrap();
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_pacio_ed25519lib_LibKt_sign(
    env: JNIEnv,
    _: JClass,
    input: (
        jbyteArray, // message: &[u8]
        jbyteArray, // keypair_bytes: &[u8]
    ),
) -> jbyteArray {
    let message_bytes_vec: Vec<u8> = env.convert_byte_array(input.0).unwrap();
    let keypair_bytes_vec: Vec<u8> = env.convert_byte_array(input.1).unwrap();
    let kp = KeyPair::from_bytes(keypair_bytes_vec.as_ref());
    let mut sig_bytes = kp.sign(&message_bytes_vec.as_ref());
    let output = env.new_direct_byte_buffer(&mut sig_bytes).unwrap();
    output.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_com_pacio_ed25519lib_LibKt_verify(
    env: JNIEnv,
    _: JClass,
    input: (
        jbyteArray, // message: &[u8],
        jbyteArray, // pubKey_bytes: &[u8],
        jbyteArray, // sig_bytes: &[u8],
    ),
) -> bool {
    let message_bytes_vec: Vec<u8> = env.convert_byte_array(input.0).unwrap();
    let pubKey_bytes_vec: Vec<u8> = env.convert_byte_array(input.1).unwrap();
    let sig_bytes_vec: Vec<u8> = env.convert_byte_array(input.2).unwrap();
    let pubKey = PublicKey::from_bytes(pubKey_bytes_vec.as_ref()).unwrap();
    let sig = Signature::from_bytes(&sig_bytes_vec.as_ref()).unwrap();
    pubKey.verify(&message_bytes_vec.as_ref(), &sig).is_ok()
}

#[no_mangle]
pub extern "system" fn Java_com_pacio_ed25519lib_LibKt_seedFromPhrase(
    env: JNIEnv,
    _: JClass,
    input: JString, // phrase: String
) -> jbyteArray {
    let phrase: String = env
        .get_string(input)
        .expect("Couldn't get Java string for arg 'phrase'!")
        .into();
    let mut seed = SeedPhrase::from_str(&phrase).into_seed();
    let output = env.new_direct_byte_buffer(&mut seed).unwrap();
    output.into_inner()
}

// TODO DELETE
#[no_mangle]
pub extern "system" fn Java_com_pacio_ed25519lib_LibKt_hello(
    env: JNIEnv,
    _: JClass,
    input: JString, // name: String
) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get Java string for arg 'name'!")
        .into();
    let output = env
        .new_string(format!("Hello from Rust: {}", input))
        .expect("Couldn't create a Java string!");
    output.into_inner()
}
