const HWID: &str = "8f314714555939bb057b5157ccb8338124276d2870660a4c4d4d1138dfabcc2c9eac246f615a6d1ee3a06fd4ca48f8978af293ef94d64118b1f2e385e5bc146f";

use std::env;

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }
    println!("cargo:rustc-env=HWID={}", HWID);
}