// export_dtos.rs
use serde::{Deserialize, Serialize};
use crate::core::networking::subnets::base::{SubnetRow, BaseCalculator};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetExport {
    pub subnet_id: u32,
    pub network_address: String,
    pub first_host: String,
    pub last_host: String,
    pub broadcast: String,
    pub hosts_count: u32,
    pub required_hosts: u32,
    pub utilization: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkExport {
    pub base_network: String,
    pub base_mask: String,
    pub base_cidr: u8,
    pub network_class: String,
    pub total_subnets: usize,
    pub total_hosts_capacity: u32,
    pub utilization_percentage: f64,
    pub subnets: Vec<SubnetExport>,
}

// Implementación para SubnetRow
impl From<&SubnetRow> for SubnetExport {
    fn from(row: &SubnetRow) -> Self {
        let hosts_count = row.hosts_per_net;
        let required_hosts = if hosts_count > 2 { hosts_count - 2 } else { hosts_count };
        let utilization = if hosts_count > 0 {
            (required_hosts as f64 / hosts_count as f64) * 100.0
        } else {
            0.0
        };
        
        Self {
            subnet_id: row.subred,
            network_address: row.direccion_red.to_string(),
            first_host: row.primera_ip.to_string(),
            last_host: row.ultima_ip.to_string(),
            broadcast: row.broadcast.to_string(),
            hosts_count,
            required_hosts,
            utilization,
        }
    }
}

// Implementación genérica para cualquier calculadora que implemente BaseCalculator
pub fn generate_network_export<T: BaseCalculator>(calculator: &T) -> NetworkExport {
    let subnets: Vec<SubnetExport> = calculator.subnets()
        .iter()
        .map(SubnetExport::from)
        .collect();

    NetworkExport {
        base_network: calculator.base_ip().to_string(),
        base_mask: calculator.base_mask().to_string(),
        base_cidr: calculator.base_cidr(),
        network_class: calculator.network_class().to_string(),
        total_subnets: calculator.subnets().len(),
        total_hosts_capacity: calculator.total_hosts(),
        utilization_percentage: calculator.utilization_percentage(),
        subnets,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::core::formatting::export::Exportable;
    use super::*;
    use crate::core::networking::subnets::flsm::FLSMCalculator;
    use serde_json;

    #[test]
    fn test_subnet_export_from_subnet_row() {
        // Crear un SubnetRow de prueba
        let row = crate::core::networking::subnets::base::SubnetRow {
            subred: 1,
            direccion_red: "192.168.1.0".parse().unwrap(),
            primera_ip: "192.168.1.1".parse().unwrap(),
            ultima_ip: "192.168.1.254".parse().unwrap(),
            broadcast: "192.168.1.255".parse().unwrap(),
            hosts_per_net: 256,
        };

        let export: SubnetExport = SubnetExport::from(&row);

        assert_eq!(export.subnet_id, 1);
        assert_eq!(export.network_address, "192.168.1.0");
        assert_eq!(export.first_host, "192.168.1.1");
        assert_eq!(export.last_host, "192.168.1.254");
        assert_eq!(export.broadcast, "192.168.1.255");
        assert_eq!(export.hosts_count, 256);
        assert_eq!(export.required_hosts, 254);
        assert!((export.utilization - 99.21875).abs() < 1e-5);
    }

    #[test]
    fn test_network_export_from_calculator() {
        let mut calc = FLSMCalculator::new("192.168.1.0/24", 2).unwrap();
        calc.generate_subnets();

        let export = generate_network_export(&calc);

        assert_eq!(export.base_network, "192.168.1.0");
        assert_eq!(export.base_cidr, 24);
        assert_eq!(export.total_subnets, 2);
        assert_eq!(export.subnets.len(), 2);

        // Verificar primeros campos del primer subnet
        let first = &export.subnets[0];
        assert_eq!(first.subnet_id, 1);
        assert_eq!(first.network_address, "192.168.1.0");

        // Verificar serialización JSON
        let json = serde_json::to_string(&export).unwrap();
        let deserialized: NetworkExport = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.base_network, export.base_network);
        assert_eq!(deserialized.subnets.len(), export.subnets.len());
    }

    #[test]
    fn test_network_export_empty_subnets() {
        let calc = FLSMCalculator::new("10.0.0.0/8", 0).unwrap();
        let export = generate_network_export(&calc);
        assert_eq!(export.total_subnets, 0);
        assert!(export.subnets.is_empty());
    }

    #[test]
    fn test_exportable_formats() {
        // 1️⃣ Crear una calculadora de prueba
        let calculator = FLSMCalculator::new("192.168.1.0/24", 2).unwrap();

        // 2️⃣ Generar NetworkExport
        let network_export: NetworkExport = generate_network_export(&calculator);

        // 3️⃣ JSON
        let json = network_export.to_json().unwrap();
        println!("JSON Output:\n{}", json);
        assert!(json.contains("192.168.1.0"));

        // 4️⃣ CSV
        let csv = network_export.to_csv().unwrap();
        println!("CSV Output:\n{}", csv);
        assert!(csv.contains("base_network"));

        // 5️⃣ Markdown
        let markdown = network_export.to_markdown().unwrap();
        println!("Markdown Output:\n{}", markdown);
        assert!(markdown.contains("| base_network |"));

        // 6️⃣ Excel
        let excel_path = "test_network_export.xlsx";
        let result = network_export.to_excel(excel_path);
        assert!(result.is_ok(), "Export to Excel failed");

        // Comprobar que el archivo fue creado
        assert!(fs::metadata(excel_path).is_ok(), "Excel file was not created");

        // Limpiar archivo después de la prueba
        let _ = fs::remove_file(excel_path);
    }
}
