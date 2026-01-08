use crate::core::adapters::io_adapters::{FakeAdapter, FakeFileMap, RealAdapter};
use crate::core::adapters::io_funcs::{FileType, ReadFn, WriteFn};
use std::collections::HashMap;

pub struct AdapterBuilder {
    read_fns: HashMap<FileType, ReadFn>,
    write_fns: HashMap<FileType, WriteFn>,
}

impl AdapterBuilder {
    pub fn new() -> Self {
        Self {
            read_fns: HashMap::new(),
            write_fns: HashMap::new(),
        }
    }

    pub fn register_read(mut self, file_type: FileType, func: ReadFn) -> Self {
        self.read_fns.insert(file_type, func);
        self
    }

    pub fn register_write(mut self, file_type: FileType, func: WriteFn) -> Self {
        self.write_fns.insert(file_type, func);
        self
    }

    pub fn get_real_adapter(self) -> RealAdapter {
        RealAdapter {
            read_fns: self.read_fns,
            write_fns: self.write_fns,
        }
    }

    pub fn get_fake_adapter(self, files: FakeFileMap) -> FakeAdapter {
        FakeAdapter {
            read_fns: self.read_fns,
            write_fns: self.write_fns,
            files,
        }
    }
}
