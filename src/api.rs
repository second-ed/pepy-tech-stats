use crate::core::{adapters::get_real_adapter, domain::errors::PepyStatsError, main, RetCode};
use pyo3::prelude::*;

#[pyfunction]
fn py_main(projects: Vec<String>, api_key: String) -> PyResult<i8> {
    let mut adapter = get_real_adapter();

    match main(&mut adapter, projects, api_key) {
        Ok(RetCode::OK) => Ok(0),
        Ok(RetCode::ERR) => Ok(1),
        _ => Ok(-1),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pepy_stats_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_main, m)?)?;
    Ok(())
}
