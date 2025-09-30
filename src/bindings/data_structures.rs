use pyo3::prelude::*;
use crate::core::data_structures;

/// Registra el módulo de estructuras de datos
pub fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(parent.py(), "data_structures")?;


    parent.add_submodule(&submodule)?;
    
    Ok(())
}