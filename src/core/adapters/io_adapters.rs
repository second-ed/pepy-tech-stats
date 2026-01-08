use crate::core::adapters::io_funcs::{FileType, IoValue, ReadFn, WriteFn};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub type ReadMap = HashMap<FileType, ReadFn>;
pub type WriteMap = HashMap<FileType, WriteFn>;
pub type FakeFileMap = HashMap<PathBuf, IoValue>;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("file not found at path: {0}")]
    NotFound(PathBuf),
    #[error("adapter given unknown file type: {0}")]
    UnknownFileType(FileType),
    #[error("function given unknown file type: {0}")]
    InvalidFileType(FileType),
    #[error("IoError: {0}")]
    Io(std::io::Error),
    #[error("Value cannot be converted into FileType")]
    TypeMismatch,
}

impl From<std::io::Error> for IoError {
    fn from(e: std::io::Error) -> Self {
        IoError::Io(e)
    }
}

pub trait Adapter {
    fn read(&mut self, path: &Path, file_type: FileType) -> std::result::Result<IoValue, IoError>;
    fn write(
        &mut self,
        path: &Path,
        data: IoValue,
        file_type: FileType,
    ) -> std::result::Result<(), IoError>;
}

pub struct RealAdapter {
    pub read_fns: ReadMap,
    pub write_fns: WriteMap,
}

impl Adapter for RealAdapter {
    fn read(&mut self, path: &Path, file_type: FileType) -> std::result::Result<IoValue, IoError> {
        let func = self
            .read_fns
            .get(&file_type)
            .ok_or(IoError::UnknownFileType(file_type))?;
        func(path)
    }

    fn write(
        &mut self,
        path: &Path,
        data: IoValue,
        file_type: FileType,
    ) -> std::result::Result<(), IoError> {
        let func = self
            .write_fns
            .get(&file_type)
            .ok_or(IoError::UnknownFileType(file_type))?;
        func(path, data)
    }
}
#[allow(unused)]
pub struct FakeAdapter {
    pub read_fns: ReadMap,
    pub write_fns: WriteMap,
    pub files: FakeFileMap,
}

impl Adapter for FakeAdapter {
    fn read(&mut self, path: &Path, file_type: FileType) -> Result<IoValue, IoError> {
        let res = self
            .files
            .get(path)
            .ok_or_else(|| IoError::NotFound(path.to_path_buf().clone()))?;

        let val = match file_type {
            FileType::Str => IoValue::Str(res.to_string()?),
        };
        Ok(val)
    }

    fn write(
        &mut self,
        path: &Path,
        data: IoValue,
        _file_type: FileType,
    ) -> std::result::Result<(), IoError> {
        self.files.insert(path.to_path_buf(), data);
        Ok(())
    }
}
