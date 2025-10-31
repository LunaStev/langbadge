use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;
use serde::Deserialize;

#[derive(Deserialize)]
struct ColorMap(HashMap<String, String>);

static COLOR_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

fn load_colors() -> HashMap<String, String> {
    let path = std::env::current_dir()
        .unwrap_or_default()
        .join("data/colors.json");

    let data = fs::read_to_string(&path)
        .unwrap_or_else(|_| {
            eprintln!("Warning: Could not read {}, using default colors", path.display());
            "{}".to_string()
        });

    serde_json::from_str::<HashMap<String, String>>(&data)
        .unwrap_or_else(|_| HashMap::new())
}

pub fn get_color(lang: &str) -> String {
    let map = COLOR_MAP.get_or_init(load_colors);
    map.get(lang).cloned().unwrap_or_else(|| "#cccccc".to_string())
}
