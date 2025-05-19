use std::process::Command;

fn main() {
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
        _ => println!("Failed to get git hash"),
    }
    println!("cargo:rerun-if-changed=.git/HEAD");
}
