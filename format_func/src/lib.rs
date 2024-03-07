use gandiva_rust_udf_macro::udf;
use std::collections::HashMap;
use strfmt::strfmt;

#[udf(name = "format")]
pub fn format_1(template: &str, value1: &str) -> String {
    let values = &[value1];
    return format(template, values);
}

#[udf(name = "format")]
pub fn format_2(template: &str, value1: &str, value2: &str) -> String {
    let values = &[value1, value2];
    return format(template, values);
}

#[udf(name = "format")]
pub fn format_3(template: &str, value1: &str, value2: &str, value3: &str) -> String {
    let values = &[value1, value2, value3];
    return format(template, values);
}

#[udf(name = "format")]
pub fn format_4(template: &str, value1: &str, value2: &str, value3: &str, value4: &str) -> String {
    let values = &[value1, value2, value3, value4];
    return format(template, values);
}

#[udf(name = "format")]
pub fn format_5(
    template: &str,
    value1: &str,
    value2: &str,
    value3: &str,
    value4: &str,
    value5: &str,
) -> String {
    let values = &[value1, value2, value3, value4, value5];
    return format(template, values);
}

pub fn format(template: &str, values: &[&str]) -> String {
    let mut vars: HashMap<String, &str> = HashMap::default();
    for (i, value) in values.iter().enumerate() {
        vars.insert((i + 1).to_string(), value);
    }
    match strfmt(template, &vars) {
        Ok(result) => result,
        Err(e) => {
            let error_msg_vals = values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "Formatting with template {} and values {} caused an error: {:?}",
                template, error_msg_vals, e
            )
        }
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn test_format_1_string() {
        let value = format_1("Hello, {1}!", "world");
        assert_eq!(value, "Hello, world!");
    }

    #[test]
    fn test_format_string_1_with_undefied_key() {
        let value = format_1("Hello, {3}!", "world");
        assert_eq!(value, "Formatting with template Hello, {3}! and values world caused an error: KeyError(\"Invalid key: 3\")");
    }

    #[test]
    fn test_format_2_string() {
        let value = format_2("Hello, {1},{2}!", "world", "yhp");
        assert_eq!(value, "Hello, world,yhp!");
    }

    #[test]
    fn test_format_2_string_with_invalid_key() {
        let value = format_2("Hello, {1},{3}!", "world", "yhp");
        assert_eq!(value, "Formatting with template Hello, {1},{3}! and values world, yhp caused an error: KeyError(\"Invalid key: 3\")");
    }

    #[test]
    fn test_format_2_string_with_all_invalid_key() {
        let value = format_2("Hello, {4},{3}!", "world", "yhp");
        assert_eq!(value, "Formatting with template Hello, {4},{3}! and values world, yhp caused an error: KeyError(\"Invalid key: 4\")");
    }
}
