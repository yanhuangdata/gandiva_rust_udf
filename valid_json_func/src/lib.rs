use gandiva_rust_udf_macro::udf;
use serde_json::Value;

#[udf]
fn valid_json(json_str: &str) -> bool {
    match serde_json::from_str::<Value>(json_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_json() {
        let json_str = r#"{"name":"John","age":30,"city":"New York"}"#;
        assert_eq!(valid_json(json_str), true);
    }

    #[test]
    fn test_is_valid_json_invalid() {
        let json_str = r#"{"name":"John","age":}"#;
        assert_eq!(valid_json(json_str), false);
    }
}
