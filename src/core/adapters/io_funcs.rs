use std::{fmt, fs, path::Path};

use crate::core::adapters::io_adapters::IoError;

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
    Json,
}

pub type ReadFn = fn(&Path) -> Result<IoValue, IoError>;
pub type WriteFn = fn(&Path, IoValue) -> Result<(), IoError>;

pub(crate) fn read_str(path: &Path) -> std::result::Result<IoValue, IoError> {
    let res = fs::read_to_string(path)?;
    Ok(IoValue::Str(res))
}

pub(crate) fn write_str(path: &Path, contents: IoValue) -> Result<(), IoError> {
    match contents {
        IoValue::Str(s) => {
            fs::write(path, s)?;
            Ok(())
        }
        _ => Err(IoError::InvalidFileType(FileType::Str)),
    }
}
