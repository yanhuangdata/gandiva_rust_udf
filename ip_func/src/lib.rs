use gandiva_rust_udf_macro::udf;
use gandiva_rust_udf_macro::context_fns;
use std::net::{Ipv4Addr, Ipv6Addr};

context_fns!();

#[udf]
pub fn is_ipv4(addr: &str) -> bool {
    addr.parse::<Ipv4Addr>().is_ok()
}

#[udf]
pub fn is_ipv6(addr: &str) -> bool {
    addr.parse::<Ipv6Addr>().is_ok()
}

#[udf]
pub fn is_ipv4_loopback(addr: &str) -> bool {
    addr.parse::<Ipv4Addr>().map_or(false, |ip| ip.is_loopback())
}

#[udf]
pub fn is_ipv6_loopback(addr: &str) -> bool {
    addr.parse::<Ipv6Addr>().map_or(false, |ip| ip.is_loopback())
}

#[udf]
pub fn ipv4_string_to_num(addr: &str) -> u32 {
    let ip = addr.parse::<Ipv4Addr>().unwrap();
    let octets = ip.octets();
    let mut num = 0;
    for i in 0..4 {
        num += (octets[i] as u32) << (8 * (3 - i));
    }
    num
}

#[udf(needs_context = true)]
pub fn ipv4_num_to_string(addr: u32) -> String {
    let mut octets = [0u8; 4];
    for i in 0..4 {
        octets[i] = ((addr >> (8 * (3 - i))) & 0xff) as u8;
    }
    Ipv4Addr::from(octets).to_string()
}

#[udf(needs_context = true)]
pub fn ipv4_to_ipv6(ipv4: &str) -> String {
    let ipv4_addr = ipv4.parse::<Ipv4Addr>().ok().unwrap();
    let ipv6_components = ipv4_addr.octets();
    let ipv6_addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff,
                                  ((ipv6_components[0] as u16) << 8) + ipv6_components[1] as u16,
                                  ((ipv6_components[2] as u16) << 8) + ipv6_components[3] as u16);
    ipv6_addr.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ipv4() {
        let result = is_ipv4("192.168.0.1");
        assert!(result);
    }

    #[test]
    fn test_is_ipv6() {
        let result = is_ipv6("192.168.0.1");
        assert!(!result);
    }

    #[test]
    fn test_is_ipv4_loopback() {
        let result = is_ipv4_loopback("127.0.0.1");
        assert!(result);
    }

    #[test]
    fn test_is_ipv6_loopback() {
        let result = is_ipv6_loopback("127.0.0.1");
        assert!(!result);
    }

    #[test]
    fn test_ipv4_string_to_num() {
        let result = ipv4_string_to_num("255.255.255.255");
        assert_eq!(result, 4294967295);
    }

    #[test]
    fn test_ipv4_num_to_string() {
        let result = ipv4_num_to_string(4294967295);
        assert_eq!(result, "255.255.255.255");
    }

    #[test]
    fn test_ipv4_to_ipv6() {
        let result = ipv4_to_ipv6("192.168.0.1");
        assert_eq!(result, "::ffff:192.168.0.1");
    }
}
