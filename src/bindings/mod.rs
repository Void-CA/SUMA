use pyo3::prelude::*;

pub mod boolean_algebra;

pub fn get_module(py: Python) -> PyResult<Bound<'_, PyModule>> {
    let module = PyModule::new(py, "boolean_algebra")?;
    boolean_algebra::register(&module)?;
    Ok(module)
}