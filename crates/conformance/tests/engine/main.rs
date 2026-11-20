//! Conformance suite for `verdant-core`, `verdant-wasm`, `verdant-cli`, and the Verdant Engine verifier (`VE-01`..`VE-61`).

use std::path::PathBuf;

pub(crate) fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below root")
        .to_path_buf()
}

/// Serializes C-ABI singleton tests calling `verdant_wasm::verdant_wasm_*` across parallel test threads.
pub(crate) fn wasm_abi_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

mod core;
mod governance;
mod world;
