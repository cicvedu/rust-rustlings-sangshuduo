use std::env;

fn main() {
    println!("cargo:rustc-cfg=feature=\"pass\"");
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    // Set TEST_FOO environment variable within the expected range
    let start_range = timestamp;

    println!("cargo:rustc-env=TEST_FOO={}", start_range);
}
