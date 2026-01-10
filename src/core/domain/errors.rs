use crate::core::adapters::FileType;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PepyStatsError {
    #[error("file not found at path: {0}")]
    NotFound(PathBuf),
    #[error("adapter given unknown file type: {0}")]
    UnknownFileType(FileType),
    #[error("function given unknown file type: {0}")]
    InvalidFileType(FileType),
    #[error("IoError: {0}")]
    Io(std::io::Error),
    #[error("JsonError: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Value cannot be converted into FileType")]
    TypeMismatch,
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("external error: {0}")]
    External(#[from] Box<dyn std::error::Error>),
    #[error("PolarsError: {0}")]
    Polars(#[from] polars::prelude::PolarsError),
    #[error("RegexError: {0}")]
    RegexError(#[from] regex::Error),
}

impl From<std::io::Error> for PepyStatsError {
    fn from(e: std::io::Error) -> Self {
        PepyStatsError::Io(e)
    }
}
