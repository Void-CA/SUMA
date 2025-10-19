pub mod flsm_calculator;
pub mod vlsm_calculator;
pub mod subnet_row;

use pyo3::prelude::*;
use crate::bindings::networking::flsm_calculator::{PyFLSMCalculator};
use crate::bindings::networking::vlsm_calculator::{PyVLSMCalculator};
use crate::bindings::networking::subnet_row::PySubnetRow;

/// Registra el módulo de redes
pub fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(parent.py(), "networking")?;
    
    submodule.add_class::<PyFLSMCalculator>()?;
    submodule.add_class::<PyVLSMCalculator>()?;
    submodule.add_class::<PySubnetRow>()?;
    
    
    parent.add_submodule(&submodule)?;
    
    // Registrar el módulo en sys.modules
    parent.py().import("sys")?
        .getattr("modules")?
        .set_item("suma_ulsa.networking", submodule)?;
    
    Ok(())
}