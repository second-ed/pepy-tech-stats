use crate::core::adapters::{
    io_funcs::{FileType, IoValue, ReadFn, WriteFn},
    io_params::{ParamKey, ParamValue},
};
use crate::core::domain::errors::PepyStatsError;
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
    fn add_param(&mut self, key: ParamKey, value: ParamValue) -> &mut Self;
}

pub struct RealAdapter {
    pub read_fns: ReadMap,
    pub write_fns: WriteMap,
    pub params: HashMap<ParamKey, ParamValue>,
}

impl RealAdapter {
    pub fn new(read_fns: ReadMap, write_fns: WriteMap) -> Self {
        Self {
            read_fns,
            write_fns,
            params: HashMap::new(),
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
        log::info!("reading: {:?}", path);
        func(path, &self.params)
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
        func(path, data, &self.params)
    }
    fn add_param(&mut self, key: ParamKey, value: ParamValue) -> &mut Self {
        self.params.insert(key, value);
        self
    }
}
#[allow(unused)]
pub struct FakeAdapter {
    pub read_fns: ReadMap,
    pub write_fns: WriteMap,
    pub files: FakeFileMap,
    pub params: HashMap<ParamKey, ParamValue>,
}

impl FakeAdapter {
    pub fn new(read_fns: ReadMap, write_fns: WriteMap, files: FakeFileMap) -> Self {
        Self {
            read_fns,
            write_fns,
            files,
            params: HashMap::new(),
        }
    }
}

impl Adapter for FakeAdapter {
    fn read(&mut self, path: &Path, file_type: FileType) -> Result<IoValue, PepyStatsError> {
        let res = self
            .files
            .get(path)
            .ok_or_else(|| PepyStatsError::NotFound(path.to_path_buf().clone()))?;

        let val = match file_type {
            FileType::Str => IoValue::Str(res.to_string()?),
            FileType::Json => res.to_owned(),
            FileType::ApiCall => res.to_owned(),
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
    fn add_param(&mut self, key: ParamKey, value: ParamValue) -> &mut Self {
        self.params.insert(key, value);
        self
    }
}
