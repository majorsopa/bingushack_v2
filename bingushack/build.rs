// sopa's:5743c28f4d3346a8ee6822283f7a0b4f0ddb0964a6f284c8f9486dcb9b7ff59d7af1c5b828a997006d280f1a5737b783b4ebfd5e20e89f13530bf4defe1a72a0
// ak's:0ddddca7f1fd2d184486051468fdf056c76959ff3fefd95878e6eed5f6e00b616bbd22df9873a2784d9e9649185e77f8ef8c8f4cee39b78e462af5e0007b4a69
// pig's:c4fb2bbbd59b32d9a1131582af1e170498f02dc4f82e6ece421802aac8d73872f1840c97c09924896f6184a8fd8486338ae639a8d0d886c0f76a1c855ff48c44

const HWID: &str = "5743c28f4d3346a8ee6822283f7a0b4f0ddb0964a6f284c8f9486dcb9b7ff59d7af1c5b828a997006d280f1a5737b783b4ebfd5e20e89f13530bf4defe1a72a0";

use std::env;

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }
    println!("cargo:rustc-env=HWID={}", HWID);
}