// Project Name:  MinmusOS
// File Name:     build.rs
// File Function: The build script of bootloader
// Author:        Jishen Lin
// License:       MIT License

fn main() {
    let local_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rustc-link-arg-bins=--script={}", local_path.join("linker.ld").display());
}