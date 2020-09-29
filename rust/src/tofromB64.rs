use ed25519_dalek as ed25d;
//
use crate::KpErr;

pub trait ToFromB64
where
    Self: Sized,
{
    fn to_str(&self) -> String;
    fn from_str(s: &str) -> Result<Self, KpErr>;
}
impl ToFromB64 for ed25d::SecretKey {
    fn to_str(&self) -> String {
        base64::encode(&self.to_bytes().to_vec())
    }
    fn from_str(s: &str) -> Result<Self, KpErr> {
        let bytes = base64::decode(&s)?;
        Ok(Self::from_bytes(&bytes)?)
    }
}

// pub trait ToFromBytes<'a> {
//     fn to_bytes(&self) -> &'a [u8];
//     fn from_bytes(b: AsRef<u8>) -> Result<Self, EdErr>
//     where
//         Self: Sized;
// }
// impl<'a, T> ToFromB64 for T
// where
//     T: ToFromBytes<'a>,
// {
//     fn to_str(&self) -> String {
//         base64::encode(&self.to_bytes().to_vec())
//     }
//     fn from_str() -> Result<Self, EdErr> {
//         unimplemented!()
//     }
// }
