# Gandiva Rust UDF
This project is a collection of UDFs written in Rust for Apache Arrow Gandiva.

# How to write a new UDF
- new a function package under the repo root
```bash
cargo new my_func --lib
```
- make type cdylib and add my_func as a workspace member in gandiva_rust_udf/Cargo.toml
```toml
# in gandiva_rust_udf/Cargo.toml
...
[lib]
crate-type = ["cdylib"]

[workspace]
members = [
    "my_func",
]

[dependencies]
libc = "0.2.152"
gandiva_rust_udf_macro = { version = "0.1.3" }
gandiva_rust_udf_shared = { version = "0.1.2" }
```
- go to gandiva_rust_udf/my_func/Cargo.toml and add gandiva_rust_udf_macro and gandiva_rust_udf_shared in dependencies
```toml
# in gandiva_rust_udf/my_func/Cargo.toml
[package]
name = "my_func"
version = "0.0.1"
edition = "2021"

[lib]
name = "my_func"
path = "src/lib.rs"

[dependencies]
# if your function requires string as parameters or return value, you should add libc as a dependency
libc = { workspace = true }
gandiva_rust_udf_macro = { workspace = true }
gandiva_rust_udf_shared = { workspace = true }
```

- code the function in gandiva_rust_udf/my_func/src/lib.rs for example
```rust
use gandiva_rust_udf_macro::udf;

#[udf]
fn add_one(x: i64) -> i64 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        let result = add_one(6);
        assert_eq!(result, 7);
    }
}

```
- install gandiva_rust_udf_build to generate udf registry
```bash
cargo install gandiva_rust_udf_build
```
- back to gandiva_rust_udf and generate udf registry
```bash
gen-udf-reg
```
- build gandiva_rust_udf and get the lib file libgdv_rust_udf_cus.dylib or libgdv_rust_udf_cus.so
```bash
cargo build --lib --release --workspace
```
- copy the lib file to stonewave path
```bash
${STONEWAVE_HOME}/var/scalar_funcs
```

# How to create a different project like this
- create a new cargo package named gandiva_rust_udf
```bash
cargo new gandiva_rust_udf_cus --lib
```
- go to gandiva_rust_udf_cus and repeat like this [How to write a new UDF](#how_to_write_a_new_udf)

# Function List
| signature |
| -- |
|utf8 bar(int64, int64, int64, int64)|
|utf8 conv(utf8, int64, int64)|
|utf8 format(utf8, utf8)|
|utf8 format(utf8, utf8, utf8)|
|utf8 format(utf8, utf8, utf8, utf8)|
|utf8 format(utf8, utf8, utf8, utf8, utf8)|
|utf8 format(utf8, utf8, utf8, utf8, utf8, utf8)|
|bool is_ipv4(utf8)|
|bool is_ipv6(utf8)|
|bool is_ipv4_loopback(utf8)|
|bool is_ipv6_loopback(utf8)|
|bool ipv4_to_ipv6(utf8)|
|bool is_ascii(utf8)|
|int64 gcd(int64, int64)|
|int64 lcm(int64, int64)|
|float64 jaro_similarity(utf8, utf8)|
|float64 jaro_winkler_similarity(utf8, utf8)|
|int64 damerau_levenshtein_distance(utf8, utf8)|
|utf8 hamming_distance(utf8, utf8)|
|float64 normalized_damerau_levenshtein_distance(utf8, utf8)|
|float64 normalized_levenshtein_distance(utf8, utf8)|
|int64 osa_distance(utf8, utf8)|
|float64 sorensen_dice_similarity(utf8, utf8)|
|utf8 protocol(utf8)|
|utf8 domain(utf8)|
|utf8 domain_without_www(utf8)|
|utf8 top_level_domain(utf8)|
|utf8 port(utf8)|
|utf8 path(utf8)|
|utf8 path_full(utf8)|
|utf8 query_string(utf8)|
|utf8 fragment(utf8)|
|utf8 netloc_username(utf8)|
|utf8 netloc_password(utf8)|
|utf8 netloc(utf8)|
|bool is_valid_url(utf8)|
|utf8 cut_www(utf8)|
|utf8 cut_query_string(utf8)|
|utf8 cut_query_string_and_fragment(utf8)|
|utf8 uuid()|
|bool valid_json(utf8)|
