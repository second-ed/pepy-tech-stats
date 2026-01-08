mod io_adapter_builder;
mod io_adapters;
mod io_funcs;

use crate::core::adapters::io_adapters::FakeFileMap;
pub(crate) use crate::core::adapters::{io_adapter_builder::AdapterBuilder, io_adapters::Adapter};
pub(crate) use io_funcs::{read_str, write_str, FileType};

fn register_fns() -> AdapterBuilder {
    AdapterBuilder::new()
        .register_read(FileType::Str, read_str)
        .register_write(FileType::Str, write_str)
}

pub fn get_real_adapter() -> impl Adapter {
    register_fns().get_real_adapter()
}

pub fn get_fake_adapter(files: FakeFileMap) -> impl Adapter {
    register_fns().get_fake_adapter(files)
}
