use gandiva_rust_udf_macro::udf;

use std::net::{Ipv4Addr, Ipv6Addr};

#[udf]
fn is_ipv4(addr: &str) -> bool {
    addr.parse::<Ipv4Addr>().is_ok()
}

#[udf]
fn is_ipv6(addr: &str) -> bool {
    addr.parse::<Ipv6Addr>().is_ok()
}

#[udf]
fn is_ipv4_loopback(addr: &str) -> bool {
    addr.parse::<Ipv4Addr>()
        .map_or(false, |ip| ip.is_loopback())
}

#[udf]
fn is_ipv6_loopback(addr: &str) -> bool {
    addr.parse::<Ipv6Addr>()
        .map_or(false, |ip| ip.is_loopback())
}

#[udf]
fn ipv4_to_ipv6(ipv4: &str) -> String {
    match ipv4.parse::<Ipv4Addr>() {
        Ok(ipv4_addr) => {
            let ipv6_components = ipv4_addr.octets();
            let ipv6_addr = Ipv6Addr::new(
                0,
                0,
                0,
                0,
                0,
                0xffff,
                ((ipv6_components[0] as u16) << 8) + ipv6_components[1] as u16,
                ((ipv6_components[2] as u16) << 8) + ipv6_components[3] as u16,
            );
            ipv6_addr.to_string()
        }
        Err(e) => format!("{}: {}", e, ipv4),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ipv4() {
        let result = is_ipv4("192.168.0.1");
        assert!(result);
        assert!(!is_ipv4("2001:0da8:0207:0000:0000:0000:0000:8207"));
        assert!(!is_ipv4("2hello"));
    }

    #[test]
    fn test_is_ipv6() {
        let result = is_ipv6("192.168.0.1");
        assert!(!result);
        assert!(is_ipv6("2001:0da8:0207:0000:0000:0000:0000:8207"));
        assert!(!is_ipv6("2hello"));
    }

    #[test]
    fn test_is_ipv4_loopback() {
        let result = is_ipv4_loopback("127.0.0.1");
        assert!(result);
        assert!(!is_ipv4_loopback("hello world"));
    }

    #[test]
    fn test_is_ipv6_loopback() {
        let result = is_ipv6_loopback("127.0.0.1");
        assert!(!result);
        assert!(is_ipv6_loopback("::1"));
        assert!(!is_ipv6_loopback("2001:0da8:0207:0000:0000:0000:0000:8207"));
        assert!(!is_ipv6_loopback("hello world"));
    }

    #[test]
    fn test_ipv4_to_ipv6() {
        let result = ipv4_to_ipv6("192.168.0.1");
        assert_eq!(result, "::ffff:192.168.0.1");
    }

    #[test]
    fn test_ipv4_to_ipv6_input_is_ipv6() {
        let result = ipv4_to_ipv6("2001:0da8:0207:0000:0000:0000:0000:8207");
        assert_eq!(
            result,
            "invalid IPv4 address syntax: 2001:0da8:0207:0000:0000:0000:0000:8207"
        );
    }

    #[test]
    fn test_ipv4_to_ipv6_input_is_non_ip() {
        let result = ipv4_to_ipv6("hello world");
        assert_eq!(result, "invalid IPv4 address syntax: hello world");
    }
}
