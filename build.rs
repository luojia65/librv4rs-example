use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    fs::copy(
        "bin/librv4rs.a",
        out_dir.join("librv4rs.a"),
    ).unwrap();

    println!("cargo:rustc-link-lib=static=rv4rs");
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=build.rs");
}
