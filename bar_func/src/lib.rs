use gandiva_rust_udf_macro::udf;

#[udf]
fn bar(x: i64, min: i64, max: i64, width: i64) -> String {
    let blocks = [" ", "▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];

    if x <= min {
        return blocks[0].repeat(width as usize);
    }
    if x >= max {
        return blocks[8].repeat(width as usize);
    }

    let ratio = (x - min) as f64 / (max - min) as f64;
    let total_length = ratio * width as f64;
    let full_blocks = total_length.trunc() as usize;
    let remainder = total_length - total_length.trunc();
    let partial_block_index = (remainder * 8.0).round() as usize;

    let mut result = blocks[8].repeat(full_blocks);
    if full_blocks < width as usize {
        result.push_str(blocks[partial_block_index]);
        result.push_str(&blocks[0].repeat(width as usize - full_blocks - 1));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar() {
        assert_eq!(bar(00, 0, 010, 10), "          ");
        assert_eq!(bar(01, 0, 010, 10), "█         ");
        assert_eq!(bar(02, 0, 010, 10), "██        ");
        assert_eq!(bar(01, 0, 100, 10), "▏         ");
        assert_eq!(bar(02, 0, 100, 10), "▎         ");
        assert_eq!(bar(12, 0, 100, 10), "█▎        ");
        assert_eq!(bar(50, 0, 100, 10), "█████     ");
    }

    #[test]
    fn test_bar_over_max() {
        assert_eq!(bar(20, 0, 020, 10), "██████████");
        assert_eq!(bar(30, 0, 020, 10), "██████████");
    }

    #[test]
    fn test_bar_remainder_in_last_char() {
        // 95 ==> 9.5 ==> 9 full blocks and 4/8 of a block
        assert_eq!(bar(95, 0, 100, 10), "█████████▌");
        assert_eq!(bar(96, 0, 100, 10), "█████████▋");
    }

    #[test]
    fn test_bar_with_different_width() {
        assert_eq!(bar(01, 0, 015, 15), "█              ");
    }
}
