use gandiva_rust_udf_macro::udf;

#[udf]
fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        let result = uuid();
        assert_eq!(result.len(), 36);
    }
}
