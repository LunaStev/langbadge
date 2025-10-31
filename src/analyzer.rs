use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;
use rayon::prelude::ParallelBridge;
use walkdir::WalkDir;
use serde::Deserialize;

use rayon::prelude::*;
use std::path::Path;

use crate::color::get_color;

#[derive(Deserialize)]
struct LangConfig {
    #[serde(default)]
    languages: HashMap<String, String>,
    #[serde(default)]
    file_names: HashMap<String, String>,
    #[serde(default)]
    header_rules: HashMap<String, Vec<String>>,
    #[serde(default)]
    exclude_exts: Vec<String>,
    #[serde(default)]
    exclude_langs: Vec<String>,
    #[serde(default)]
    threshold: Option<f64>
}

static LANG_CONFIG: OnceLock<LangConfig> = OnceLock::new();

pub fn analyze_languages_fast(path: &str) -> HashMap<String, f64> {
    let config = get_config();

    let cache_dir = dirs::cache_dir().unwrap_or_else(|| Path::new(".").to_path_buf()).join("langbadge");
    let _ = fs::create_dir_all(&cache_dir);

    let hash = format!("{:x}", md5::compute(path.as_bytes()));
    let cache_file = cache_dir.join(format!("{}.json", hash));
    if let Ok(data) = fs::read_to_string(&cache_file) {
        if let Ok(result) = serde_json::from_str::<HashMap<String, f64>>(&data) {
            println!("⚡ Using cached result");
            return result;
        }
    }

    let entries: Vec<_> = WalkDir::new(path)
        .max_depth(15)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .collect();

    let counts: HashMap<String, usize> = entries
        .par_iter()
        .filter_map(|entry| {
            let path_str = entry.path().to_string_lossy();
            if path_str.contains("/.git/") || path_str.contains("/node_modules/") || path_str.contains("/target/") {
                return None;
            }

            let ext = entry.path().extension().and_then(|s| s.to_str())?;
            if config.exclude_exts.contains(&ext.to_string()) {
                return None;
            }

            let lang = get_lang_from_ext(ext, entry.path().to_str().unwrap_or(""))?;
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            Some((lang, size as usize))
        })
        .fold(HashMap::new, |mut acc, (lang, size)| {
            *acc.entry(lang).or_insert(0) += size;
            acc
        })
        .reduce(HashMap::new, |mut a, b| {
            for (k, v) in b {
                *a.entry(k).or_insert(0) += v;
            }
            a
        });

    let total: usize = counts.values().sum();
    let result: HashMap<String, f64> = counts
        .into_iter()
        .map(|(k, v)| (k, (v as f64 / total as f64) * 100.0))
        .collect();

    let _ = fs::write(&cache_file, serde_json::to_string(&result).unwrap_or_default());

    result
}

fn load_lang_config() -> LangConfig {
    let path = std::env::current_dir().unwrap_or_default().join("data/lang.json");
    let data = fs::read_to_string(&path).unwrap_or_else(|_| {
        eprintln!(
            "⚠️ Warning: Could not read {}, using default configuration",
            path.display()
        );
        "{}".to_string()
    });

    serde_json::from_str::<LangConfig>(&data).unwrap_or_else(|_| LangConfig {
        languages: HashMap::new(),
        file_names: HashMap::new(),
        header_rules: HashMap::new(),
        exclude_exts: vec![
            "txt".to_string(),
            "json".to_string(),
            "toml".to_string(),
            "lock".to_string(),
            "yml".to_string(),
            "yaml".to_string(),
            "log".to_string(),
            "md".to_string(),
        ],
        exclude_langs: vec![
            "Markdown".to_string(),
            "HTML".to_string(),
            "JSON".to_string(),
            "TOML".to_string(),
            "XML".to_string(),
            "YAML".to_string(),
        ],
        threshold: Some(1.0),
    })

}

fn get_config() -> &'static LangConfig {
    LANG_CONFIG.get_or_init(load_lang_config)
}

fn detect_header_lang(path: &str) -> Option<String> {
    let c_found = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .any(|e| e.path().extension().and_then(|s| s.to_str()) == Some("c"));

    let cpp_found = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .any(|e| {
            matches!(
                e.path().extension().and_then(|s| s.to_str()),
                Some("cpp") | Some("cc") | Some("cxx") | Some("hpp")
            )
        });

    if cpp_found {
        Some("C++".to_string())
    } else if c_found {
        Some("C".to_string())
    } else {
        None
    }
}

fn get_lang_from_ext(ext: &str, path: &str) -> Option<String> {
    let config = get_config();

    if config.exclude_exts.contains(&ext.to_string()) {
        return None;
    }

    if ext == "h" || ext == "hpp" {
        if let Some(lang) = detect_header_lang(path) {
            return Some(lang);
        }
    }

    config
        .languages
        .get(ext)
        .cloned()
        .or_else(|| Some("Other".to_string()))
}

fn get_lang_from_file_name(file_name: &str) -> Option<String> {
    let config = get_config();
    let lower_name = file_name.to_lowercase();

    for (key, value) in &config.file_names {
        if key.to_lowercase() == lower_name {
            return Some(value.clone());
        }
    }

    None
}


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
        let file_name = entry.file_name().to_string_lossy().to_string();

        if entry.path().extension().is_none() {
            if let Some(lang) = get_lang_from_file_name(&file_name) {
                let size = fs::metadata(entry.path()).map(|m| m.len()).unwrap_or(0);
                *counts.entry(lang).or_insert(0) += size as usize;
                total += size as usize;
                continue;
            }
        }

        if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
            if let Some(lang) = get_lang_from_ext(ext, path) {
                if get_config().exclude_langs.contains(&lang) {
                    continue; // 제외 언어
                }

                let size = fs::metadata(entry.path()).map(|m| m.len()).unwrap_or(0);
                *counts.entry(lang).or_insert(0) += size as usize;
                total += size as usize;
            }
        }
    }

    let mut result = HashMap::new();
    if total == 0 {
        return result;
    }

    for (lang, size) in counts {
        let percent = (size as f64 / total as f64) * 100.0;
        result.insert(lang, percent);
    }

    let threshold = get_config().threshold.unwrap_or(1.0);
    let mut others_total = 0.0;
    let mut filtered = HashMap::new();

    for (lang, percent) in result.iter() {
        if *percent < threshold {
            others_total += percent;
        } else {
            filtered.insert(lang.clone(), *percent);
        }
    }

    if others_total > 0.0 {
        filtered.insert("Other".to_string(), others_total);
    }

    filtered
}
