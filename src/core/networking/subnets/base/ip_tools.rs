// ip_tools.rs
use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn cidr_to_mask(cidr: u8) -> Ipv4Addr {
    let capped_cidr = cidr.min(32);
    let mask_u32 = match capped_cidr {
        0 => 0,
        32 => u32::MAX,
        _ => (!0u32) << (32 - capped_cidr),
    };
    Ipv4Addr::from(mask_u32)
}

pub fn mask_to_cidr(mask: Ipv4Addr) -> u8 {
    let octets = mask.octets();
    let mut cidr = 0;

    for &octet in &octets {
        cidr += octet.count_ones();
    }

    cidr as u8
}


pub fn increment_ip(ip: Ipv4Addr, n: u32) -> Ipv4Addr {
    Ipv4Addr::from(u32::from(ip).saturating_add(n))
}

pub fn network_class(ip: Ipv4Addr) -> &'static str {
    match ip.octets()[0] {
        0..=126 => "Clase A",
        127 => "Loopback", // Special case
        128..=191 => "Clase B",
        192..=223 => "Clase C",
        224..=239 => "Clase D",
        240..=255 => "Clase E",
        _ => "Unknown",
    }
}

pub fn calculate_network_address(ip: Ipv4Addr, mask: Ipv4Addr) -> Ipv4Addr {
    Ipv4Addr::from(u32::from(ip) & u32::from(mask))
}

pub fn calculate_broadcast_address(network: Ipv4Addr, mask: Ipv4Addr) -> Ipv4Addr {
    let network_u32 = u32::from(network);
    let mask_u32 = u32::from(mask);
    let wildcard = !mask_u32;
    Ipv4Addr::from(network_u32 | wildcard)
}

pub fn calculate_first_host(network: Ipv4Addr) -> Ipv4Addr {
    increment_ip(network, 1)
}

pub fn calculate_last_host(broadcast: Ipv4Addr) -> Ipv4Addr {
    let broadcast_u32 = u32::from(broadcast);
    Ipv4Addr::from(broadcast_u32 - 1)
}

pub fn hosts_per_subnet(mask: Ipv4Addr) -> u32 {
    let host_bits = 32 - mask_to_cidr(mask);

    match host_bits {
        0 => 1,  // /32
        1 => 2,  // /31
        n => {
            // /0..=/30
            let hosts = 2u64.pow(n as u32) - 2;
            hosts as u32
        }
    }
}





pub fn parse_ip_cidr(ip_str: &str) -> Result<(Ipv4Addr, u8), String> {
    if let Some((ip_part, cidr_part)) = ip_str.split_once('/') {
        let ip = Ipv4Addr::from_str(ip_part).map_err(|_| "Invalid IP address".to_string())?;
        let cidr = cidr_part.parse::<u8>().map_err(|_| "Invalid CIDR notation".to_string())?;
        if cidr > 32 {
            return Err("CIDR must be between 0 and 32".to_string());
        }
        Ok((ip, cidr))
    } else {
        Err("IP must be in the format x.x.x.x/y".to_string())
    }
}


