mod io_adapter_builder;
mod io_adapters;
mod io_funcs;
mod io_params;
pub(crate) use crate::core::adapters::{
    io_adapter_builder::AdapterBuilder,
    io_funcs::{get_request, read_str, write_str},
};
pub use io_adapters::{Adapter, FakeFileMap};
pub use io_funcs::{FileType, IoValue};
pub use io_params::{ParamKey, ParamValue};

fn register_fns() -> AdapterBuilder {
    AdapterBuilder::new()
        .register_read(FileType::Str, read_str)
        .register_write(FileType::Str, write_str)
        .register_read(FileType::ApiCall, get_request)
}

pub fn get_real_adapter() -> impl Adapter {
    register_fns().get_real_adapter()
}

pub fn get_fake_adapter(files: FakeFileMap) -> impl Adapter {
    register_fns().get_fake_adapter(files)
}
