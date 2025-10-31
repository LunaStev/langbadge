use std::process::Command;
use std::path::Path;
use std::fs;

pub fn clone_repo(url: &str) -> Option<String> {
    let temp_dir = "temp_repo";

    if Path::new(temp_dir).exists() {
        let _ = fs::remove_dir_all(temp_dir);
    }
    fs::create_dir_all(temp_dir).ok()?;

    println!("Cloning repo: {}", url);
    let status = Command::new("git")
        .args(["clone", "--depth", "1", url, temp_dir])
        .status()
        .expect("Failed to run git command");

    if status.success() {
        Some(temp_dir.to_string())
    } else {
        None
    }
}
