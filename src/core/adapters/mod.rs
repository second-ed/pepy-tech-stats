mod io_adapter_builder;
mod io_adapters;
mod io_funcs;

pub(crate) use crate::core::adapters::{io_adapter_builder::AdapterBuilder, io_adapters::Adapter};
pub(crate) use io_funcs::{read_str, write_str, FileType};

pub fn get_real_adapter() -> impl Adapter {
    AdapterBuilder::new()
        .register_read(FileType::Str, read_str)
        .register_write(FileType::Str, write_str)
        .get_real_adapter()
}
