use gandiva_rust_udf_macro::udf;
use radix_fmt::radix;

#[udf]
pub fn conv(num: &str, from_radix: i64, to_radix: i64) -> Result<String, String> {
    if (from_radix < 2 || from_radix > 36) || (to_radix < 2 || to_radix > 36) {
        // return an error with the actual radix given as part of the error message
        return Err(format!(
            "Radix must be between 2 and 36, got from_radix: {}, to_radix: {}",
            from_radix, to_radix
        ));
    }
    let value = i64::from_str_radix(num, from_radix as u32);
    match value {
        Ok(v) => Ok(radix(v, to_radix as u8).to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod conv_tests {
    use super::*;

    fn assert_conv(num: &str, from_radix: i64, to_radix: i64, expected: &str) {
        let result = conv(num, from_radix, to_radix);
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, expected);
    }

    fn assert_conv_err(num: &str, from_radix: i64, to_radix: i64, expected: &str) {
        let result = conv(num, from_radix, to_radix);
        assert!(result.is_err());
        let value = result.err().unwrap();
        assert_eq!(value, expected);
    }

    #[test]
    fn test_conv_binary_to_decimal() {
        assert_conv("100", 2, 10, "4");
    }

    #[test]
    fn test_conv_decimal_to_binary() {
        assert_conv("5", 10, 2, "101");
    }

    #[test]
    fn test_conv_binary_to_hex() {
        assert_conv("1110", 2, 16, "e");
    }

    #[test]
    fn test_conv_hex_to_binary() {
        assert_conv("e", 16, 2, "1110");
    }

    #[test]
    fn test_conv_decimal_to_hex() {
        assert_conv("255", 10, 16, "ff");
    }

    #[test]
    fn test_conv_hex_to_decimal() {
        assert_conv("A", 16, 10, "10");
    }

    #[test]
    fn test_conv_hexatridecimal_to_decimal() {
        assert_conv("21", 36, 10, "73");
    }

    #[test]
    fn test_conv_hexatridecimal_without_num_to_decima() {
        assert_conv("HELLO", 36, 10, "29234652");
    }

    #[test]
    fn test_empty_str_conv() {
        assert_conv_err("", 2, 16, "cannot parse integer from empty string");
    }

    #[test]
    fn test_str_invalid_digit() {
        assert_conv_err("2", 2, 10, "invalid digit found in string");
    }

    #[test]
    fn test_from_radix_too_large() {
        assert_conv_err(
            "2",
            37,
            10,
            "Radix must be between 2 and 36, got from_radix: 37, to_radix: 10",
        );
    }
}
