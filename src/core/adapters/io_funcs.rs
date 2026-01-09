use std::{collections::HashMap, fmt, fs, path::Path};

use crate::core::adapters::{io_adapters::IoError, io_params::Extras};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum FileType {
    Str,
}
impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // simply delegate to Debug
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum IoValue {
    Str(String),
    Json(serde_json::Value),
}
impl IoValue {
    pub fn to_string(&self) -> Result<String, IoError> {
        match self {
            IoValue::Str(s) => Ok(s.to_string()),
            _ => Err(IoError::TypeMismatch),
        }
    }
}

pub type ReadFn = fn(&Path, &Extras) -> Result<IoValue, IoError>;
pub type WriteFn = fn(&Path, IoValue, &Extras) -> Result<(), IoError>;

pub(crate) fn read_str(path: &Path, _extras: &Extras) -> std::result::Result<IoValue, IoError> {
    let res = fs::read_to_string(path)?;
    Ok(IoValue::Str(res))
}

pub(crate) fn write_str(path: &Path, contents: IoValue, _extras: &Extras) -> Result<(), IoError> {
    match contents {
        IoValue::Str(s) => {
            fs::write(path, s)?;
            Ok(())
        }
        _ => Err(IoError::InvalidFileType(FileType::Str)),
    }
}

pub(crate) fn get_request(url: &Path, extras: &Extras) -> std::result::Result<IoValue, IoError> {
    pub fn extras_to_headers(extras: &Extras) -> Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();

        for (key, value) in extras {
            let header_name = HeaderName::try_from(key.as_str())?;
            let header_value = HeaderValue::try_from(value.as_str())?;
            headers.insert(header_name, header_value);
        }

        Ok(headers)
    }
    let client = Client::new();
    let headers = extras_to_headers(extras)?;
    let res = client
        .get(url.to_string_lossy().to_string())
        .headers(headers)
        .send()?;
    match res.json() {
        Ok(j) => Ok(IoValue::Json(j)),
        Err(e) => Err(IoError::ReqwestError(e)),
    }
}
