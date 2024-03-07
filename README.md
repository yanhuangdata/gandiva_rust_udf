# Gandiva Rust UDF
This project is a collection of UDFs written in Rust for Apache Arrow Gandiva.

# How to write a new UDF
1. Create a new cargo workspace, like `my_func`
2. Two dependencies should be added for this new workspace:
    1. gandiva_rust_udf_macro 
    2. gandiva_rust_udf_shared
3. If your function accepts string parameter or returns string value, you should add `libc` as dependency
4. Add a new function in `my_func/src/lib.rs` 
   1. import `use gandiva_rust_udf_macro::udf`
   2. use `#[udf]` macro to annotate this function
   3. Unit test your function
5. Add `my_func` as dependency in `udf_core/Cargo.toml`
6. Register `my_func` in `udf_core/src/lib.rs`
7. Compile `udf_core` as shared library
    1. `cargo build --release --lib --workspace`
8. Deploy `libgandiva_rust_udf` shared library to where your Gandiva application can load it

# How to create a different project like this
1. udf_core
```
[lib]
crate-type = ["cdylib"]
```

1. each function MUST depends on `gandiva_rust_udf_shared` crate
