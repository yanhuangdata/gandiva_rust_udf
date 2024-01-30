use gandiva_rust_udf_macro::udf;

#[udf]
pub fn pi() -> f64 {
    std::f64::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi() {
        let value = pi();
        assert!(value > 3.14);
    }
}
