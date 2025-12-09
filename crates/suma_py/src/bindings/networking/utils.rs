use pyo3::prelude::*;
use crate::core::networking::utils as core_utils;

#[pyfunction]
#[pyo3(name = "compress_ipv6")]
#[pyo3(text_signature = "(addr)")]
pub fn compress_ipv6(addr: &str) -> PyResult<String> {
    core_utils::compress_ipv6(addr)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pyfunction]
#[pyo3(name = "expand_ipv6")]
#[pyo3(text_signature = "(addr)")]
pub fn expand_ipv6(addr: &str) -> PyResult<String> {
    core_utils::expand_ipv6(addr)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

