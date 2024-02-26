use radix_fmt::radix;
use gandiva_rust_udf_macro::udf;

#[udf]
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
    fn test_conv_decimal_to_binary(){
        assert_eq!(conv("5", 10, 2), "101");
    }

    #[test]
    fn test_conv_binary_to_hex() {
        let result = conv("1110", 2, 16);
        assert_eq!(result, "e");
    }

    #[test]
    fn test_conv_hex_to_binary() {
        let result = conv("e", 16, 2);
        assert_eq!(result, "1110");
    }

    #[test]
    fn test_conv_decimal_to_hex(){
        assert_eq!(conv("255", 10, 16), "ff");
    }

    #[test]
    fn test_conv_hex_to_decimal(){
        assert_eq!(conv("A", 16, 10), "10");
    }

    #[test]
    fn test_conv_hexatridecimal_to_decimal(){
       assert_eq!(conv("21", 36, 10), "73");
    }
    #[test]
    fn test_conv_hexatridecimal_without_num_to_decima(){
        assert_eq!(conv("HELLO", 36, 10), "29234652");
    }

    #[test]
    fn test_empty_str_conv(){
        assert_eq!(conv("", 2, 16), "");
    }

    #[test]
    fn test_str_without_num_conv(){
        assert_eq!(conv("HELLO", 2, 10), "");
    }



}
