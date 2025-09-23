use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Importa el core
use crate::core::boolean_algebra::truth_table;

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let func = wrap_pyfunction!(generate_truth_table, m)?;
    m.add_function(func)?;

    Ok(())
}

#[pyfunction]
fn generate_truth_table(variables: Vec<String>) -> PyResult<Vec<Vec<bool>>> {
    Ok(truth_table::generate_truth_table(variables))
}
