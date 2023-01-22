// sopa's:5743c28f4d3346a8ee6822283f7a0b4f0ddb0964a6f284c8f9486dcb9b7ff59d7af1c5b828a997006d280f1a5737b783b4ebfd5e20e89f13530bf4defe1a72a0

const HWID: &str = "5743c28f4d3346a8ee6822283f7a0b4f0ddb0964a6f284c8f9486dcb9b7ff59d7af1c5b828a997006d280f1a5737b783b4ebfd5e20e89f13530bf4defe1a72a0";

use std::env;

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }
    println!("cargo:rustc-env=HWID={}", HWID);
}