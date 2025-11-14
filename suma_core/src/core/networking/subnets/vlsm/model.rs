// vlsm_calculator.rs
use std::net::Ipv4Addr;
use serde::{Deserialize, Serialize};
use crate::core::networking::subnets::base::model::{BaseCalculator, SubnetRow};
use crate::core::networking::subnets::base::ip_tools::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct VLSMCalculator {
    base_ip: Ipv4Addr,
    base_cidr: u8,
    host_requirements: Vec<u32>,
    subnets: Vec<SubnetRow>,
    allocated_subnets: Vec<AllocatedSubnet>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AllocatedSubnet {
    original_index: usize,
    hosts_required: u32,
    cidr: u8,
    network: Ipv4Addr,
}

impl VLSMCalculator {
    pub fn new(ip: &str, host_requirements: Vec<u32>) -> Result<Self, String> {
        let (base_ip, base_cidr) = parse_ip_cidr(ip)?;
        
        let mut calculator = Self {
            base_ip,
            base_cidr,
            host_requirements,
            subnets: Vec::new(),
            allocated_subnets: Vec::new(),
        };
        
        calculator.generate_subnets();
        
        Ok(calculator)
    }

    pub(crate) fn calculate_subnet_cidr(&self, hosts_required: u32) -> u8 {
        if hosts_required == 0 {
            return 32; // Subred mínima
        }
        
        // Hosts + red + broadcast = hosts_required + 2
        let needed_hosts = hosts_required + 2;
        let host_bits = (needed_hosts as f64).log2().ceil() as u32;
        32 - host_bits as u8
    }
    
    fn calculate_subnet_size(&self, cidr: u8) -> u32 {
        2u32.pow((32 - cidr) as u32)
    }

    pub fn efficiency(&self) -> f64 {
        let total_required: u32 = self.host_requirements.iter().sum();
        if total_required == 0 {
            return 0.0;
        }

        let total_allocated: u32 = self.host_requirements.iter().map(|&hosts| {
            let needed = hosts + 2; // incluye red y broadcast
            let exp = (needed as f64).log2().ceil() as u32;
            2u32.pow(exp) - 2 // hosts útiles en la subred
        }).sum();

        (total_required as f64 / total_allocated as f64) * 100.0
    }

    pub fn host_requirements(&self) -> &[u32] {
        &self.host_requirements
    }
}

impl BaseCalculator for VLSMCalculator {
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
        self.allocated_subnets.clear();
        
        // Ordenar requerimientos de mayor a menor (VLSM óptimo)
        let mut sorted_requirements: Vec<(usize, u32)> = self.host_requirements
            .iter()
            .enumerate()
            .map(|(i, &h)| (i, h))
            .collect();
        
        sorted_requirements.sort_by(|a, b| b.1.cmp(&a.1));
        
        let mut current_network = calculate_network_address(self.base_ip, self.base_mask());
        
        for (original_index, hosts_required) in sorted_requirements {
            let cidr = self.calculate_subnet_cidr(hosts_required);
            let subnet_size = self.calculate_subnet_size(cidr);
            let mask = cidr_to_mask(cidr);
            
            // Asegurar que estamos en un boundary de red válido
            let network = calculate_network_address(current_network, mask);
            
            let subnet_row = SubnetRow::new(
                (original_index + 1) as u32,
                network,
                mask,
            );
            
            self.allocated_subnets.push(AllocatedSubnet {
                original_index,
                hosts_required,
                cidr,
                network,
            });
            
            current_network = increment_ip(network, subnet_size);
        }

        // Reordenar para mantener el orden original
        self.allocated_subnets.sort_by(|a, b| a.original_index.cmp(&b.original_index));

        for allocated in &self.allocated_subnets {
            let subnet_row = SubnetRow::new(
                (allocated.original_index + 1) as u32,
                allocated.network,
                cidr_to_mask(allocated.cidr),
            );
            self.subnets.push(subnet_row);
        }
    }
}