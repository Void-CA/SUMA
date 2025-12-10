use pyo3::{prelude::*, types::PyDict};

use suma_core::SubnetRow;

#[pyclass(name = "SubnetRow", module = "suma_ulsa.networking")]
#[derive(Clone)]
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
    #[pyo3(get)]
    pub hosts_per_net: u32,
}

impl From<SubnetRow> for PySubnetRow {
    fn from(row: SubnetRow) -> Self {
        Self {
            subred: row.subred,
            direccion_red: row.direccion_red.to_string(),
            primera_ip: row.primera_ip.to_string(),
            ultima_ip: row.ultima_ip.to_string(),
            broadcast: row.broadcast.to_string(),
            hosts_per_net: row.hosts_per_net,
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
        dict.set_item("hosts_per_net", self.hosts_per_net)?;
        Ok(dict.into())
    }

    /// Pretty display for individual subnet row
    #[pyo3(name = "to_pretty_string")]
    #[pyo3(text_signature = "($self)")]
    pub fn to_pretty_string(&self) -> String {
        format!(
            "┌─ SUBNET {:3} ──────────────────────────────────────────────────────────┐\n\
        {:2}Network: {:15} │ First: {:15} │ Hosts: {:9}  \n\
                 {:2}Broadcast: {:13} │ Last:  {:15} │         {:2}        \n\
            └───────────────────────────────────────────────────────────────────────┘",
            self.subred,
            "",
            self.direccion_red,
            self.primera_ip,
            self.hosts_per_net,
            "",
            self.broadcast,
            self.ultima_ip,
            ""

        )
    }

    fn __str__(&self) -> String {
        self.to_pretty_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "SubnetRow(subnet={}, network='{}', first_host='{}', last_host='{}', broadcast='{}', hosts_per_net={})",
            self.subred, self.direccion_red, self.primera_ip, self.ultima_ip, self.broadcast, self.hosts_per_net
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_string() {
        let subnet_row = PySubnetRow {
            subred: 1,
            direccion_red: "192.168.1.0".to_string(),
            primera_ip: "192.168.1.1".to_string(),
            ultima_ip: "192.168.1.254".to_string(),
            broadcast: "192.168.1.255".to_string(),
            hosts_per_net: 256,
        };
        let pretty = subnet_row.to_pretty_string();
        println!("{}", pretty);
        assert!(pretty.contains("SUBNET   1"));
    }
        
    }