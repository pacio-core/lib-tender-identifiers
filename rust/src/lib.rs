use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use rand::rngs::OsRng;

pub fn yo() {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
