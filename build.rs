// Встраивает короткий хеш коммита в бинарник (RULES.md §6).
// Приоритет: env GIT_HASH (Docker/CI) → `git rev-parse` → "dev".
use std::process::Command;

fn main() {
    let hash = std::env::var("GIT_HASH")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| {
            Command::new("git")
                .args(["rev-parse", "--short=7", "HEAD"])
                .output()
                .ok()
                .filter(|out| out.status.success())
                .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_owned())
                .filter(|s| !s.is_empty())
        })
        .map(|h| h.chars().take(7).collect::<String>())
        .unwrap_or_else(|| "dev".to_owned());

    println!("cargo:rustc-env=GIT_HASH={hash}");
    println!("cargo:rerun-if-env-changed=GIT_HASH");
}
