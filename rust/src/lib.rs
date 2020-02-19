#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use ed25519_dalek::{self as ed, SecretKey};

mod ts_fns;
pub use ed::{PublicKey, Signature};

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
    // pub fn ser(&self) -> String {
    //     let kp_bytes = self.0.to_bytes();
    //     base64::encode(&kp_bytes)
    // }
    // pub fn from_ser(ser: &str) -> Self {
    //     let deser = base64::decode(&ser).unwrap();
    //     let kp = ed::Keypair::from_bytes(&deser).unwrap();
    //     Self(kp)
    // }
}

pub struct SeedPhrase(String);
impl SeedPhrase {
    pub fn from_str(s: &str) -> Self {
        return Self(s.to_owned());
    }
    pub fn new_random() -> Self {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        Self(mnemonic.phrase().to_owned())
    }
    pub fn to_string(&self) -> String {
        self.0.to_owned()
    }
    pub fn into_seed(&self) -> [u8; 32] {
        let mnemonic = Mnemonic::from_phrase(&self.0, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, "78s9fhjsuiofhjdskh");
        let mut arr: [u8; 32] = Default::default();
        arr.copy_from_slice(&seed.as_bytes()[..32]);
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
                29, 22, 183, 232, 153, 72, 107, 114, 216, 190, 123, 229, 127, 163, 85, 76, 167, 8,
                198, 158, 3, 75, 35, 50, 163, 112, 4, 205, 106, 31, 109, 243
            ]
        );
        for (k, byte) in [
            25, 199, 110, 145, 17, 190, 23, 43, 7, 174, 207, 107, 157, 225, 128, 162, 6, 154, 186,
            163, 144, 43, 14, 178, 29, 226, 67, 47, 227, 224, 232, 102,
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(*byte, kp.0.secret.to_bytes()[k])
        }
    }

    #[test]
    fn test_signs_and_verifies() {
        let kp = utils::new_keypair();

        let msg: &[u8] = b"This is a test of the tsunami alert system.";
        let sig_bytes = kp.sign(msg);
        assert!(kp.verify(msg, &sig_bytes));

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
