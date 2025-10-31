use crate::color::get_color;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn render_svg(langs: &HashMap<String, f64>, output: &str, style: &str) -> std::io::Result<()> {
    match style {
        "github" => render_github(langs, output),
        "badge" => render_badge(langs, output),
        "pie" => render_pie(langs, output),
        _ => render_github(langs, output),
    }
}

fn render_github(langs: &HashMap<String, f64>, output: &str) -> std::io::Result<()> {
    let total_width = 440.0;
    let height = 12.0;
    let mut svg = String::new();

    svg.push_str(&format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{:.0}' height='{:.0}'><rect rx='6' ry='6' width='{:.0}' height='{:.0}' fill='#e1e4e8'/>",
        total_width, height, total_width, height
    ));

    let mut x_offset = 0.0;
    for (lang, percent) in langs {
        let width = total_width * (*percent / 100.0);
        let color = get_color(lang);
        svg.push_str(&format!(
            "<rect x='{:.1}' y='0' width='{:.1}' height='{:.1}' fill='{}'/>",
            x_offset, width, height, color
        ));
        x_offset += width;
    }

    svg.push_str("</svg>");
    let mut file = File::create(output)?;
    file.write_all(svg.as_bytes())?;
    Ok(())
}

fn render_badge(langs: &HashMap<String, f64>, output: &str) -> std::io::Result<()> {
    let (top_lang, percent) = langs.iter().max_by(|a, b| a.1.total_cmp(b.1)).unwrap();
    let color = get_color(top_lang);

    let svg = format!(
        "<svg xmlns='http://www.w3.org/2000/svg' height='20' width='150'>
            <rect width='150' height='20' fill='#555'/>
            <rect x='60' width='90' height='20' fill='{color}'/>
            <text x='5' y='14' fill='white' font-size='11'>{}</text>
            <text x='65' y='14' fill='white' font-size='11'>{:.1}%</text>
        </svg>",
        top_lang, percent
    );

    let mut file = File::create(output)?;
    file.write_all(svg.as_bytes())?;
    Ok(())
}

fn render_pie(_: &HashMap<String, f64>, output: &str) -> std::io::Result<()> {
    let mut file = File::create(output)?;
    file.write_all(b"<svg><text x='10' y='20'>Pie chart style not implemented yet</text></svg>")?;
    Ok(())
}
