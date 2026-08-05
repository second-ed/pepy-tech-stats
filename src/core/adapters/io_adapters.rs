use crate::core::{
    adapters::io_funcs::{FileType, IoValue, ReadFn, WriteFn},
    domain::errors::PepyStatsError,
};
use log;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub type ReadMap = HashMap<FileType, ReadFn>;
pub type WriteMap = HashMap<FileType, WriteFn>;
pub type FakeFileMap = HashMap<PathBuf, IoValue>;

pub trait Adapter {
    fn read(
        &mut self,
        path: &Path,
        file_type: FileType,
    ) -> std::result::Result<IoValue, PepyStatsError>;
    fn write(
        &mut self,
        path: &Path,
        data: IoValue,
        file_type: FileType,
    ) -> std::result::Result<(), PepyStatsError>;
}

pub struct RealAdapter {
    pub read_fns: ReadMap,
    pub write_fns: WriteMap,
}

impl RealAdapter {
    pub fn new(read_fns: ReadMap, write_fns: WriteMap) -> Self {
        Self {
            read_fns,
            write_fns,
        }
    }
}

impl Adapter for RealAdapter {
    fn read(
        &mut self,
        path: &Path,
        file_type: FileType,
    ) -> std::result::Result<IoValue, PepyStatsError> {
        let func = self
            .read_fns
            .get(&file_type)
            .ok_or(PepyStatsError::UnknownFileType(file_type))?;
        log::info!("reading: {}", path.display());
        func(path)
    }

    fn write(
        &mut self,
        path: &Path,
        data: IoValue,
        file_type: FileType,
    ) -> std::result::Result<(), PepyStatsError> {
        let func = self
            .write_fns
            .get(&file_type)
            .ok_or(PepyStatsError::UnknownFileType(file_type))?;
        func(path, data)
    }
}
#[allow(unused)]
pub struct FakeAdapter {
    pub read_fns: ReadMap,
    pub write_fns: WriteMap,
    pub files: FakeFileMap,
}

impl FakeAdapter {
    pub fn new(read_fns: ReadMap, write_fns: WriteMap, files: FakeFileMap) -> Self {
        Self {
            read_fns,
            write_fns,
            files,
        }
    }
}

impl Adapter for FakeAdapter {
    fn read(&mut self, path: &Path, file_type: FileType) -> Result<IoValue, PepyStatsError> {
        let res = self
            .files
            .get(path)
            .ok_or_else(|| PepyStatsError::NotFound(path.to_path_buf()))?;

        let val = match file_type {
            FileType::Str => IoValue::Str(res.to_string()?),
            FileType::Json => res.to_owned(),
        };
        Ok(val)
    }
    fn write(
        &mut self,
        path: &Path,
        data: IoValue,
        _file_type: FileType,
    ) -> std::result::Result<(), PepyStatsError> {
        self.files.insert(path.to_path_buf(), data);
        Ok(())
    }
}
