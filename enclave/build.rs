use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git command");
    let git_hash = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 sequence")
        .trim()
        .to_string();
    println!("cargo:rustc-env=ENCLAVE_GIT_REVISION={}", git_hash);
    println!("cargo:rerun-if-changed=.git/HEAD");
}
