// Project Name:  MinmusOS
// File Name:     main.rs
// File Function: The build script of boot
// Author:        Jishen Lin
// License:       MIT License

fn main() {
    let local_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rustc-link-arg-bins=--script={}", local_path.join("linker.ld").display());
}