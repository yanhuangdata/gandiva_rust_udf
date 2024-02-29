pub use pi_func::pi;
use gandiva_rust_udf_macro::udf_registry;

// FIXME: use build.rs to automate the registration of all functions
#[udf_registry]
pub fn register_all_funcs() {
    valid_json_func::register_valid_json_utf8();
    is_ascii_func::register_is_ascii_utf8();
    format_func::register_format_1_utf8_utf8();
    format_func::register_format_2_utf8_utf8_utf8();
    format_func::register_format_3_utf8_utf8_utf8_utf8();
    format_func::register_format_4_utf8_utf8_utf8_utf8_utf8();
    format_func::register_format_5_utf8_utf8_utf8_utf8_utf8_utf8();
    conv_func::register_conv_utf8_int64_int64();
    ip_func::register_is_ipv4_utf8();
    ip_func::register_is_ipv6_utf8();
    ip_func::register_is_ipv4_loopback_utf8();
    ip_func::register_is_ipv6_loopback_utf8();
    ip_func::register_ipv4_to_ipv6_utf8();
    uuid_func::register_uuid_();
    url_func::register_is_valid_url_utf8();
    url_func::register_protocol_utf8();
    url_func::register_domain_utf8();
    url_func::register_domain_without_www_utf8();
    url_func::register_top_level_domain_utf8();
    url_func::register_port_utf8();
    url_func::register_path_utf8();
    url_func::register_path_full_utf8();
    url_func::register_query_string_utf8();
    url_func::register_fragment_utf8();
    url_func::register_netloc_username_utf8();
    url_func::register_netloc_password_utf8();
    url_func::register_netloc_utf8();
    url_func::register_cut_www_utf8();
    url_func::register_cut_query_string_utf8();
    url_func::register_cut_query_string_and_fragment_utf8();
    strsim_func::register_jaro_similarity_utf8_utf8();
    strsim_func::register_jaro_winkler_similarity_utf8_utf8();
    strsim_func::register_damerau_levenshtein_distance_utf8_utf8();
    strsim_func::register_hamming_distance_utf8_utf8();
    strsim_func::register_normalized_damerau_levenshtein_distance_utf8_utf8();
    strsim_func::register_normalized_levenshtein_distance_utf8_utf8();
    strsim_func::register_osa_distance_utf8_utf8();
    strsim_func::register_sorensen_dice_similarity_utf8_utf8();
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
        let registry_c_str = gandiva_rust_udf_shared::get_udf_registry();
        unsafe {
            let registry = std::ffi::CString::from_raw(registry_c_str);
            let registry_str = registry.to_str().unwrap();
            let udf_registry: gandiva_rust_udf_shared::UdfRegistry = serde_json::from_str(registry_str).unwrap();
            assert_eq!(udf_registry.functions.len(), 38);
        }
    }
}