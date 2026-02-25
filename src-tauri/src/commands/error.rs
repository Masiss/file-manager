use std::io::Error as ErrorIO;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error : {0}")]
    IO(#[from] ErrorIO),
    #[error("Error : {0}")]
    AnyHow(#[from] anyhow::Error),
    #[error("JSON error : {0}")]
    Json(String),
    #[error("Decode error :{0}")]
    Decode(#[from] prost::DecodeError),
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err.to_string())
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
