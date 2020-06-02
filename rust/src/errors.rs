use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed parsing: {0}")]
    ParseErr(anyhow::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::ParseErr(e.into())
    }
}
impl From<ed25519_dalek::SignatureError> for Error {
    fn from(e: ed25519_dalek::SignatureError) -> Self {
        Error::Other(e.into())
    }
}
