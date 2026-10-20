#![no_main]

use libfuzzer_sys::fuzz_target;
use std::path::PathBuf;
use std::sync::OnceLock;

use stonktop::config::Config;

// Config::load only accepts a path, so each input is written to one scratch
// file that is reused across iterations.
static SCRATCH: OnceLock<PathBuf> = OnceLock::new();

fuzz_target!(|data: &[u8]| {
    let path = SCRATCH.get_or_init(|| {
        std::env::temp_dir().join(format!("stonktop-fuzz-{}.toml", std::process::id()))
    });
    if std::fs::write(path, data).is_ok() {
        let _ = Config::load(path);
    }
});
