use std::collections::HashMap;
use walkdir::WalkDir;
use std::fs;

pub fn analyze_languages(path: &str) -> HashMap<String, f64> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut total = 0usize;

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .filter(|e| {
            !e.path().to_string_lossy().contains("/.git/")
                && !e.path().to_string_lossy().contains("/target/")
                && !e.path().to_string_lossy().contains("/build/")
                && !e.path().to_string_lossy().contains("/dist/")
                && !e.path().to_string_lossy().contains("/.idea/")
                && !e.path().to_string_lossy().contains("/node_modules/")
        })
    {
        if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
            let lang = match ext {
                "rs" => "Rust",
                "c" => "C",
                "cpp" | "cc" | "cxx" => "C++",
                "py" => "Python",
                "js" => "JavaScript",
                "ts" => "TypeScript",
                "kt" | "kts" => "Kotlin",
                "java" => "Java",
                "html" => "HTML",
                "css" => "CSS",
                "toml" => "TOML",
                "json" => "JSON",
                "md" => "Markdown",
                "sh" => "Shell",
                "wave" => "Wave",
                _ => "Other",
            };

            let size = fs::metadata(entry.path()).map(|m| m.len()).unwrap_or(0);
            *counts.entry(lang.to_string()).or_insert(0) += size as usize;
            total += size as usize;
        }
    }

    let mut result = HashMap::new();
    for (lang, size) in counts {
        if total > 0 {
            result.insert(lang, (size as f64 / total as f64) * 100.0);
        }
    }

    result
}
