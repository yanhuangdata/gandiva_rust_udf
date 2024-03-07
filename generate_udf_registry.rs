//! ```cargo
//! [dependencies]
//! gandiva_rust_udf_build = "0.1.0"
//! ```
extern crate gandiva_rust_udf_build;

use std::env;

use gandiva_rust_udf_build::generate_udf_registry;

fn main() {
    match env::current_dir() {
        Ok(result) => {
            generate_udf_registry(&result);
        }
        Err(e) => println!("failed to get current dir: {:?}", e),
    }
}
