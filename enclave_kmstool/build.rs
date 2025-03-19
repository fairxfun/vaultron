extern crate bindgen;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .clang_arg("--target=x86_64-unknown-linux-gnu")
        .derive_default(true)
        .derive_eq(true)
        .size_t_is_usize(true)
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .layout_tests(false)
        .use_core()
        .sort_semantically(true)
        .rustfmt_configuration_file(None)
        .header("./lib/aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib/kmstool_enclave_lib.h")
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

    let out_path = PathBuf::from("./src/gen");
    bindings
        .write_to_file(out_path.join("kmstool_enclave_lib_gen.rs"))
        .expect("Couldn't write bindings!");
}
