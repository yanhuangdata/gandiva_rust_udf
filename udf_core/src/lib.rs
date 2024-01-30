pub use pi_func::pi;
use gandiva_rust_udf_shared::{get_udf_registry, free_udf_registry};

// FIXME: use build.rs to automate the registration of all functions
pub fn register_all_funcs() {
    pi_func::register_pi_();
    valid_json_func::register_valid_json_utf8();
    to_hex_func::register_rust_to_hex_int64();
    num_func::register_gcd_int64_int64();
    num_func::register_lcm_int64_int64();
    rad_func::register_rad_to_degree_float64();
    rad_func::register_degree_to_rad_float64();
    is_ascii_func::register_is_ascii_utf8();
    format_func::register_format_utf8_utf8();
    conv_func::register_conv_utf8_int64_int64();
    ip_func::register_is_ipv4_utf8();
    ip_func::register_is_ipv6_utf8();
    ip_func::register_is_ipv4_loopback_utf8();
    ip_func::register_is_ipv6_loopback_utf8();
    uuid_func::register_uuid_();
    url_func::register_is_valid_url_utf8();
    // TODO: there are many other functions in url module
    openai_func::register_askai_utf8();
}

#[no_mangle]
pub extern "C" fn load_registered_udfs() -> *mut libc::c_char {
    register_all_funcs();
    let registry_c_str = get_udf_registry();
    registry_c_str
}

#[no_mangle]
pub extern "C" fn finish_loading_registered_udfs(registry: *mut libc::c_char) {
    free_udf_registry(registry);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi() {
        let value = pi_func::pi();
        assert!(value > 3.14);
    }

    #[test]
    fn test_register_func() {
        register_all_funcs();
        let registry_c_str = get_udf_registry();
        unsafe {
            let registry = std::ffi::CString::from_raw(registry_c_str);
            let registry_str = registry.to_str().unwrap();
            let udf_registry: gandiva_rust_udf_shared::UdfRegistry = serde_json::from_str(registry_str).unwrap();
            assert_eq!(udf_registry.functions.len(), 17);
        }
    }
}