use strfmt::strfmt;
use std::collections::HashMap;
use gandiva_rust_udf_macro::udf;
use gandiva_rust_udf_macro::context_fns;

context_fns!();

#[udf(needs_context = true)]
pub fn format(template: &str, value1: &str) -> String {
    let mut vars: HashMap<String, &str> = HashMap::default();
    vars.insert("1".to_string(), value1);
    strfmt(template, &vars).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_string() {
        let value = format("Hello, {1}!", "world");
        assert_eq!(value, "Hello, world!");
    }
}
