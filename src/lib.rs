// src/lib.rs - Required for Android APK

pub fn hello_android() {
    println!("Hello from Rust on Android!");
}

// Android entry point
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(_app: android_activity::AndroidApp) {
    hello_android();
    
    // Keep app running
    std::thread::sleep(std::time::Duration::from_secs(10));
}
