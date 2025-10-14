use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::wrap_pyfunction;

use crate::core::networking::subnets::subnet_calculator::{SubnetCalculator, SubnetRow};

#[pyclass(name = "SubnetCalculator", module = "suma_ulsa.networking")]
pub struct PySubnetCalculator {
    inner: SubnetCalculator,
}

#[pymethods]
impl PySubnetCalculator {
    #[new]
    #[pyo3(signature = (ip, subnet_quantity))]
    #[pyo3(text_signature = "(ip, subnet_quantity)")]
    pub fn new(ip: &str, subnet_quantity: usize) -> PyResult<Self> {
        // Agregar validación básica
        if subnet_quantity == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "subnet_quantity must be greater than 0",
            ));
        }
        
        Ok(Self {
            inner: SubnetCalculator::new(ip, subnet_quantity),
        })
    }

    /// Returns a simple summary string of the subnet calculation.
    #[pyo3(name = "summary")]
    #[pyo3(text_signature = "($self)")]
    pub fn summary(&self) -> String {
        let rows = self.inner.generate_rows();

        // Helper para convertir máscara a binario
        fn mask_to_bin(mask: &str) -> String {
            mask.split('.')
                .map(|octet| format!("{:08b}", octet.parse::<u8>().unwrap_or(0)))
                .collect::<Vec<_>>()
                .join(".")
        }

        let orig_mask = self.inner.subnet_mask();
        let new_mask = self.inner.new_subnet_mask();

        format!(
            "Subnet Summary\n\
            ───────────────\n\
            IP Address        : {ip}\n\
            Network Class     : {class}\n\
            Original Mask     : {orig_mask}   ({orig_bin})\n\
            New Subnet Mask   : {new_mask}   ({new_bin})\n\
            Network Jump      : {jump}\n\
            Hosts/Subnet      : {hosts}\n\
            Total Subnets     : {total}\n\
            Usable Subnets    : {usable}\n",
            ip = self.inner.original_ip(),
            class = self.inner.net_class(),
            orig_mask = orig_mask,
            orig_bin = mask_to_bin(&orig_mask),
            new_mask = new_mask,
            new_bin = mask_to_bin(&new_mask),
            jump = self.inner.net_jump(),
            hosts = format_number(self.inner.hosts_quantity().try_into().unwrap()),
            total = self.inner.subnet_quantity(),
            usable = rows.len()
        )
    }

    /// Prints the summary to stdout.
    #[pyo3(name = "print_summary")]
    #[pyo3(text_signature = "($self)")]
    pub fn print_summary(&self) {
        println!("{}", self.summary());
    }

    /// Prints the subnet table to stdout.
    #[pyo3(name = "print_table")]
    #[pyo3(text_signature = "($self)")]
    pub fn print_table(&self) {
        println!("{}", self.subnets_table());
    }

    /// Returns a subnets table format
    #[pyo3(name = "subnets_table")]
    #[pyo3(text_signature = "($self)")]
    pub fn subnets_table(&self) -> String {
        let rows = self.inner.generate_rows();
        
        let mut output = String::new();
        output.push_str("Subnet │ Network       │ First Host    │ Last Host     │ Broadcast\n");
        output.push_str("───────┼───────────────┼───────────────┼───────────────┼───────────────\n");
        
        for row in rows {
            output.push_str(&format!(
                "{:6} │ {:13} │ {:13} │ {:13} │ {:13}\n",
                row.subred,
                truncate_string(&row.direccion_red, 13),
                truncate_string(&row.primera_ip, 13),
                truncate_string(&row.ultima_ip, 13),
                truncate_string(&row.broadcast, 13)
            ));
        }
        
        output
    }

    /// Returns all subnet rows as Python objects
    #[pyo3(name = "get_rows")]
    #[pyo3(text_signature = "($self)")]
    pub fn get_rows(&self) -> Vec<PySubnetRow> {
        self.inner.generate_rows()
            .into_iter()
            .map(PySubnetRow::from)
            .collect()
    }

    /// Returns a specific subnet row
    #[pyo3(name = "get_row")]
    #[pyo3(text_signature = "($self, subnet_number)")]
    pub fn get_row(&self, subnet_number: usize) -> PyResult<PySubnetRow> {
        let rows = self.inner.generate_rows();
        if subnet_number == 0 || subnet_number > rows.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                format!("Subnet number {} out of range (1-{})", subnet_number, rows.len())
            ));
        }
        
        Ok(PySubnetRow::from(rows[subnet_number - 1].clone()))
    }

    /// Convert to Python dictionary
    #[pyo3(name = "to_dict")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("ip", self.inner.original_ip())?;
        dict.set_item("subnet_quantity", self.inner.subnet_quantity())?;
        dict.set_item("hosts_per_subnet", self.inner.hosts_quantity())?;
        dict.set_item("network_class", self.inner.net_class())?;
        dict.set_item("subnet_mask", self.inner.subnet_mask())?;
        dict.set_item("new_subnet_mask", self.inner.new_subnet_mask())?;
        dict.set_item("network_jump", self.inner.net_jump())?;
        
        let rows = self.get_rows();
        let py_rows = PyList::empty(py);
        for row in rows {
            py_rows.append(row.to_dict(py)?)?;
        }
        dict.set_item("subnets", py_rows)?;
        
        Ok(dict.into())
    }

    /// Convierte toda la información a formato JSON
    #[pyo3(name = "to_json")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_json(&self) -> PyResult<String> {
        use serde_json::{json, Value};
        
        let rows = self.get_rows();
        let subnets_json: Vec<Value> = rows
            .iter()
            .map(|row| {
                json!({
                    "subnet": row.subred,
                    "network": row.direccion_red,
                    "first_host": row.primera_ip,
                    "last_host": row.ultima_ip,
                    "broadcast": row.broadcast,
                    "usable_hosts": self.calculate_usable_hosts_for_subnet(row)
                })
            })
            .collect();

        let total_usable_hosts = self.calculate_total_usable_hosts();

        let json_data = json!({
            "original_network": {
                "ip_address": self.inner.original_ip(),
                "network_class": self.inner.net_class(),
                "original_mask": self.inner.subnet_mask(),
                "new_subnet_mask": self.inner.new_subnet_mask(),
                "network_jump": self.inner.net_jump()
            },
            "subnet_information": {
                "total_subnets": self.inner.subnet_quantity(),
                "usable_subnets": rows.len(),
                "hosts_per_subnet": self.inner.hosts_quantity(),
                "total_usable_hosts": total_usable_hosts
            },
            "subnets": subnets_json
        });

        serde_json::to_string_pretty(&json_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Error generating JSON: {}", e)
            ))
    }

    /// Convierte la tabla de subredes a formato CSV
    #[pyo3(name = "to_csv")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_csv(&self) -> String {
        let rows = self.get_rows();
        let mut output = String::from("Subnet,Network Address,First Host,Last Host,Broadcast,Usable Hosts\n");
        
        for row in rows {
            let usable_hosts = self.calculate_usable_hosts_for_subnet(&row);
            output.push_str(&format!(
                "{},{},{},{},{},{}\n",
                row.subred,
                row.direccion_red,
                row.primera_ip,
                row.ultima_ip,
                row.broadcast,
                usable_hosts
            ));
        }
        
        output
    }

    /// Convierte a formato YAML
    #[pyo3(name = "to_yaml")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_yaml(&self) -> PyResult<String> {
        use serde_yaml::Value;
        
        let rows = self.get_rows();
        let total_usable_hosts = self.calculate_total_usable_hosts();
        
        let mut yaml_data = serde_yaml::Mapping::new();
        
        // Información de la red original
        let mut original_net = serde_yaml::Mapping::new();
        original_net.insert(
            Value::String("ip_address".to_string()),
            Value::String(self.inner.original_ip().to_string())
        );
        original_net.insert(
            Value::String("network_class".to_string()),
            Value::String(self.inner.net_class().to_string())
        );
        original_net.insert(
            Value::String("original_mask".to_string()),
            Value::String(self.inner.subnet_mask().to_string())
        );
        original_net.insert(
            Value::String("new_subnet_mask".to_string()),
            Value::String(self.inner.new_subnet_mask().to_string())
        );
        original_net.insert(
            Value::String("network_jump".to_string()),
            Value::String(self.inner.net_jump().to_string())
        );
        yaml_data.insert(
            Value::String("original_network".to_string()),
            Value::Mapping(original_net)
        );
        
        // Información de subredes
        let mut subnets_info = serde_yaml::Mapping::new();
        subnets_info.insert(
            Value::String("total_subnets".to_string()),
            Value::Number(self.inner.subnet_quantity().into())
        );
        subnets_info.insert(
            Value::String("usable_subnets".to_string()),
            Value::Number(rows.len().into())
        );
        subnets_info.insert(
            Value::String("hosts_per_subnet".to_string()),
            Value::Number(self.inner.hosts_quantity().into())
        );
        subnets_info.insert(
            Value::String("total_usable_hosts".to_string()),
            Value::Number(total_usable_hosts.into())
        );
        yaml_data.insert(
            Value::String("subnet_information".to_string()),
            Value::Mapping(subnets_info)
        );
        
        // Lista de subredes
        let mut subnets_list = serde_yaml::Sequence::new();
        for row in rows {
            let mut subnet = serde_yaml::Mapping::new();
            subnet.insert(
                Value::String("subnet".to_string()),
                Value::Number(row.subred.into())
            );
            subnet.insert(
                Value::String("network".to_string()),
                Value::String(row.direccion_red.clone())
            );
            subnet.insert(
                Value::String("first_host".to_string()),
                Value::String(row.primera_ip.clone())
            );
            subnet.insert(
                Value::String("last_host".to_string()),
                Value::String(row.ultima_ip.clone())
            );
            subnet.insert(
                Value::String("broadcast".to_string()),
                Value::String(row.broadcast.clone())
            );
            subnet.insert(
                Value::String("usable_hosts".to_string()),
                Value::Number(self.calculate_usable_hosts_for_subnet(&row).into())
            );
            subnets_list.push(Value::Mapping(subnet));
        }
        yaml_data.insert(
            Value::String("subnets".to_string()),
            Value::Sequence(subnets_list)
        );
        
        serde_yaml::to_string(&yaml_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Error generating YAML: {}", e)
            ))
    }

    /// Convierte a formato XML
    #[pyo3(name = "to_xml")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_xml(&self) -> String {
        let rows = self.get_rows();
        let total_usable_hosts = self.calculate_total_usable_hosts();
        
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<subnet_calculation>\n");
        
        // Información de la red original
        xml.push_str("  <original_network>\n");
        xml.push_str(&format!("    <ip_address>{}</ip_address>\n", self.inner.original_ip()));
        xml.push_str(&format!("    <network_class>{}</network_class>\n", self.inner.net_class()));
        xml.push_str(&format!("    <original_mask>{}</original_mask>\n", self.inner.subnet_mask()));
        xml.push_str(&format!("    <new_subnet_mask>{}</new_subnet_mask>\n", self.inner.new_subnet_mask()));
        xml.push_str(&format!("    <network_jump>{}</network_jump>\n", self.inner.net_jump()));
        xml.push_str("  </original_network>\n");
        
        // Información de subredes
        xml.push_str("  <subnet_information>\n");
        xml.push_str(&format!("    <total_subnets>{}</total_subnets>\n", self.inner.subnet_quantity()));
        xml.push_str(&format!("    <usable_subnets>{}</usable_subnets>\n", rows.len()));
        xml.push_str(&format!("    <hosts_per_subnet>{}</hosts_per_subnet>\n", self.inner.hosts_quantity()));
        xml.push_str(&format!("    <total_usable_hosts>{}</total_usable_hosts>\n", total_usable_hosts));
        xml.push_str("  </subnet_information>\n");
        
        // Lista de subredes
        xml.push_str("  <subnets>\n");
        for row in rows {
            let usable_hosts = self.calculate_usable_hosts_for_subnet(&row);
            xml.push_str(&format!("    <subnet id=\"{}\">\n", row.subred));
            xml.push_str(&format!("      <network>{}</network>\n", row.direccion_red));
            xml.push_str(&format!("      <first_host>{}</first_host>\n", row.primera_ip));
            xml.push_str(&format!("      <last_host>{}</last_host>\n", row.ultima_ip));
            xml.push_str(&format!("      <broadcast>{}</broadcast>\n", row.broadcast));
            xml.push_str(&format!("      <usable_hosts>{}</usable_hosts>\n", usable_hosts));
            xml.push_str("    </subnet>\n");
        }
        xml.push_str("  </subnets>\n");
        xml.push_str("</subnet_calculation>");
        
        xml
    }

    /// Convierte a formato Markdown
    #[pyo3(name = "to_markdown")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_markdown(&self) -> String {
        let rows = self.get_rows();
        let total_usable_hosts = self.calculate_total_usable_hosts();
        
        let mut md = String::new();
        
        // Encabezado
        md.push_str(&format!("# Subnet Calculation: {}\n\n", self.inner.original_ip()));
        
        // Información de la red
        md.push_str("## Network Information\n\n");
        md.push_str(&format!("- **IP Address**: `{}`\n", self.inner.original_ip()));
        md.push_str(&format!("- **Network Class**: {}\n", self.inner.net_class()));
        md.push_str(&format!("- **Original Mask**: `{}`\n", self.inner.subnet_mask()));
        md.push_str(&format!("- **New Subnet Mask**: `{}`\n", self.inner.new_subnet_mask()));
        md.push_str(&format!("- **Network Jump**: `{}`\n", self.inner.net_jump()));
        md.push_str(&format!("- **Total Subnets**: {}\n", self.inner.subnet_quantity()));
        md.push_str(&format!("- **Usable Subnets**: {}\n", rows.len()));
        md.push_str(&format!("- **Hosts per Subnet**: {}\n", self.inner.hosts_quantity()));
        md.push_str(&format!("- **Total Usable Hosts**: {}\n\n", total_usable_hosts));
        
        // Tabla de subredes
        md.push_str("## Subnets\n\n");
        md.push_str("| Subnet | Network | First Host | Last Host | Broadcast | Usable Hosts |\n");
        md.push_str("|--------|---------|------------|-----------|-----------|--------------|\n");
        
        for row in rows {
            let usable_hosts = self.calculate_usable_hosts_for_subnet(&row);
            md.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.subred,
                row.direccion_red,
                row.primera_ip,
                row.ultima_ip,
                row.broadcast,
                usable_hosts
            ));
        }
        
        md
    }

    /// Exporta a un archivo en el formato especificado
    #[pyo3(name = "export_to_file")]
    #[pyo3(text_signature = "($self, filename, format)")]
    pub fn export_to_file(&self, filename: &str, format: &str) -> PyResult<()> {
        use std::fs::File;
        use std::io::Write;
        
        let content = match format.to_lowercase().as_str() {
            "json" => self.to_json()?,
            "csv" => self.to_csv(),
            "yaml" | "yml" => self.to_yaml()?,
            "xml" => self.to_xml(),
            "md" | "markdown" => self.to_markdown(),
            "txt" | "text" => self.subnets_table(),
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Unsupported format: {}. Supported formats: json, csv, yaml, xml, md, txt", format)
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

    // Métodos auxiliares privados - ahora sin parámetros problemáticos
    fn calculate_usable_hosts_for_subnet(&self, _row: &PySubnetRow) -> usize {
        // En IPv4, se restan 2 hosts por subred (red y broadcast)
        let total_hosts: usize = self.inner.hosts_quantity().try_into().unwrap_or(0);
        if total_hosts >= 2 {
            total_hosts - 2
        } else {
            0
        }
    }

    fn calculate_total_usable_hosts(&self) -> usize {
        let rows = self.get_rows();
        let hosts_per_subnet: usize = self.inner.hosts_quantity().try_into().unwrap_or(0);
        let usable_per_subnet = if hosts_per_subnet >= 2 {
            hosts_per_subnet - 2
        } else {
            0
        };
        
        rows.len() * usable_per_subnet
    }

    // Properties para acceso directo a los atributos
    #[getter]
    fn ip(&self) -> String {
        self.inner.original_ip().to_string()
    }

    #[getter]
    fn subnet_quantity(&self) -> usize {
        self.inner.subnet_quantity()
    }

    #[getter]
    fn hosts_per_subnet(&self) -> usize {
        self.inner.hosts_quantity().try_into().unwrap()
    }

    #[getter]
    fn network_class(&self) -> String {
        self.inner.net_class().to_string()
    }

    #[getter]
    fn subnet_mask(&self) -> String {
        self.inner.subnet_mask().to_string()
    }

    #[getter]   
    fn new_subnet_mask(&self) -> String {
        self.inner.new_subnet_mask().to_string()
    }

    #[getter]
    fn network_jump(&self) -> String {
        self.inner.net_jump().to_string()
    }

    /// Default string representation
    fn __str__(&self) -> String {
        self.summary()
    }

    /// Representation for debugging
    fn __repr__(&self) -> String {
        format!(
            "SubnetCalculator(ip='{}', subnet_quantity={})",
            self.inner.original_ip(),
            self.inner.subnet_quantity()
        )
    }
}

// Wrapper para SubnetRow
#[pyclass(name = "SubnetRow", module = "suma_ulsa.networking")]
pub struct PySubnetRow {
    #[pyo3(get)]
    pub subred: u32,
    #[pyo3(get)]
    pub direccion_red: String,
    #[pyo3(get)]
    pub primera_ip: String,
    #[pyo3(get)]
    pub ultima_ip: String,
    #[pyo3(get)]
    pub broadcast: String,
}

impl From<SubnetRow> for PySubnetRow {
    fn from(row: SubnetRow) -> Self {
        Self {
            subred: row.subred,
            direccion_red: row.direccion_red,
            primera_ip: row.primera_ip,
            ultima_ip: row.ultima_ip,
            broadcast: row.broadcast,
        }
    }
}

#[pymethods]
impl PySubnetRow {
    #[pyo3(text_signature = "($self)")]
    pub fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("subnet", self.subred)?;
        dict.set_item("network", &self.direccion_red)?;
        dict.set_item("first_host", &self.primera_ip)?;
        dict.set_item("last_host", &self.ultima_ip)?;
        dict.set_item("broadcast", &self.broadcast)?;
        Ok(dict.into())
    }

    /// Pretty display for individual subnet row
    #[pyo3(name = "to_pretty_string")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_pretty_string(&self) -> String {
        format!(
            "┌─────────────────────────┐\n\
             │      SUBNET {:3}         │\n\
             ├─────────────────────────┤\n\
             │ Network:   {:15} │\n\
             │ First:     {:15} │\n\
             │ Last:      {:15} │\n\
             │ Broadcast: {:15} │\n\
             └─────────────────────────┘",
            self.subred,
            self.direccion_red,
            self.primera_ip,
            self.ultima_ip,
            self.broadcast
        )
    }

    fn __str__(&self) -> String {
        self.to_pretty_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "SubnetRow(subnet={}, network='{}', first_host='{}', last_host='{}', broadcast='{}')",
            self.subred, self.direccion_red, self.primera_ip, self.ultima_ip, self.broadcast
        )
    }

    /// Convierte la fila a formato JSON
    #[pyo3(name = "to_json")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_json(&self) -> PyResult<String> {
        use serde_json::json;
        
        let json_data = json!({
            "subnet": self.subred,
            "network": self.direccion_red,
            "first_host": self.primera_ip,
            "last_host": self.ultima_ip,
            "broadcast": self.broadcast,
            "ip_range": {
                "start": self.primera_ip,
                "end": self.ultima_ip
            }
        });
        
        serde_json::to_string_pretty(&json_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Error generating JSON: {}", e)
            ))
    }

    /// Convierte la fila a formato CSV
    #[pyo3(name = "to_csv")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{}",
            self.subred,
            self.direccion_red,
            self.primera_ip,
            self.ultima_ip,
            self.broadcast
        )
    }

    /// Convierte la fila a formato YAML
    #[pyo3(name = "to_yaml")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_yaml(&self) -> PyResult<String> {
        use serde_yaml::Value;
        
        let mut yaml_data = serde_yaml::Mapping::new();
        yaml_data.insert(
            Value::String("subnet".to_string()),
            Value::Number(self.subred.into())
        );
        yaml_data.insert(
            Value::String("network".to_string()),
            Value::String(self.direccion_red.clone())
        );
        yaml_data.insert(
            Value::String("first_host".to_string()),
            Value::String(self.primera_ip.clone())
        );
        yaml_data.insert(
            Value::String("last_host".to_string()),
            Value::String(self.ultima_ip.clone())
        );
        yaml_data.insert(
            Value::String("broadcast".to_string()),
            Value::String(self.broadcast.clone())
        );
        
        serde_yaml::to_string(&yaml_data)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Error generating YAML: {}", e)
            ))
    }
}

// Funciones auxiliares
fn format_number(num: usize) -> String {
    if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

// Función de utilidad para crear un calculador rápido
#[pyfunction]
#[pyo3(signature = (ip, subnet_quantity))]
#[pyo3(text_signature = "(ip, subnet_quantity)")]
pub fn create_subnet_calculator(ip: &str, subnet_quantity: usize) -> PyResult<PySubnetCalculator> {
    PySubnetCalculator::new(ip, subnet_quantity)
}


/// Registra el módulo de redes
pub fn register(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(parent.py(), "networking")?;
    
    submodule.add_class::<PySubnetCalculator>()?;
    submodule.add_class::<PySubnetRow>()?;
    submodule.add_function(wrap_pyfunction!(create_subnet_calculator, &submodule)?)?;
    
    
    parent.add_submodule(&submodule)?;
    
    // Registrar el módulo en sys.modules
    parent.py().import("sys")?
        .getattr("modules")?
        .set_item("suma_ulsa.networking", submodule)?;
    
    Ok(())
}
