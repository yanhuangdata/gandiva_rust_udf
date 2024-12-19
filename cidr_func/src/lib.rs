use gandiva_rust_udf_macro::udf;
use ipnetwork::IpNetwork;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

#[udf]
pub fn netmask(ip: &str) -> Result<String, String> {
    match ip.parse::<IpNetwork>() {
        Ok(network) => Ok(network.mask().to_string()),
        Err(_) => Ok("Invalid cidr specification".to_string())
    }
}

#[udf]
pub fn base_ip(ip: &str) -> Result<String, String> {
    match ip.parse::<IpNetwork>() {
        Ok(network) => Ok(network.ip().to_string()),
        Err(_) => Ok("Invalid cidr specification".to_string())
    }
}

#[udf]
pub fn broadcast_ip(ip: &str) -> Result<String, String> {
    match ip.parse::<IpNetwork>() {
        Ok(network) => Ok(network.broadcast().to_string()),
        Err(_) => Ok("Invalid cidr specification".to_string())
    }
}

#[udf]
pub fn host_count(ip: &str) -> Result<String, String> {
    let network: IpNetwork = match ip.parse::<IpNetwork>() {
        Ok(network) => network,
        Err(_) => return Ok("Invalid cidr specification".to_string())
    };

    match network {
        IpNetwork::V4(net) => {
            let bits = 32 - net.prefix();
            if bits > 32 {
                return Ok("Network size too large".to_string());
            }
            Ok((1u128 << bits).to_string())
        }
        IpNetwork::V6(net) => {
            // let bits = 128 - net.prefix();
            // if bits > 128 {
            //     return Ok("Network size too large".to_string());
            // }
            // Ok((1u128 << bits).to_string())

            let prefix_len = net.prefix();
            let host_bits = 128u128 - prefix_len as u128;
            let network_size = if host_bits == 128 {
                u128::MAX
            } else {
                1u128 << host_bits
            };
            Ok(network_size.to_string())
        }
    }
}

#[udf]
pub fn first_usable_ip(ip: &str) -> Result<String, String> {
    match ip.parse::<IpNetwork>() {
        Ok(network) => {
            match network {
                IpNetwork::V4(net) => {
                    if net.prefix() == 32 {
                        Ok(net.network().to_string())
                    } else {
                        Ok(Ipv4Addr::from(u32::from(net.network()).saturating_add(1)).to_string())
                    }
                },
                IpNetwork::V6(net) => Ok(net.network().to_string())
            }
        }
        Err(_) => Ok("Invalid cidr specification".to_string())
    }
}

