use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=link.x");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_DEBUG_STUB");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let debug_stub = env::var_os("CARGO_FEATURE_DEBUG_STUB").is_some();

    let memory_x = if debug_stub {
        r#"MEMORY
{
  RP1_APP_SRAM (rwx)   : ORIGIN = 0x20000000, LENGTH = 63K
  RP1_DEBUG_STUB (rwx) : ORIGIN = 0x2000fc00, LENGTH = 1K
}

_stack_start = ORIGIN(RP1_DEBUG_STUB);
__rp1_debug_stub_start = ORIGIN(RP1_DEBUG_STUB);
__rp1_debug_stub_end = ORIGIN(RP1_DEBUG_STUB) + LENGTH(RP1_DEBUG_STUB);
__rp1_debug_mailbox = ORIGIN(RP1_DEBUG_STUB);
"#
    } else {
        r#"MEMORY
{
  RP1_APP_SRAM (rwx)   : ORIGIN = 0x20000000, LENGTH = 64K
  RP1_DEBUG_STUB (rwx) : ORIGIN = 0x20010000, LENGTH = 0
}

_stack_start = ORIGIN(RP1_APP_SRAM) + LENGTH(RP1_APP_SRAM);
__rp1_debug_stub_start = ORIGIN(RP1_DEBUG_STUB);
__rp1_debug_stub_end = ORIGIN(RP1_DEBUG_STUB);
__rp1_debug_mailbox = ORIGIN(RP1_DEBUG_STUB);
"#
    };

    fs::write(out_dir.join("rp1-memory.x"), memory_x).unwrap();

    println!("cargo:rustc-link-search={}", manifest_dir);
    println!("cargo:rustc-link-search={}", out_dir.display());
}
