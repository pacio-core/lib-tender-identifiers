#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use bip39::{Language, Mnemonic, MnemonicType};
use blake2::{Blake2b, Digest};
use ed25519_dalek::{self as ed, SecretKey};

pub use ed::{PublicKey, Signature};
// mod bindings;

#[derive(Debug)]
pub struct KeyPair(ed::Keypair);
impl KeyPair {
    pub fn from_phrase(phrase: &str) -> Self {
        let seed = SeedPhrase::from_str(phrase).into_seed();
        let secret = SecretKey::from_bytes(&seed).unwrap();
        let public: PublicKey = (&secret).into();

        let mut pair = vec![];
        pair.extend_from_slice(&seed);
        pair.extend_from_slice(public.as_bytes());

        let kp = ed::Keypair::from_bytes(&pair).unwrap();
        Self(kp)
    }

    pub fn pubkey(&self) -> PublicKey {
        self.0.public
    }

    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.0.sign(&message).to_bytes()
    }
    pub fn verify(&self, message: &[u8], sig: &[u8]) -> bool {
        self.0
            .public
            .verify(message, &Signature::from_bytes(&sig).unwrap())
            .is_ok()
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), 64);
        Self(ed::Keypair::from_bytes(bytes).unwrap())
    }
}

pub struct SeedPhrase(String);
impl SeedPhrase {
    pub fn new_random() -> Self {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        Self(mnemonic.phrase().to_owned())
    }
    pub fn from_str(s: &str) -> Self {
        return Self(s.to_owned());
    }
    pub fn to_str(&self) -> String {
        self.0.to_owned()
    }
    pub fn into_seed(&self) -> [u8; 32] {
        let mut arr: [u8; 32] = Default::default();
        arr.copy_from_slice(&Blake2b::digest(self.0.as_bytes())[..32]);
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyPair, Signature};
    use utils::bErr;

    #[test]
    fn test_generates_keypair() {
        let phrase = String::from(
            "famous concert update chimney vicious repeat camp awful equal cash leisure stable",
        );
        let kp = KeyPair::from_phrase(&phrase);

        assert_eq!(
            kp.0.public.to_bytes(),
            [
                73, 49, 171, 46, 173, 51, 219, 117, 97, 85, 196, 48, 227, 42, 201, 0, 38, 245, 250,
                186, 82, 194, 87, 85, 208, 148, 180, 231, 240, 204, 242, 90
            ]
        );
        assert_eq!(
            kp.0.secret.to_bytes(),
            [
                136, 43, 140, 164, 87, 8, 104, 39, 242, 182, 44, 253, 236, 253, 115, 28, 152, 43,
                56, 73, 78, 26, 8, 248, 146, 1, 64, 92, 38, 169, 53, 217
            ]
        );
        for (k, byte) in [
            136, 43, 140, 164, 87, 8, 104, 39, 242, 182, 44, 253, 236, 253, 115, 28, 152, 43, 56,
            73, 78, 26, 8, 248, 146, 1, 64, 92, 38, 169, 53, 217, 73, 49, 171, 46, 173, 51, 219,
            117, 97, 85, 196, 48, 227, 42, 201, 0, 38, 245, 250, 186, 82, 194, 87, 85, 208, 148,
            180, 231, 240, 204, 242, 90,
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(*byte, kp.to_bytes()[k])
        }
    }

    #[test]
    fn test_signs_and_verifies() {
        let kp = utils::new_keypair();

        let msg: &[u8] = b"A short msg";
        let sig_bytes = kp.sign(msg);
        assert!(kp.verify(msg, &sig_bytes));

        for (k, byte) in [
            146, 245, 180, 26, 7, 213, 47, 135, 129, 130, 210, 72, 55, 84, 111, 162, 191, 204, 89,
            164, 62, 160, 4, 208, 106, 157, 198, 129, 217, 23, 73, 206, 178, 105, 25, 40, 235, 18,
            18, 49, 13, 69, 11, 163, 148, 250, 102, 70, 81, 58, 88, 101, 29, 132, 64, 206, 0, 234,
            249, 102, 106, 153, 46, 0,
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(*byte, sig_bytes[k])
        }

        let pubKey = kp.0.public;
        let sig = Signature::from_bytes(&sig_bytes).unwrap();
        assert!(pubKey.verify(msg, &sig).is_ok());
    }

    #[test]
    fn test_ser_deser() -> Result<(), bErr> {
        let kp = utils::new_keypair();
        let ser = kp.to_bytes();
        assert_eq!(ser.len(), 64);

        let deser = KeyPair::from_bytes(&ser);
        assert_eq!(deser.0.public.to_bytes(), kp.0.public.to_bytes());
        assert_eq!(deser.0.secret.to_bytes(), kp.0.secret.to_bytes());

        Ok(())
    }

    //

    mod utils {
        use super::KeyPair;

        pub type bErr = Box<dyn std::error::Error>;

        pub fn new_keypair() -> KeyPair {
            let phrase = String::from(
                "famous concert update chimney vicious repeat camp awful equal cash leisure stable",
            );
            KeyPair::from_phrase(&phrase)
        }
    }
}
