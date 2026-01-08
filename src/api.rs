// use crate::core::{adapters::RealFileSystem, domain::RetCode, main};
// use pyo3::prelude::*;

// fn py_main(projects: Vec<String>, api_key: String) -> PyResult<i8> {
//     let mut file_sys = RealFileSystem;

//     match main(&mut file_sys, projects, api_key) {}
// }

// /// A Python module implemented in Rust.
// #[pymodule]
// fn pepy_stats_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(py_main, m)?)?;
//     Ok(())
// }
