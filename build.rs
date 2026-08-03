use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/plugin.c")
        .include("includes/")
        .std("c23")
        .compile("plugin");

    let bindings = bindgen::Builder::default()
        .header("includes/plugin.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-std=c23")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!("cargo::rerun-if-changed=src/plugin.c");
}
