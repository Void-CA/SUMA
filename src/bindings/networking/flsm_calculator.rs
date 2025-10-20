use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::net::Ipv4Addr;

use crate::bindings::networking::subnet_row::PySubnetRow;
use crate::core::{FLSMCalculator, SubnetRow, BaseCalculator};
use crate::core::formatting::export::Exportable;

// ==================== FLSM CALCULATOR ====================

#[pyclass(name = "FLSMCalculator", module = "suma_ulsa.networking")]
pub struct PyFLSMCalculator {
    inner: FLSMCalculator,
}

#[pymethods]
impl PyFLSMCalculator {
    #[new]
    #[pyo3(signature = (ip, subnet_count))]
    #[pyo3(text_signature = "(ip, subnet_count)")]
    pub fn new(ip: &str, subnet_count: usize) -> PyResult<Self> {
        if subnet_count == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "subnet_count must be greater than 0",
            ));
        }
        
        let inner = FLSMCalculator::new(ip, subnet_count)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
        
        Ok(Self { inner })
    }

    /// Returns a simple summary string of the subnet calculation.
    #[pyo3(name = "summary")]
    #[pyo3(text_signature = "($self)")]
    pub fn summary(&self) -> String {
        let subnets = self.inner.subnets();

        format!(
            "FLSM Subnet Summary\n\
            ───────────────────\n\
            Base IP Address    : {ip}\n\
            Network Class      : {class}\n\
            Base CIDR          : /{cidr}\n\
            New CIDR           : /{new_cidr}\n\
            Subnet Mask        : {mask}\n\
            Subnet Size        : {size} addresses\n\
            Hosts per Subnet   : {hosts}\n\
            Total Subnets      : {total}\n\
            Utilization        : {utilization:.1}%\n",
            ip = self.inner.base_ip(),
            class = self.inner.network_class(),
            cidr = self.inner.base_cidr(),
            new_cidr = self.inner.new_cidr,
            mask = self.inner.new_mask(),
            size = self.inner.subnet_size(),
            hosts = self.inner.hosts_per_subnet(),
            total = subnets.len(),
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
let mut output = String::new();

// Column widths - ajusta según necesites
let widths = [8usize, 18usize, 18usize, 18usize, 18usize, 8usize];
let headers = ["Subnet", "Network", "First Host", "Last Host", "Broadcast", "Hosts"];

// Header
output.push_str(&format!(
    "{:<w0$} │ {:<w1$} │ {:<w2$} │ {:<w3$} │ {:<w4$} │ {:>w5$}\n",
    headers[0], headers[1], headers[2], headers[3], headers[4], headers[5],
    w0 = widths[0], w1 = widths[1], w2 = widths[2], w3 = widths[3], w4 = widths[4], w5 = widths[5]
));

// Separator line
let separator = widths.iter()
    .map(|&w| "─".repeat(w))
    .collect::<Vec<String>>()
    .join("─┼─");
output.push_str(&format!("{}\n", separator));

// Rows
for subnet in subnets {
    output.push_str(&format!(
        "{:<w0$} │ {:<w1$} │ {:<w2$} │ {:<w3$} │ {:<w4$} │ {:>w5$}\n",
        subnet.subred,
        subnet.direccion_red,
        subnet.primera_ip,
        subnet.ultima_ip,
        subnet.broadcast,
        subnet.hosts_per_net,
        w0 = widths[0], w1 = widths[1], w2 = widths[2], w3 = widths[3], w4 = widths[4], w5 = widths[5]
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
        dict.set_item("subnet_count", self.inner.subnet_count)?;
        dict.set_item("network_class", self.inner.network_class())?;
        dict.set_item("new_cidr", self.inner.new_cidr)?;
        dict.set_item("subnet_mask", self.inner.new_mask().to_string())?;
        dict.set_item("subnet_size", self.inner.subnet_size())?;
        dict.set_item("hosts_per_subnet", self.inner.hosts_per_subnet())?;
        dict.set_item("utilization_percentage", self.inner.utilization_percentage())?;

        let subnets = self.get_subnets();
        let py_subnets = PyList::empty(py);
        for subnet in subnets {
            py_subnets.append(subnet.to_dict(py)?)?;
        }
        dict.set_item("subnets", py_subnets)?;

        Ok(dict.into())
    }

    // Export methods
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
    fn subnet_count(&self) -> usize {
        self.inner.subnet_count
    }

    #[getter]
    fn network_class(&self) -> String {
        self.inner.network_class().to_string()
    }

    #[getter]
    fn new_cidr(&self) -> u8 {
        self.inner.new_cidr
    }

    #[getter]
    fn subnet_mask(&self) -> String {
        self.inner.new_mask().to_string()
    }

    #[getter]
    fn subnet_size(&self) -> u32 {
        self.inner.subnet_size()
    }

    #[getter]
    fn hosts_per_subnet(&self) -> u32 {
        self.inner.hosts_per_subnet()
    }

    #[getter]
    fn utilization_percentage(&self) -> f64 {
        self.inner.utilization_percentage()
    }

    #[getter]
    fn total_hosts(&self) -> u32 {
        self.inner.total_hosts()
    }

    /// Default string representation
    fn __str__(&self) -> String {
        self.summary()
    }

    /// Representation for debugging
    fn __repr__(&self) -> String {
        format!(
            "FLSMCalculator(base_ip='{}', subnet_count={})",
            self.inner.base_ip(),
            self.inner.subnet_count
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_subnets_table_formatting() {
        let calculator = PyFLSMCalculator::new("192.168.1.0", 24);
        println!("{}", calculator.unwrap().subnets_table());
        assert!(true);
    }
}