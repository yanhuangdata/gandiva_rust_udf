use gandiva_rust_udf_macro::udf;

#[udf]
fn gcd(x: i64, y: i64) -> i64 {
    num_integer::gcd(x, y)
}

#[udf]
fn lcm(x: i64, y: i64) -> i64 {
    num_integer::lcm(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let result = gcd(6, 8);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_lcm() {
        let result = lcm(6, 8);
        assert_eq!(result, 24);
    }
}
