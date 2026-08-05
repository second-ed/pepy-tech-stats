use crate::core::domain::errors::PepyStatsError;
use serde_json::Value;
use std::{fmt, fs, path::Path};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum FileType {
    Str,
    Json,
}
impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // simply delegate to Debug
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoValue {
    Str(String),
    Json(serde_json::Value),
}
impl IoValue {
    pub fn to_string(&self) -> Result<String, PepyStatsError> {
        match self {
            Self::Str(s) => Ok(s.clone()),
            Self::Json(_) => Err(PepyStatsError::TypeMismatch),
        }
    }
    pub fn to_json(&self) -> Result<Value, PepyStatsError> {
        match self {
            Self::Json(j) => Ok(j.to_owned()),
            Self::Str(_) => Err(PepyStatsError::TypeMismatch),
        }
    }
}

pub type ReadFn = fn(&Path) -> Result<IoValue, PepyStatsError>;
pub type WriteFn = fn(&Path, IoValue) -> Result<(), PepyStatsError>;

pub fn read_str(path: &Path) -> std::result::Result<IoValue, PepyStatsError> {
    let res = fs::read_to_string(path)?;
    Ok(IoValue::Str(res))
}

pub fn write_str(path: &Path, contents: IoValue) -> Result<(), PepyStatsError> {
    match contents {
        IoValue::Str(s) => {
            fs::write(path, s)?;
            Ok(())
        }
        IoValue::Json(_) => Err(PepyStatsError::InvalidFileType(FileType::Str)),
    }
}
