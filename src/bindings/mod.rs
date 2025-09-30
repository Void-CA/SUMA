use pyo3::prelude::*;
use pyo3::types::PyModule;

pub mod boolean_algebra;
pub mod data_structures;
pub mod matrixes;

pub fn register_modules(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    boolean_algebra::register(parent)?;
    data_structures::register(parent)?;
    Ok(())
}
