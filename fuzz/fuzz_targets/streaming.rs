#![no_main]
use libfuzzer_sys::fuzz_target;

// Placeholder until streaming API exposed more cleanly
fuzz_target!(|data: &[u8]| {
    // Intentionally lightweight: ensure no panics on arbitrary UTF-8
    let _ = std::str::from_utf8(data);
});
