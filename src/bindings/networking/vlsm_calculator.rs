use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::net::Ipv4Addr;

use crate::bindings::networking::subnet_row::PySubnetRow;
use crate::core::{SubnetRow, BaseCalculator, VLSMCalculator};
use crate::core::formatting::export::Exportable;

#[pyclass(name = "VLSMCalculator", module = "suma_ulsa.networking")]
pub struct PyVLSMCalculator {
    inner: VLSMCalculator,
}

#[pymethods]
impl PyVLSMCalculator {
    #[new]
    #[pyo3(signature = (ip, host_requirements))]
    #[pyo3(text_signature = "(ip, host_requirements)")]
    pub fn new(ip: &str, host_requirements: Vec<u32>) -> PyResult<Self> {
        if host_requirements.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "host_requirements must not be empty",
            ));
        }
        
        let inner = VLSMCalculator::new(ip, host_requirements)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
        
        Ok(Self { inner })
    }

    /// Returns a simple summary string of the VLSM calculation.
    #[pyo3(name = "summary")]
    #[pyo3(text_signature = "($self)")]
    pub fn summary(&self) -> String {
        let subnets = self.inner.subnets();

        format!(
            "VLSM Subnet Summary\n\
            ───────────────────\n\
            Base IP Address    : {ip}\n\
            Network Class      : {class}\n\
            Base CIDR          : /{cidr}\n\
            Total Subnets      : {total}\n\
            Total Hosts        : {hosts}\n\
            Efficiency         : {efficiency:.1}%\n\
            Utilization        : {utilization:.1}%\n",
            ip = self.inner.base_ip(),
            class = self.inner.network_class(),
            cidr = self.inner.base_cidr(),
            total = subnets.len(),
            hosts = self.inner.total_hosts(),
            efficiency = self.inner.efficiency(),
            utilization = self.inner.utilization_percentage()
        )
    }

    /// Prints the summary to stdout.
    #[pyo3(name = "print_summary")]
    #[pyo3(text_signature = "($self)")]
    pub fn print_summary(&self) {
        println!("{}", self.summary());
    }

    /// Returns a subnets table format
    #[pyo3(name = "subnets_table")]
    #[pyo3(text_signature = "($self)")]
    pub fn subnets_table(&self) -> String {
        let subnets = self.inner.subnets();
        let requirements = self.inner.host_requirements();

        let mut output = String::new();

        // Definir anchos de columna
        let widths = [7, 15, 15, 15, 15, 7, 10, 7];
        let headers = ["Subnet", "Network", "First Host", "Last Host", "Broadcast", "Hosts", "Required", "Usage"];

        // Header
        output.push_str(&format!(
            "{:<w0$} │ {:<w1$} │ {:<w2$} │ {:<w3$} │ {:<w4$} │ {:>w5$} │ {:>w6$} │ {:>w7$}\n",
            headers[0], headers[1], headers[2], headers[3], headers[4], headers[5], headers[6], headers[7],
            w0 = widths[0], w1 = widths[1], w2 = widths[2], w3 = widths[3], w4 = widths[4], 
            w5 = widths[5], w6 = widths[6], w7 = widths[7]
        ));

        // Separator line
        let separator = widths.iter()
            .map(|&w| "─".repeat(w))
            .collect::<Vec<String>>()
            .join("─┼─");
        output.push_str(&format!("{}\n", separator));

        // Data rows
        for (i, subnet) in subnets.iter().enumerate() {
            let required = requirements.get(i).copied().unwrap_or(0);
            let usage = if subnet.hosts_per_net > 0 {
                (required as f64 / subnet.hosts_per_net as f64) * 100.0
            } else {
                0.0
            };
            
            output.push_str(&format!(
                "{:<w0$} │ {:<w1$} │ {:<w2$} │ {:<w3$} │ {:<w4$} │ {:>w5$} │ {:>w6$} │ {:>w7$.1}%\n",
                subnet.subred,
                subnet.direccion_red,
                subnet.primera_ip,
                subnet.ultima_ip,
                subnet.broadcast,
                subnet.hosts_per_net,
                required,
                usage,
                w0 = widths[0], w1 = widths[1], w2 = widths[2], w3 = widths[3], w4 = widths[4],
                w5 = widths[5], w6 = widths[6], w7 = widths[7]
            ));
        }

        output
}

    /// Prints the subnet table to stdout.
    #[pyo3(name = "print_table")]
    #[pyo3(text_signature = "($self)")]
    pub fn print_table(&self) {
        println!("{}", self.subnets_table());
    }

    /// Returns all subnet rows as Python objects
    #[pyo3(name = "get_subnets")]
    #[pyo3(text_signature = "($self)")]
    pub fn get_subnets(&self) -> Vec<PySubnetRow> {
        self.inner.subnets()
            .iter()
            .map(|row| PySubnetRow::from(row.clone()))
            .collect()
    }

    /// Returns a specific subnet row
    #[pyo3(name = "get_subnet")]
    #[pyo3(text_signature = "($self, subnet_number)")]
    pub fn get_subnet(&self, subnet_number: usize) -> PyResult<PySubnetRow> {
        let subnets = self.inner.subnets();
        if subnet_number == 0 || subnet_number > subnets.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                format!("Subnet number {} out of range (1-{})", subnet_number, subnets.len())
            ));
        }

        Ok(PySubnetRow::from(subnets[subnet_number - 1].clone()))
    }

    /// Convert to Python dictionary
    #[pyo3(name = "to_dict")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("base_ip", self.inner.base_ip().to_string())?;
        dict.set_item("base_cidr", self.inner.base_cidr())?;
        dict.set_item("network_class", self.inner.network_class())?;
        dict.set_item("host_requirements", self.inner.host_requirements())?;
        dict.set_item("efficiency", self.inner.efficiency())?;
        dict.set_item("utilization_percentage", self.inner.utilization_percentage())?;
        dict.set_item("total_hosts", self.inner.total_hosts())?;

        let subnets = self.get_subnets();
        let py_subnets = PyList::empty(py);
        for subnet in subnets {
            py_subnets.append(subnet.to_dict(py)?)?;
        }
        dict.set_item("subnets", py_subnets)?;

        Ok(dict.into())
    }

    // Export methods (same as FLSM)
    #[pyo3(name = "to_json")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_json(&self) -> PyResult<String> {
        self.inner.to_json().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })
    }

    #[pyo3(name = "to_csv")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_csv(&self) -> PyResult<String> {
        SubnetRow::to_csv(self.inner.subnets()).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })
    }

    #[pyo3(name = "to_markdown")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_markdown(&self) -> PyResult<String> {
        self.inner.to_markdown_hierarchical().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })
    }

    #[pyo3(name = "to_excel")]
    #[pyo3(signature = (path, /))]
    #[pyo3(text_signature = "($self, path)")]
    pub fn to_excel(&self, path: &str) -> PyResult<()> {
        self.inner.to_excel(path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
        })
    }

    #[pyo3(name = "export_to_file")]
    #[pyo3(text_signature = "($self, filename, format)")]
    pub fn export_to_file(&self, filename: &str, format: &str) -> PyResult<()> {
        use std::fs::File;
        use std::io::Write;

        let format_lower = format.to_lowercase();
        if format_lower == "xlsx" || format_lower == "excel" {
            return self.to_excel(filename);
        }

        let content = match format_lower.as_str() {
            "json" => self.to_json()?,
            "csv" => self.to_csv()?,
            "md" | "markdown" => self.to_markdown()?,
            "txt" | "text" => self.subnets_table(),
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Unsupported format: {}. Supported formats: json, csv, md, txt, xlsx, excel", format)
                ));
            }
        };

        let mut file = File::create(filename)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Error creating file {}: {}", filename, e)
            ))?;

        file.write_all(content.as_bytes())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(
                format!("Error writing to file {}: {}", filename, e)
            ))?;

        Ok(())
    }

    // Properties
    #[getter]
    fn base_ip(&self) -> String {
        self.inner.base_ip().to_string()
    }

    #[getter]
    fn base_cidr(&self) -> u8 {
        self.inner.base_cidr()
    }

    #[getter]
    fn network_class(&self) -> String {
        self.inner.network_class().to_string()
    }

    #[getter]
    fn host_requirements(&self) -> Vec<u32> {
        self.inner.host_requirements().to_vec()
    }

    #[getter]
    fn efficiency(&self) -> f64 {
        self.inner.efficiency()
    }

    #[getter]
    fn utilization_percentage(&self) -> f64 {
        self.inner.utilization_percentage()
    }

    #[getter]
    fn total_hosts(&self) -> u32 {
        self.inner.total_hosts()
    }

    #[getter]
    fn subnet_count(&self) -> usize {
        self.inner.subnets().len()
    }

    /// Default string representation
    fn __str__(self_: PyRef<'_, Self>) -> PyResult<String> {
        Ok(self_.summary())
    }

    /// Representation for debugging
    fn __repr__(&self) -> String {
        format!(
            "VLSMCalculator(base_ip='{}', host_requirements={:?})",
            self.inner.base_ip(),
            self.inner.host_requirements()
        )
    }
}
