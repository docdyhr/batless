use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Re-run if git HEAD or refs change so metadata stays current
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads");

    // Try to capture current short git commit hash
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            if let Ok(hash) = String::from_utf8(output.stdout) {
                let trimmed = hash.trim();
                if !trimmed.is_empty() {
                    println!("cargo:rustc-env=BATLESS_GIT_HASH={trimmed}");
                }
            }
        }
    }

    // Emit build timestamp (unix epoch seconds)
    if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
        println!(
            "cargo:rustc-env=BATLESS_BUILD_TIMESTAMP={}",
            duration.as_secs()
        );
    }
}
