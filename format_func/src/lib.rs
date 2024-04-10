use gandiva_rust_udf_macro::udf;
use std::collections::HashMap;
use strfmt::strfmt;

#[udf(name = "format")]
pub fn format_1(template: &str, value1: &str) -> Result<String, String> {
    let values = &[value1];
    return format(template, values);
}

#[udf(name = "format")]
pub fn format_2(template: &str, value1: &str, value2: &str) -> Result<String, String> {
    let values = &[value1, value2];
    return format(template, values);
}

#[udf(name = "format")]
pub fn format_3(
    template: &str,
    value1: &str,
    value2: &str,
    value3: &str,
) -> Result<String, String> {
    let values = &[value1, value2, value3];
    return format(template, values);
}

#[udf(name = "format")]
pub fn format_4(
    template: &str,
    value1: &str,
    value2: &str,
    value3: &str,
    value4: &str,
) -> Result<String, String> {
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
) -> Result<String, String> {
    let values = &[value1, value2, value3, value4, value5];
    return format(template, values);
}

pub fn format(template: &str, values: &[&str]) -> Result<String, String> {
    let mut vars: HashMap<String, &str> = HashMap::default();
    for (i, value) in values.iter().enumerate() {
        vars.insert((i + 1).to_string(), value);
    }
    match strfmt(template, &vars) {
        Ok(result) => Ok(result),
        Err(e) => {
            let _error_msg_vals = values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            Err(format!(
                "Formatting with template '{}' caused an error: {:?}",
                template, e
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_1_string() {
        let result = format_1("Hello, {1}!", "world");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "Hello, world!");
    }

    #[test]
    fn test_format_string_1_with_undefied_key() {
        let result = format_1("Hello, {3}!", "world");
        assert!(result.is_err());
        let value = result.err().unwrap();
        assert_eq!(
            value,
            "Formatting with template 'Hello, {3}!' caused an error: KeyError(\"Invalid key: 3\")"
        );
    }

    #[test]
    fn test_format_2_string() {
        let result = format_2("Hello, {1},{2}!", "world", "yhp");
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value, "Hello, world,yhp!");
    }

    #[test]
    fn test_format_2_string_with_invalid_key() {
        let result = format_2("Hello, {1},{3}!", "world", "yhp");
        assert!(result.is_err());
        let value = result.err().unwrap();
        assert_eq!(value, "Formatting with template 'Hello, {1},{3}!' caused an error: KeyError(\"Invalid key: 3\")");
    }

    #[test]
    fn test_format_2_string_with_all_invalid_key() {
        let result = format_2("Hello, {4},{3}!", "world", "yhp");
        assert!(result.is_err());
        let value = result.err().unwrap();
        assert_eq!(value, "Formatting with template 'Hello, {4},{3}!' caused an error: KeyError(\"Invalid key: 4\")");
    }
}
