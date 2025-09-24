use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Importa el core
use crate::core::boolean_algebra::truth_table;

pub fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    // Crea submódulo
    let submodule = PyModule::new(parent.py(), "boolean_algebra")?;

    // Añade funciones
    submodule.add_function(wrap_pyfunction!(generate_truth_table, &submodule)?)?;

    // Agrega el submódulo al módulo raíz
    parent.add_submodule(&submodule)?;
    Ok(())
}

#[pyfunction]
fn generate_truth_table(variables: Vec<String>) -> PyResult<Vec<Vec<bool>>> {
    Ok(truth_table::generate_truth_table(variables))
}
