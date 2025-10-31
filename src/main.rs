mod analyzer;
mod renderer;
mod color;
mod git;

use std::env;
use std::fs;
use analyzer::analyze_languages;
use renderer::render_svg;
use git::clone_repo;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: langbadge <path_or_repo> [output.svg] [--style github]");
        return;
    }

    let target = &args[1];
    let default_output = "langbadge.svg".to_string();
    let default_style = "--style=github".to_string();

    let output = args.get(2).unwrap_or(&default_output);
    let style_arg = args.iter().find(|a| a.starts_with("--style=")).unwrap_or(&default_style);
    let style_name = style_arg.split('=').nth(1).unwrap_or("github");

    let is_git_repo = target.starts_with("http://") || target.starts_with("https://");

    let path = if is_git_repo {
        match clone_repo(target) {
            Some(repo_path) => repo_path,
            None => {
                eprintln!("❌ Failed to clone repository!");
                return;
            }
        }
    } else {
        target.clone()
    };

    println!("🔍 Analyzing: {}", path);
    let result = analyze_languages(&path);

    println!("🖼️ Rendering style: {}", style_name);
    render_svg(&result, output, style_name).expect("Failed to render SVG");

    // ✅ 안전한 삭제 로직
    if is_git_repo {
        if path == "temp_repo" {
            if let Err(e) = fs::remove_dir_all(&path) {
                eprintln!("⚠️ Failed to remove temp repo: {}", e);
            } else {
                println!("🧹 Removed temporary repository folder: {}", path);
            }
        } else {
            println!("🛑 Skipped deletion (path != temp_repo): {}", path);
        }
    }

    println!("✅ Done! Output: {}", output);
}
