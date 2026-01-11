use crate::core::{adapters::io_params::Extras, domain::errors::PepyStatsError};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde_json::Value;
use std::{fmt, fs, path::Path};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum FileType {
    Str,
    Json,
    ApiCall,
}
impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // simply delegate to Debug
        write!(f, "{:?}", self)
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
            IoValue::Str(s) => Ok(s.to_string()),
            _ => Err(PepyStatsError::TypeMismatch),
        }
    }
    pub fn to_json(&self) -> Result<Value, PepyStatsError> {
        match self {
            IoValue::Json(j) => Ok(j.to_owned()),
            _ => Err(PepyStatsError::TypeMismatch),
        }
    }
}

pub type ReadFn = fn(&Path, &Extras) -> Result<IoValue, PepyStatsError>;
pub type WriteFn = fn(&Path, IoValue, &Extras) -> Result<(), PepyStatsError>;

pub(crate) fn read_str(
    path: &Path,
    _extras: &Extras,
) -> std::result::Result<IoValue, PepyStatsError> {
    let res = fs::read_to_string(path)?;
    Ok(IoValue::Str(res))
}

pub(crate) fn write_str(
    path: &Path,
    contents: IoValue,
    _extras: &Extras,
) -> Result<(), PepyStatsError> {
    match contents {
        IoValue::Str(s) => {
            fs::write(path, s)?;
            Ok(())
        }
        _ => Err(PepyStatsError::InvalidFileType(FileType::Str)),
    }
}

pub(crate) fn get_request(
    url: &Path,
    extras: &Extras,
) -> std::result::Result<IoValue, PepyStatsError> {
    pub fn extras_to_headers(extras: &Extras) -> Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();

        for (key, value) in extras {
            let header_name = HeaderName::try_from(key.as_str())?;
            let header_value = HeaderValue::try_from(value.as_str())?;
            headers.insert(header_name, header_value);
        }

        Ok(headers)
    }

    log::info!("Sending request to: {:?}", &url);

    let client = Client::new();
    let headers = extras_to_headers(extras)?;
    let res = client
        .get(url.to_string_lossy().to_string())
        .headers(headers)
        .send()?;

    log::info!("{:?} | {:?}", &res.url().as_str(), &res.status());

    match res.json() {
        Ok(j) => Ok(IoValue::Json(j)),
        Err(e) => Err(PepyStatsError::ReqwestError(e)),
    }
}
