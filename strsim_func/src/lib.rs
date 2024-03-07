use gandiva_rust_udf_macro::udf;
use strsim::damerau_levenshtein;
use strsim::hamming;
use strsim::jaro;
use strsim::jaro_winkler;
use strsim::normalized_damerau_levenshtein;
use strsim::normalized_levenshtein;

#[udf]
pub fn jaro_similarity(a: &str, b: &str) -> f64 {
    return jaro(a, b);
}

#[udf]
pub fn jaro_winkler_similarity(a: &str, b: &str) -> f64 {
    return jaro_winkler(a, b);
}

#[udf]
pub fn damerau_levenshtein_distance(a: &str, b: &str) -> i64 {
    if let Ok(i64_value) = damerau_levenshtein(a, b).try_into() {
        i64_value
    } else {
        0
    }
}

#[udf]
pub fn hamming_distance(a: &str, b: &str) -> String {
    let value = hamming(a, b);
    match value {
        Ok(v) => v.to_string(),
        Err(e) => e.to_string(),
    }
}

#[udf]
pub fn normalized_damerau_levenshtein_distance(a: &str, b: &str) -> f64 {
    return normalized_damerau_levenshtein(a, b);
}

#[udf]
pub fn normalized_levenshtein_distance(a: &str, b: &str) -> f64 {
    return normalized_levenshtein(a, b);
}

#[udf]
pub fn osa_distance(a: &str, b: &str) -> i64 {
    if let Ok(i64_value) = strsim::osa_distance(a, b).try_into() {
        i64_value
    } else {
        0
    }
}

#[udf]
pub fn sorensen_dice_similarity(a: &str, b: &str) -> f64 {
    return strsim::sorensen_dice(a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        let result = hamming_distance("clickhouse", "click");
        assert_eq!(result, "Differing length arguments provided");
    }
}
