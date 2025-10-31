mod analyzer;
mod renderer;
mod color;
mod git;

use std::env;
use std::fs;
use analyzer::{analyze_languages, analyze_languages_fast};
use renderer::render_all;
use git::clone_repo;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: langbadge <path_or_repo> [--style github|badge|github-ui] [--fast]");
        return;
    }

    let target = &args[1];
    let is_fast = args.contains(&"--fast".to_string());

    let style = args.iter().find(|a| a.starts_with("--style="))
        .map(|s| s.split('=').nth(1).unwrap_or("github"))
        .unwrap_or("github");

    let is_git_repo = target.starts_with("http://") || target.starts_with("https://");
    let path = if is_git_repo {
        clone_repo(target).unwrap_or_else(|| ".".to_string())
    } else {
        target.clone()
    };

    println!("Analyzing: {}{}", path, if is_fast { " (fast mode)" } else { "" });

    let result = if is_fast {
        analyze_languages_fast(&path)
    } else {
        analyze_languages(&path)
    };

    println!("🖼Rendering...");
    render_all(&result).expect("Failed to render output");
    println!("Done! Results saved as langbadge.svg / langbadge.html");

    if is_git_repo && path == "temp_repo" {
        let _ = fs::remove_dir_all(&path);
    }
}
