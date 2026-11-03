#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| ui::fuzz::pages(data));
