use gandiva_rust_udf_macro::udf;

#[udf]
pub fn degree_to_rad(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}

#[udf]
pub fn rad_to_degree(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_to_rad() {
        let result = degree_to_rad(180.0);
        // assert float with a tolerance
        assert!((result - std::f64::consts::PI).abs() < std::f64::EPSILON);
    }

    #[test]
    fn test_rad_to_degree() {
        let result = rad_to_degree(std::f64::consts::PI);
        // assert float with a tolerance
        assert!((result - 180.0).abs() < std::f64::EPSILON);
    }
}
