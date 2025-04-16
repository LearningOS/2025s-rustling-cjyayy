//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // Part for tests7 - 设置 TEST_FOO 环境变量
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 设置 TEST_FOO 环境变量为当前时间戳
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Part for tests8 - 启用 "pass" 特性
    // 这会使得 tests8 中的测试提前返回
    println!("cargo:rustc-cfg=feature=\"pass\"");
}