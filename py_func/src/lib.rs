use gandiva_rust_udf_macro::udf;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[udf]
pub fn py(script: &str, arg: &str) -> String {
    // use pyo3 to run the python script, and pass the arg to the script
    // return the result of the script
    let mut ret: String = "".to_string();
    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        locals.set_item("arg", arg).unwrap();
        let result = py.run(&script, None, Some(locals));
        match result {
            Ok(_) => {
                let result = locals.get_item("result").unwrap();
                ret = result
                    .expect("`result` should not be empty")
                    .extract::<String>()
                    .unwrap();
            }
            Err(e) => {
                ret = format!("Error: {:?}", e);
            }
        }
    });
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py() {
        let script = "result = arg.upper()";
        let value = py(script, "hello");
        assert_eq!(value, "HELLO");
    }

    #[test]
    fn test_remove_aeiou() {
        let script = r#"import re; result = re.sub(r'[aeiou]', '', arg)"#;
        let value = py(script, "hello");
        assert_eq!(value, "hll");
    }
}
