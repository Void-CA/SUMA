use pyo3::{prelude::*, types::PyDict};

use crate::core::SubnetRow;

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
            "┌─────────────────────────┐\n\
             │      SUBNET {:3}         │\n\
             ├─────────────────────────┤\n\
             │ Network:   {:15} │\n\
             │ First:     {:15} │\n\
             │ Last:      {:15} │\n\
             │ Broadcast: {:15} │\n\
             │ Hosts:     {:15} │\n\
             └─────────────────────────┘",
            self.subred,
            self.direccion_red,
            self.primera_ip,
            self.ultima_ip,
            self.broadcast,
            self.hosts_per_net
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