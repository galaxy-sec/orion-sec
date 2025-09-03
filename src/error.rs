use derive_more::From;
use orion_error::{StructError, UvsReason};
use serde_derive::Serialize;
use thiserror::Error;

#[derive(Debug, PartialEq, Serialize, Error, From)]
pub enum OrionSecReason {
    #[error("{0}")]
    Sec(SecReason),
    #[error("{0}")]
    Uvs(UvsReason),
}

#[derive(Debug, PartialEq, Serialize, Error)]
pub enum SecReason {
    #[error("sensivive msg {0}")]
    SensitiveMsg(String),
}

pub type SecError = StructError<OrionSecReason>;
pub type SecResult<T> = Result<T, SecError>;
