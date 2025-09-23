mod core;
mod bindings;

use pyo3::prelude::*;

#[pymodule]
fn suma_ulsa(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_submodule(&bindings::get_module(_py)?)?;
    Ok(())
}
