use std::{env, process::Command};

fn main() {
    // Get git tag version
    let tags = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        });
    match tags {
        Some(v) => {
            println!("cargo:rustc-env=VAULTRON_VERSION={}", v);
        }
        None => {
            if env::var("VAULTRON_VERSION").is_err() {
                let version = format!("v{}", env!("CARGO_PKG_VERSION"));
                println!("cargo:rustc-env=VAULTRON_VERSION={}", version);
            }
        }
    }

    // Get git hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().chars().take(7).collect::<String>())
            } else {
                None
            }
        });
    match git_hash {
        Some(hash) => println!("cargo:rustc-env=VAULTRON_GIT_REVISION={}", hash),
        None => println!("Failed to get git hash"),
    }
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/tags");
}
