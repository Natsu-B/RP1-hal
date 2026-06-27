fn main() {
    println!("cargo:rerun-if-changed=link.x");
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}