// ip_tools.rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    // Tests para cidr_to_mask
    #[test]
    fn test_cidr_to_mask_valid() {
        assert_eq!(cidr_to_mask(0), Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(cidr_to_mask(8), Ipv4Addr::new(255, 0, 0, 0));
        assert_eq!(cidr_to_mask(16), Ipv4Addr::new(255, 255, 0, 0));
        assert_eq!(cidr_to_mask(24), Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(cidr_to_mask(32), Ipv4Addr::new(255, 255, 255, 255));
    }

    #[test]
    fn test_cidr_to_mask_edge_cases() {
        assert_eq!(cidr_to_mask(1), Ipv4Addr::new(128, 0, 0, 0));
        assert_eq!(cidr_to_mask(25), Ipv4Addr::new(255, 255, 255, 128));
        assert_eq!(cidr_to_mask(30), Ipv4Addr::new(255, 255, 255, 252));
    }

    #[test]
    fn test_cidr_to_mask_invalid() {
        // CIDR mayor a 32 debería devolver máscara completa
        assert_eq!(cidr_to_mask(33), Ipv4Addr::new(255, 255, 255, 255));
        assert_eq!(cidr_to_mask(255), Ipv4Addr::new(255, 255, 255, 255));
    }

    // Tests para mask_to_cidr
    #[test]
    fn test_mask_to_cidr_valid() {
        assert_eq!(mask_to_cidr(Ipv4Addr::new(0, 0, 0, 0)), 0);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 0, 0, 0)), 8);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 0, 0)), 16);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 0)), 24);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 255)), 32);
    }

    #[test]
    fn test_mask_to_cidr_non_standard() {
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 128)), 25);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 192)), 26);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 224)), 27);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 240)), 28);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 248)), 29);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 252)), 30);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 254)), 31);
    }

    #[test]
    fn test_mask_to_cidr_invalid_masks() {
        // Máscaras no válidas (no contiguas) - deberían contar los bits 1
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 0, 255, 0)), 16); // 8 + 8 = 16
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 128, 0, 0)), 9); // 8 + 1 = 9
    }

    // Tests para increment_ip
    #[test]
    fn test_increment_ip_normal() {
        let ip = Ipv4Addr::new(192, 168, 1, 1);
        assert_eq!(increment_ip(ip, 1), Ipv4Addr::new(192, 168, 1, 2));
        assert_eq!(increment_ip(ip, 10), Ipv4Addr::new(192, 168, 1, 11));
        assert_eq!(increment_ip(ip, 254), Ipv4Addr::new(192, 168, 1, 255));
    }

    #[test]
    fn test_increment_ip_cross_octet() {
        let ip = Ipv4Addr::new(192, 168, 1, 255);
        assert_eq!(increment_ip(ip, 1), Ipv4Addr::new(192, 168, 2, 0));
        assert_eq!(increment_ip(ip, 256), Ipv4Addr::new(192, 168, 2, 255));
    }

    #[test]
    fn test_increment_ip_overflow() {
        let max_ip = Ipv4Addr::new(255, 255, 255, 255);
        // Saturating add - no overflow
        assert_eq!(increment_ip(max_ip, 1), max_ip);
        assert_eq!(increment_ip(max_ip, 1000), max_ip);
    }

    #[test]
    fn test_increment_ip_decrement() {
        let ip = Ipv4Addr::new(192, 168, 1, 10);
        // Usando underflow de unsigned (saturating_sub)
        assert_eq!(increment_ip(ip, 0), ip);

        let ip2 = Ipv4Addr::new(192, 168, 1, 0);
        assert_eq!(increment_ip(ip2, 0), ip2);
    }

    // Tests para network_class
    #[test]
    fn test_network_class_a() {
        assert_eq!(network_class(Ipv4Addr::new(1, 0, 0, 1)), "Clase A");
        assert_eq!(network_class(Ipv4Addr::new(10, 0, 0, 1)), "Clase A");
        assert_eq!(network_class(Ipv4Addr::new(126, 255, 255, 254)), "Clase A");
    }

    #[test]
    fn test_network_class_loopback() {
        assert_eq!(network_class(Ipv4Addr::new(127, 0, 0, 1)), "Loopback");
        assert_eq!(network_class(Ipv4Addr::new(127, 255, 255, 255)), "Loopback");
    }

    #[test]
    fn test_network_class_b() {
        assert_eq!(network_class(Ipv4Addr::new(128, 0, 0, 1)), "Clase B");
        assert_eq!(network_class(Ipv4Addr::new(172, 16, 0, 1)), "Clase B");
        assert_eq!(network_class(Ipv4Addr::new(191, 255, 255, 254)), "Clase B");
    }

    #[test]
    fn test_network_class_c() {
        assert_eq!(network_class(Ipv4Addr::new(192, 0, 0, 1)), "Clase C");
        assert_eq!(network_class(Ipv4Addr::new(192, 168, 1, 1)), "Clase C");
        assert_eq!(network_class(Ipv4Addr::new(223, 255, 255, 254)), "Clase C");
    }

    #[test]
    fn test_network_class_d_e() {
        assert_eq!(network_class(Ipv4Addr::new(224, 0, 0, 1)), "Clase D");
        assert_eq!(network_class(Ipv4Addr::new(239, 255, 255, 255)), "Clase D");
        assert_eq!(network_class(Ipv4Addr::new(240, 0, 0, 1)), "Clase E");
        assert_eq!(network_class(Ipv4Addr::new(255, 255, 255, 255)), "Clase E");
    }

    // Tests para calculate_network_address
    #[test]
    fn test_calculate_network_address() {
        let ip = Ipv4Addr::new(192, 168, 1, 100);
        let mask = Ipv4Addr::new(255, 255, 255, 0);
        assert_eq!(calculate_network_address(ip, mask), Ipv4Addr::new(192, 168, 1, 0));

        let ip2 = Ipv4Addr::new(10, 20, 30, 40);
        let mask2 = Ipv4Addr::new(255, 0, 0, 0);
        assert_eq!(calculate_network_address(ip2, mask2), Ipv4Addr::new(10, 0, 0, 0));

        let ip3 = Ipv4Addr::new(192, 168, 1, 100);
        let mask3 = Ipv4Addr::new(255, 255, 255, 192); // /26
        assert_eq!(calculate_network_address(ip3, mask3), Ipv4Addr::new(192, 168, 1, 64));
    }

    // Tests para calculate_broadcast_address
    #[test]
    fn test_calculate_broadcast_address() {
        let network = Ipv4Addr::new(192, 168, 1, 0);
        let mask = Ipv4Addr::new(255, 255, 255, 0);
        assert_eq!(calculate_broadcast_address(network, mask), Ipv4Addr::new(192, 168, 1, 255));

        let network2 = Ipv4Addr::new(10, 0, 0, 0);
        let mask2 = Ipv4Addr::new(255, 0, 0, 0);
        assert_eq!(calculate_broadcast_address(network2, mask2), Ipv4Addr::new(10, 255, 255, 255));

        let network3 = Ipv4Addr::new(192, 168, 1, 64);
        let mask3 = Ipv4Addr::new(255, 255, 255, 192); // /26
        assert_eq!(calculate_broadcast_address(network3, mask3), Ipv4Addr::new(192, 168, 1, 127));
    }

    // Tests para calculate_first_host
    #[test]
    fn test_calculate_first_host() {
        assert_eq!(calculate_first_host(Ipv4Addr::new(192, 168, 1, 0)), Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(calculate_first_host(Ipv4Addr::new(10, 0, 0, 0)), Ipv4Addr::new(10, 0, 0, 1));
        assert_eq!(calculate_first_host(Ipv4Addr::new(172, 16, 0, 0)), Ipv4Addr::new(172, 16, 0, 1));
    }

    // Tests para calculate_last_host
    #[test]
    fn test_calculate_last_host() {
        assert_eq!(calculate_last_host(Ipv4Addr::new(192, 168, 1, 255)), Ipv4Addr::new(192, 168, 1, 254));
        assert_eq!(calculate_last_host(Ipv4Addr::new(10, 255, 255, 255)), Ipv4Addr::new(10, 255, 255, 254));
        assert_eq!(calculate_last_host(Ipv4Addr::new(172, 16, 255, 255)), Ipv4Addr::new(172, 16, 255, 254));
    }

    // Tests para hosts_per_subnet
    #[test]
    fn test_hosts_per_subnet_standard() {
        assert_eq!(hosts_per_subnet(Ipv4Addr::new(255, 255, 255, 0)), 254);   // /24
        assert_eq!(hosts_per_subnet(Ipv4Addr::new(255, 255, 0, 0)), 65534);   // /16
        assert_eq!(hosts_per_subnet(Ipv4Addr::new(255, 0, 0, 0)), 16777214);  // /8
    }

    #[test]
    fn test_hosts_per_subnet_small() {
        assert_eq!(hosts_per_subnet(Ipv4Addr::new(255, 255, 255, 252)), 2);   // /30
        assert_eq!(hosts_per_subnet(Ipv4Addr::new(255, 255, 255, 248)), 6);   // /29
        assert_eq!(hosts_per_subnet(Ipv4Addr::new(255, 255, 255, 240)), 14);  // /28
    }

    #[test]
    fn test_hosts_per_subnet_special_cases() {
        assert_eq!(hosts_per_subnet(Ipv4Addr::new(255, 255, 255, 255)), 1);   // /32
        assert_eq!(hosts_per_subnet(Ipv4Addr::new(255, 255, 255, 254)), 2);   // /31
    }

    #[test]
    fn test_hosts_per_subnet_zero_mask() {
        assert_eq!(hosts_per_subnet(Ipv4Addr::new(0, 0, 0, 0)), 4294967294);  // /0
    }

    // Tests para parse_ip_cidr
    #[test]
    fn test_parse_ip_cidr_valid() {
        assert_eq!(
            parse_ip_cidr("192.168.1.0/24").unwrap(),
            (Ipv4Addr::new(192, 168, 1, 0), 24)
        );
        assert_eq!(
            parse_ip_cidr("10.0.0.0/8").unwrap(),
            (Ipv4Addr::new(10, 0, 0, 0), 8)
        );
        assert_eq!(
            parse_ip_cidr("0.0.0.0/0").unwrap(),
            (Ipv4Addr::new(0, 0, 0, 0), 0)
        );
        assert_eq!(
            parse_ip_cidr("255.255.255.255/32").unwrap(),
            (Ipv4Addr::new(255, 255, 255, 255), 32)
        );
    }

    #[test]
    fn test_parse_ip_cidr_invalid_format() {
        assert!(parse_ip_cidr("192.168.1.1").is_err());
        assert!(parse_ip_cidr("invalid").is_err());
        assert!(parse_ip_cidr("192.168.1.1/").is_err());
        assert!(parse_ip_cidr("/24").is_err());
    }

    #[test]
    fn test_parse_ip_cidr_invalid_cidr() {
        assert!(parse_ip_cidr("192.168.1.1/33").is_err());
        assert!(parse_ip_cidr("192.168.1.1/100").is_err());
        assert!(parse_ip_cidr("192.168.1.1/-1").is_err());
    }

    #[test]
    fn test_parse_ip_cidr_invalid_ip() {
        assert!(parse_ip_cidr("256.168.1.1/24").is_err());
        assert!(parse_ip_cidr("192.168.1.256/24").is_err());
        assert!(parse_ip_cidr("192.168.1/24").is_err());
    }

    // Tests de integración entre funciones
    #[test]
    fn test_network_calculation_integration() {
        let ip = Ipv4Addr::new(192, 168, 1, 100);
        let mask = Ipv4Addr::new(255, 255, 255, 0);

        let network = calculate_network_address(ip, mask);
        let broadcast = calculate_broadcast_address(network, mask);
        let first_host = calculate_first_host(network);
        let last_host = calculate_last_host(broadcast);

        assert_eq!(network, Ipv4Addr::new(192, 168, 1, 0));
        assert_eq!(broadcast, Ipv4Addr::new(192, 168, 1, 255));
        assert_eq!(first_host, Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(last_host, Ipv4Addr::new(192, 168, 1, 254));
    }

    #[test]
    fn test_cidr_mask_conversion_round_trip() {
        for cidr in 0..=32 {
            let mask = cidr_to_mask(cidr);
            let calculated_cidr = mask_to_cidr(mask);
            assert_eq!(cidr, calculated_cidr, "CIDR {} -> mask {:?} -> CIDR {}", cidr, mask, calculated_cidr);
        }
    }

    #[test]
    fn test_subnet_calculation_complete() {
        // Test completo de cálculo de subred
        let (ip, cidr) = parse_ip_cidr("192.168.1.100/26").unwrap();
        let mask = cidr_to_mask(cidr);

        let network = calculate_network_address(ip, mask);
        let broadcast = calculate_broadcast_address(network, mask);
        let first_host = calculate_first_host(network);
        let last_host = calculate_last_host(broadcast);
        let hosts_count = hosts_per_subnet(mask);

        assert_eq!(network, Ipv4Addr::new(192, 168, 1, 64));
        assert_eq!(broadcast, Ipv4Addr::new(192, 168, 1, 127));
        assert_eq!(first_host, Ipv4Addr::new(192, 168, 1, 65));
        assert_eq!(last_host, Ipv4Addr::new(192, 168, 1, 126));
        assert_eq!(hosts_count, 62);
    }

    #[test]
    fn test_edge_case_ips() {
        // Test con IPs extremas
        let min_ip = Ipv4Addr::new(0, 0, 0, 0);
        let max_ip = Ipv4Addr::new(255, 255, 255, 255);

        assert_eq!(increment_ip(min_ip, 1), Ipv4Addr::new(0, 0, 0, 1));
        assert_eq!(increment_ip(max_ip, 1), max_ip); // saturating

        assert_eq!(network_class(min_ip), "Clase A");
        assert_eq!(network_class(max_ip), "Clase E");
    }

    #[test]
    fn test_private_network_classes() {
        // Verificar que las IPs privadas tengan la clase correcta
        assert_eq!(network_class(Ipv4Addr::new(10, 0, 0, 1)), "Clase A");
        assert_eq!(network_class(Ipv4Addr::new(172, 16, 0, 1)), "Clase B");
        assert_eq!(network_class(Ipv4Addr::new(192, 168, 0, 1)), "Clase C");
    }
}