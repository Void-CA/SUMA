// models.rs
use std::{error::Error, net::Ipv4Addr};
use csv::Writer;
use serde::{Deserialize, Serialize};
use crate::core::networking::subnets::base::ip_tools::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetRow {
    pub subred: u32,
    pub direccion_red: Ipv4Addr,
    pub primera_ip: Ipv4Addr,
    pub ultima_ip: Ipv4Addr,
    pub broadcast: Ipv4Addr,
    pub hosts_per_net: u32,
}

impl SubnetRow {
    pub fn new(
        subred: u32,
        network: Ipv4Addr,
        mask: Ipv4Addr,
    ) -> Self {
        let broadcast = calculate_broadcast_address(network, mask);
        let first_host = calculate_first_host(network);
        let last_host = calculate_last_host(broadcast);
        let hosts_per_net = hosts_per_subnet(mask);

        Self {
            subred,
            direccion_red: network,
            primera_ip: first_host,
            ultima_ip: last_host,
            broadcast,
            hosts_per_net,
        }
    }

    pub fn to_csv(subnets: &[Self]) -> Result<String, Box<dyn Error>> {
        let mut wtr = Writer::from_writer(vec![]);
        for s in subnets {
            wtr.serialize(s)?; // escribe una fila
        }
        wtr.flush()?;
        let data = String::from_utf8(wtr.into_inner()?)?;
        Ok(data)
    }
}

pub trait BaseCalculator {
    fn base_ip(&self) -> Ipv4Addr;
    fn base_cidr(&self) -> u8;
    fn subnets(&self) -> &[SubnetRow];
    fn generate_subnets(&mut self);
    
    // Métodos comunes con implementación por defecto
    fn base_mask(&self) -> Ipv4Addr {
        cidr_to_mask(self.base_cidr())
    }
    
    fn network_class(&self) -> &'static str {
        network_class(self.base_ip())
    }
    
    fn total_hosts(&self) -> u32 {
        self.subnets().iter().map(|s| s.hosts_per_net).sum()
    }
    
    fn utilization_percentage(&self) -> f64 {
        let total_capacity: u32 = self.subnets().iter().map(|s| s.hosts_per_net).sum();
        if total_capacity == 0 {
            return 0.0;
        }
        let total_used: u32 = self.subnets().iter().map(|s| {
            // Calcular hosts usados (última - primera + 1)
            let first = u32::from(s.primera_ip);
            let last = u32::from(s.ultima_ip);
            if last >= first { last - first + 1 } else { 0 }
        }).sum();
        
        (total_used as f64 / total_capacity as f64) * 100.0
    }
}