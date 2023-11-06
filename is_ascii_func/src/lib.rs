use gandiva_rust_udf_macro::udf;

#[udf]
fn is_ascii(data: &str) -> bool {
    data.is_ascii()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ascii() {
        assert!(is_ascii("hello"));
        assert!(!is_ascii("你好"));
    }
}
