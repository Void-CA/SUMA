use std::net::Ipv6Addr;
use crate::core::networking::error::NetworkError;

pub fn compress_ipv6(addr: &str) -> Result<String, NetworkError> {
    match addr.parse::<Ipv6Addr>() {
        Ok(ipv6) => Ok(ipv6.to_string()),
        Err(_) => Err(NetworkError::InvalidAddress(addr.to_string())),
    }
}

pub fn expand_ipv6(addr: &str) -> Result<String, NetworkError> {
    match addr.parse::<Ipv6Addr>() {
        Ok(ipv6) => Ok(
            ipv6.segments()
                .iter()
                .map(|s| format!("{:04x}", s))
                .collect::<Vec<_>>()
                .join(":")
        ),
        Err(_) => Err(NetworkError::InvalidAddress(addr.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_ipv6() {
        let result = compress_ipv6("2001:0db8:0000:0000:0000:ff00:0042:8329").unwrap();
        assert_eq!(result, "2001:db8::ff00:42:8329");
    }

    #[test]
    fn test_expand_ipv6() {
        let result = expand_ipv6("2001:db8::ff00:42:8329").unwrap();
        assert_eq!(result, "2001:0db8:0000:0000:0000:ff00:0042:8329");
    }
}
