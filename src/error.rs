use orion_error::{StructError, UvsReason};
use serde_derive::Serialize;
use thiserror::Error;

#[derive(Debug, PartialEq, Serialize, Error)]
pub enum SecErrReason {
    #[error("sensivive msg {0}")]
    SensitiveMsg(String),
    #[error("{0}")]
    Uvs(UvsReason),
}

pub type SecError = StructError<SecErrReason>;
pub type SecResult<T> = Result<T, SecError>;

impl From<UvsReason> for SecErrReason {
    fn from(value: UvsReason) -> Self {
        SecErrReason::Uvs(value)
    }
}
