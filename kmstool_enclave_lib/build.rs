extern crate bindgen;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("./lib/kmstool_enclave_lib.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("kmstool_enclave_lib.rs"))
        .expect("Couldn't write bindings!");
}
