// flsm_calculator.rs
use std::net::Ipv4Addr;
use serde::{Deserialize, Serialize};
use crate::core::networking::subnets::{BaseCalculator, SubnetRow};
use crate::core::networking::subnets::base::ip_tools::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FLSMCalculator {
    base_ip: Ipv4Addr,
    network_address: Ipv4Addr,
    base_cidr: u8,
    pub subnet_count: usize,
    pub hosts_per_subnet: u32,
    subnets: Vec<SubnetRow>,
    pub(crate) new_cidr: u8,
}

impl FLSMCalculator {
    pub fn new(ip: &str, subnet_count: usize) -> Result<Self, String> {
        let (base_ip, base_cidr) = parse_ip_cidr(ip)?;
        let hosts_per_subnet = hosts_per_subnet(cidr_to_mask(base_cidr));
        let network_address = calculate_network_address(base_ip, cidr_to_mask(base_cidr));
        let mut calculator = Self {
            base_ip,
            network_address,
            base_cidr,
            subnet_count,
            hosts_per_subnet,
            subnets: Vec::new(),
            new_cidr: 0,
        };
        
        calculator.calculate_new_mask();
        calculator.generate_subnets();
        
        Ok(calculator)
    }

    fn calculate_new_mask(&mut self) {
        if self.subnet_count <= 1 {
            self.new_cidr = self.base_cidr;
            return;
        }

        let subnet_bits = (self.subnet_count as f64).log2().ceil() as u8;
        self.new_cidr = (self.base_cidr + subnet_bits).min(32);
    }

    pub fn new_mask(&self) -> Ipv4Addr {
        cidr_to_mask(self.new_cidr)
    }
    
    pub fn subnet_size(&self) -> u32 {
        2u32.pow((32 - self.new_cidr) as u32)
    }
    
    pub fn hosts_per_subnet(&self) -> u32 {
        hosts_per_subnet(self.new_mask())
    }
    
    pub fn subnet_jump(&self) -> u32 {
        self.subnet_size()
    }
}

impl BaseCalculator for FLSMCalculator {
    fn base_ip(&self) -> Ipv4Addr {
        self.base_ip
    }
    
    fn base_cidr(&self) -> u8 {
        self.base_cidr
    }
    
    fn subnets(&self) -> &[SubnetRow] {
        &self.subnets
    }
    
    fn generate_subnets(&mut self) {
        self.subnets.clear();
        let mask = self.new_mask();
        let subnet_size = self.subnet_size();
        let mut current_network = calculate_network_address(self.network_address, mask);

        for i in 0..self.subnet_count {
            let subnet_row = SubnetRow::new(
                (i + 1) as u32,
                current_network,
                mask,
            );
            
            self.subnets.push(subnet_row);
            current_network = increment_ip(current_network, subnet_size);
        }
    }
}

// Helper function para parsear IP/CIDR
fn parse_ip_cidr(ip_str: &str) -> Result<(Ipv4Addr, u8), String> {
    if let Some((ip_part, cidr_part)) = ip_str.split_once('/') {
        let ip = ip_part.parse().map_err(|e| format!("IP inválida: {}", e))?;
        let cidr = cidr_part.parse().map_err(|e| format!("CIDR inválido: {}", e))?;
        if cidr > 32 {
            return Err("CIDR debe estar entre 0 y 32".to_string());
        }
        Ok((ip, cidr))
    } else {
        let ip = ip_str.parse().map_err(|e| format!("IP inválida: {}", e))?;
        // CIDR por defecto basado en la clase
        let cidr = match network_class(ip) {
            "Clase A" => 8,
            "Clase B" => 16,
            "Clase C" => 24,
            _ => 24, // Por defecto
        };
        Ok((ip, cidr))
    }
}