#[udf]
pub fn last_usable_ip(ip: &str) -> Result<String, String> {
    match ip.parse::<IpNetwork>() {
        Ok(network) => {
            match network {
                IpNetwork::V4(ipv4_network) => {
                    if ipv4_network.prefix() >= 31 {
                        Ok(ipv4_network.broadcast().to_string())
                    } else {
                        let broadcast_addr = ipv4_network.broadcast();
                        let last_usable = Ipv4Addr::from(u32::from(broadcast_addr) - 1);
                        Ok(last_usable.to_string())
                    }
                },
                IpNetwork::V6(ipv6_network) => {
                    let prefix_len = ipv6_network.prefix();
                    let host_bits = 128u128 - prefix_len as u128;
                    let network_size = if host_bits == 128 {
                        u128::MAX
                    } else {
                        1u128 << host_bits
                    };
                    let last = Ipv6Addr::from(
                        u128::from(ipv6_network.network())
                            .wrapping_add(network_size - 1)
                    );
                    Ok(last.to_string())
                }
            }
        },
        Err(_) => Ok("Invalid cidr specification".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn netmask_ipv4_works() {
        let result = netmask("10.88.135.144/28");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "255.255.255.240");
    }


    #[test]
    fn netmask_ipv6_works() {
        let result = netmask("2001:db8::/32");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "ffff:ffff::");
    }

    #[test]
    fn netmask_invalid() {
        let result = netmask("invalid");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "Invalid cidr specification");
    }

    #[test]
    fn base_ip_ipv4_works() {
        let result = base_ip("10.88.135.144/28");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "10.88.135.144");
    }

    #[test]
    fn base_ip_ipv6_works() {
        let result = base_ip("2001:db8::/32");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "2001:db8::");
    }

    #[test]
    fn base_ip_invalid() {
        let result = base_ip("invalid");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "Invalid cidr specification");
    }

    #[test]
    fn broadcast_ip_ipv4_works() {
        let result = broadcast_ip("10.88.135.144/28");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "10.88.135.159");
    }

    #[test]
    fn broadcast_ip_ipv6_works() {
        let result = broadcast_ip("2001:db8::/32");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff");
    }

    #[test]
    fn broadcast_ip_invalid() {
        let result = broadcast_ip("invalid");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "Invalid cidr specification");
    }

    #[test]
    fn count_ipv4_works() {
        let result = host_count("10.88.135.144/28");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "16");
    }

    #[test]
    fn count_ipv6_works() {
        let result = host_count("2001:db8::/32");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "79228162514264337593543950336");
    }

    #[test]
    fn count_invalid() {
        let result = host_count("invalid");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "Invalid cidr specification");
    }

    #[test]
    fn first_usable_ip_ipv4_works() {
        let result = first_usable_ip("10.88.135.144/28");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "10.88.135.145");
    }

    #[test]
    fn first_usable_ip_ipv6_works() {
        let result = first_usable_ip("2001:db8::/32");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "2001:db8::");
    }

    #[test]
    fn first_usable_ip_invalid() {
        let result = first_usable_ip("invalid");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "Invalid cidr specification");
    }
    #[test]
    fn last_usable_ip_ipv4_works() {
        let result = last_usable_ip("10.88.135.144/28");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "10.88.135.158");
    }

    #[test]
    fn last_usable_ip_ipv6_works() {
        let result = last_usable_ip("2001:db8::/32");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "2001:db8:ffff:ffff:ffff:ffff:ffff:ffff");
    }

    #[test]
    fn last_usable_ip_invalid() {
        let result = last_usable_ip("invalid");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "Invalid cidr specification");
    }

    #[test]
    fn test_ipv4_last_usable_ip() {
        let result = last_usable_ip("192.168.1.0/24");
        assert!(result.is_ok());
        let ip = result.unwrap();
        assert_eq!(ip, "192.168.1.254");
    }

    #[test]
    fn test_ipv4_last_usable_ip_with_prefix_less_than_31() {
        let result = last_usable_ip("192.168.1.0/30");
        assert!(result.is_ok());
        let ip = result.unwrap();
        assert_eq!(ip, "192.168.1.2");
    }

    #[test]
    fn test_ipv6_last_usable_ip() {
        let result = last_usable_ip("2001:db8::/64");
        assert!(result.is_ok());
        let ip = result.unwrap();
        assert_eq!(ip, "2001:db8::ffff:ffff:ffff:ffff");
    }

    #[test]
    fn test_ipv6_last_usable_ip_with_prefix_less_than_31() {
        let result = last_usable_ip("2001:db8::/96");
        assert!(result.is_ok());
        let ip = result.unwrap();
        assert_eq!(ip, "2001:db8::ffff:ffff");
    }

    #[test]
    fn test_invalid_specification() {
        let result = last_usable_ip("256.256.256.256/24");
        assert!(result.is_ok());
        let error_message = result.unwrap();
        assert_eq!(error_message, "Invalid cidr specification");
    }

    #[test]
    fn test_with_single_ip() {
        let result = last_usable_ip("192.168.1.1/32");
        assert!(result.is_ok());
        let ip = result.unwrap();
        assert_eq!(ip, "192.168.1.1");
    }

    #[test]
    fn netmask_ipv4_1_works() {
        let result = netmask("10.88.135.144/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "0.0.0.0");
    }

    #[test]
    fn base_ip_ipv4_1_works() {
        let result = base_ip("10.88.135.144/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "10.88.135.144");
    }

    #[test]
    fn broadcast_ip_ipv4_1_works() {
        let result = broadcast_ip("10.88.135.144/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "255.255.255.255");
    }

    #[test]
    fn network_size_ipv4_1_works() {
        let result = host_count("10.88.135.144/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "4294967296");
    }

    #[test]
    fn first_usable_ip_ipv4_1_works() {
        let result = first_usable_ip("10.88.135.144/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "0.0.0.1");
    }

    #[test]
    fn last_usable_ip_ipv4_1_works() {
        let result = last_usable_ip("10.88.135.144/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "255.255.255.254");
    }


    #[test]
    fn netmask_ipv6_1_works() {
        let result = netmask("2001:db8::/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "::");
    }

    #[test]
    fn base_ip_ipv6_1_works() {
        let result = base_ip("2001:db8::/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "2001:db8::");
    }

    #[test]
    fn broadcast_ip_ipv6_1_works() {
        let result = broadcast_ip("2001:db8::/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff");
    }

    #[test]
    fn network_size_ipv6_1_works() {
        let result = host_count("2001:db8::/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "340282366920938463463374607431768211455");
    }

    #[test]
    fn first_usable_ip_ipv6_1_works() {
        let result = first_usable_ip("2001:db8::/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "::");
    }

    #[test]
    fn last_usable_ip_ipv6_1_works() {
        let result = last_usable_ip("2001:db8::/0");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "ffff:ffff:ffff:ffff:ffff:ffff:ffff:fffe");
    }

}
