use super::{KeyPair, Signature, Verifier};
use anyhow::Error as AnyErr;

#[test]
fn test_generates_keypair() -> Result<(), AnyErr> {
    let phrase = String::from(
        "famous concert update chimney vicious repeat camp awful equal cash leisure stable",
    );
    let kp = KeyPair::from_phrase(&phrase)?;

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
            136, 43, 140, 164, 87, 8, 104, 39, 242, 182, 44, 253, 236, 253, 115, 28, 152, 43, 56,
            73, 78, 26, 8, 248, 146, 1, 64, 92, 38, 169, 53, 217
        ]
    );
    for (k, byte) in [
        136, 43, 140, 164, 87, 8, 104, 39, 242, 182, 44, 253, 236, 253, 115, 28, 152, 43, 56, 73,
        78, 26, 8, 248, 146, 1, 64, 92, 38, 169, 53, 217, 73, 49, 171, 46, 173, 51, 219, 117, 97,
        85, 196, 48, 227, 42, 201, 0, 38, 245, 250, 186, 82, 194, 87, 85, 208, 148, 180, 231, 240,
        204, 242, 90,
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(*byte, kp.to_bytes()[k])
    }
    Ok(())
}
#[test]
fn test_signs_and_verifies() -> Result<(), AnyErr> {
    let kp = utils::new_keypair()?;
    let msg: &[u8] = b"A short msg";
    let sig_bytes = kp.sign(msg);
    assert!(kp.verify(msg, &sig_bytes));
    for (k, byte) in [
        146, 245, 180, 26, 7, 213, 47, 135, 129, 130, 210, 72, 55, 84, 111, 162, 191, 204, 89, 164,
        62, 160, 4, 208, 106, 157, 198, 129, 217, 23, 73, 206, 178, 105, 25, 40, 235, 18, 18, 49,
        13, 69, 11, 163, 148, 250, 102, 70, 81, 58, 88, 101, 29, 132, 64, 206, 0, 234, 249, 102,
        106, 153, 46, 0,
    ]
    .iter()
    .enumerate()
    {
        assert_eq!(*byte, sig_bytes[k])
    }

    let pubKey = kp.0.public;
    let sig = Signature::new(sig_bytes);
    assert!(pubKey.verify(msg, &sig).is_ok());
    Ok(())
}

#[test]
fn test_ser_deser() -> Result<(), AnyErr> {
    let kp = utils::new_keypair()?;
    let ser = kp.to_bytes();
    assert_eq!(ser.len(), 64);

    let deser = KeyPair::from_bytes(&ser)?;
    assert_eq!(deser.0.public.to_bytes(), kp.0.public.to_bytes());
    assert_eq!(deser.0.secret.to_bytes(), kp.0.secret.to_bytes());

    Ok(())
}

//

mod utils {
    use super::*;

    pub fn new_keypair() -> Result<KeyPair, AnyErr> {
        let phrase = String::from(
            "famous concert update chimney vicious repeat camp awful equal cash leisure stable",
        );
        Ok(KeyPair::from_phrase(&phrase)?)
    }
}
