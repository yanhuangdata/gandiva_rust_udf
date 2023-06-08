use radix_fmt::radix;
use gandiva_rust_udf_macro::udf;
use gandiva_rust_udf_macro::context_fns;

context_fns!();

#[udf(needs_context = true)]
pub fn conv(num: &str, from_radix: i64, to_radix: i64) -> String {
    let value = i64::from_str_radix(num, from_radix as u32);
    match value {
        Ok(v) => radix(v, to_radix as u8).to_string(),
        Err(_) => String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv_binary_to_decimal() {
        let result = conv("100", 2, 10);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_conv_binary_to_hex() {
        let result = conv("1110", 2, 16);
        assert_eq!(result, "e");
    }
}
