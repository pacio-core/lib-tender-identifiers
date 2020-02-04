// pub fn yo() {}
// use rand::rand_chacha::ChaChaRng;

#[cfg(test)]
mod tests {
    #![allow(non_camel_case_types)]
    // use super::*;
    use ed25519_dalek::{
        Keypair, PublicKey, Signature, KEYPAIR_LENGTH, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH,
    };
    // use rand::rngs::adapter::ReadRng;
    use rand::rngs::{OsRng, StdRng};
    use rand::SeedableRng;
    use std::convert::TryInto;

    fn seed_rng() -> StdRng {
        // let seed: &[u8] = &[1, 2, 3, 4,5,6,7,8,9,10];
        let seed_str: [u8; 32] = "correct horse battery staple pad"
            .as_bytes()
            .try_into()
            .expect("slice with incorrect length");
        let rng: StdRng = SeedableRng::from_seed(seed_str);
        // let mut rng = StdRng::from_seed(SeedableRng::seed_from_u64(&mut 1))
        rng
    }

    #[test]
    fn signs_and_verifies() {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let message: &[u8] = b"This is a test of the tsunami alert system.";
        let signature: Signature = keypair.sign(message);
        assert!(keypair.verify(message, &signature).is_ok());

        let public_key: PublicKey = keypair.public;
        assert!(public_key.verify(message, &signature).is_ok());
    }
    #[test]
    fn ser_deser() -> Result<(), bErr> {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let keypair_bytes: [u8; KEYPAIR_LENGTH] = keypair.to_bytes();
        assert_eq!(keypair_bytes.len(), 64);

        let got_keypair = Keypair::from_bytes(&keypair_bytes)?;
        assert_eq!(got_keypair.public, keypair.public);
        assert_eq!(got_keypair.secret.to_bytes(), keypair.secret.to_bytes());

        Ok(())
    }

    #[test]
    fn key_sigs_length() {
        assert_eq!(PUBLIC_KEY_LENGTH, 32);
        assert_eq!(SECRET_KEY_LENGTH, 32);

        let keypair = new_keypair();
        assert_eq!(keypair.public.to_bytes().len(), 32);
        assert_eq!(keypair.secret.to_bytes().len(), 32);

        let message: &[u8] = b"This is a test of the tsunami alert system.";
        let sig: Signature = keypair.sign(message);
        assert!(keypair.verify(message, &sig).is_ok());
        assert_eq!(sig.to_bytes().len(), 64);

        // dbg!(kp);
    }

    //

    //////////////////
    // HELPERS

    type bErr = Box<dyn std::error::Error>;

    fn new_keypair() -> Keypair {
        // let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        // let mut rng = ReadRng::new(&data[..]);
        // println!("{:x}", rng.gen::<u32>());

        let mut rng = seed_rng();
        let keypair: Keypair = Keypair::generate(&mut rng);
        dbg!(keypair.public);
        keypair
    }
}
