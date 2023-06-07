use gandiva_rust_udf_macro::udf;
use gandiva_rust_udf_macro::context_fns;

context_fns!();

#[udf(needs_context = true)]
pub fn rust_to_hex(value: i64) -> String {
    format!("{:x}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hex() {
        let value = rust_to_hex(123);
        assert_eq!(value, "7b");
    }
}
