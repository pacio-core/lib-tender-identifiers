#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use bip39::{Language, Mnemonic, MnemonicType};
use blake2::{Blake2b, Digest};
use ed25519_dalek::{self as ed25d, SecretKey, Signer, Verifier};

#[cfg(test)]
mod _tests;
mod tofromB64;
pub use ed25d::{PublicKey, Signature};
pub use tofromB64::B64;

#[derive(Debug)]
pub struct KeyPair(ed25d::Keypair);
impl KeyPair {
    pub fn from_phrase(phrase: &str) -> Result<Self, KpErr> {
        let seed = SeedPhrase::from_str(phrase).to_seed();
        let secret = SecretKey::from_bytes(&seed)?;
        let public: PublicKey = (&secret).into();

        let mut pair = vec![];
        pair.extend_from_slice(&seed);
        pair.extend_from_slice(public.as_bytes());

        let kp = ed25d::Keypair::from_bytes(&pair)?;
        Ok(Self(kp))
    }

    pub fn pubkey(&self) -> &ed25d::PublicKey {
        &self.0.public
    }
    pub fn privkey(&self) -> &ed25d::SecretKey {
        &self.0.secret
    }
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.0.sign(&message).to_bytes()
    }
    pub fn verify(&self, message: &[u8], sig: &[u8; 64]) -> bool {
        self.0
            .public
            .verify(message, &Signature::new(sig.clone()))
            .is_ok()
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KpErr> {
        if bytes.len() != 64 {
            return Err(KpErr::BytesLengthErr {
                expected: 64,
                got: bytes.len(),
            });
        }
        Ok(Self(ed25d::Keypair::from_bytes(bytes)?))
    }
    pub fn to_str(&self) -> String {
        base64::encode(&self.to_bytes().to_vec())
    }
    pub fn from_str(token: &str) -> Result<Self, KpErr> {
        let bytes = base64::decode(&token)?;
        Ok(Self::from_bytes(&bytes)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum KpErr {
    #[error("unexpected bytes length:")]
    BytesLengthErr { expected: usize, got: usize },
    #[error(transparent)]
    SignatureErr(#[from] ed25d::SignatureError),
    #[error(transparent)]
    Base64DecodeErr(#[from] base64::DecodeError),
}

pub struct SeedPhrase(String);
impl SeedPhrase {
    pub fn new_random() -> Self {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        Self(mnemonic.phrase().to_owned())
    }

    pub fn to_seed(&self) -> [u8; 32] {
        let mut arr: [u8; 32] = Default::default();
        arr.copy_from_slice(&Blake2b::digest(self.0.as_bytes())[..32]);
        arr
    }

    pub fn from_str(s: &str) -> Self {
        return Self(s.to_owned());
    }
    pub fn to_str(&self) -> String {
        self.0.to_owned()
    }
}
