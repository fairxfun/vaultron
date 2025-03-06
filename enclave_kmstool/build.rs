extern crate bindgen;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("./src/c_lib/kmstool_enclave_lib.h")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(improper_ctypes)]")
        .raw_line("#![allow(warnings)]")
        .raw_line("")
        .raw_line("#[link(name = \"kmstool-enclave-lib\")]")
        .raw_line("unsafe extern \"C\" {}")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./src/c_lib");
    bindings
        .write_to_file(out_path.join("kmstool_enclave_lib.rs"))
        .expect("Couldn't write bindings!");
}
