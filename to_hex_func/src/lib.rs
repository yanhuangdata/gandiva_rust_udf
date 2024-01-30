use gandiva_rust_udf_macro::udf;

#[udf]
fn rust_to_hex(value: i64) -> String {
    let result = format!("{:x}", value);
    result
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
