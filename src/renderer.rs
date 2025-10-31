use crate::color::get_color;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn sort_langs(langs: &HashMap<String, f64>) -> Vec<(&String, &f64)> {
    let mut sorted: Vec<_> = langs.iter().collect();
    sorted.sort_by(|a, b| {
        if a.0 == "Other" && b.0 != "Other" {
            std::cmp::Ordering::Greater
        } else if b.0 == "Other" && a.0 != "Other" {
            std::cmp::Ordering::Less
        } else {
            b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal)
        }
    });
    sorted
}

pub fn render_all(langs: &HashMap<String, f64>) -> std::io::Result<()> {
    render_html_ui(langs, "langbadge.html")?;
    render_svg_ui(langs, "langbadge.svg")?;
    Ok(())
}

fn render_html_ui(langs: &HashMap<String, f64>, output: &str) -> std::io::Result<()> {
    let sorted = sort_langs(langs);
    let mut html = String::new();

    html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>Languages</title>
<style>
body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
  background-color: #0d1117;
  color: #c9d1d9;
  margin: 0;
  padding: 20px;
}

.lang-bar {
  display: flex;
  height: 8px;
  border-radius: 6px;
  overflow: hidden;
  background-color: #21262d;
  width: 100%;
  max-width: 320px;
  margin-bottom: 8px;
}

.lang {
  height: 100%;
}

.lang-line {
  display: flex;
  flex-wrap: wrap;
  max-width: 320px;
  gap: 8px 16px;
  font-size: 12px;
  color: #c9d1d9;
}

.lang-item {
  display: flex;
  align-items: center;
  gap: 6px;
  line-height: 1.6;
}

.lang-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.lang-percent {
  color: #8b949e;
}
</style>
</head>
<body>
"#);

    html.push_str("<div class='lang-bar'>\n");
    for (lang, percent) in &sorted {
        let color = get_color(lang);
        html.push_str(&format!(
            "  <div class='lang' style='width:{:.1}%; background:{}'></div>\n",
            percent, color
        ));
    }
    html.push_str("</div>\n");

    html.push_str("<div class='lang-line'>\n");
    for (lang, percent) in &sorted {
        let color = get_color(lang);
        html.push_str(&format!(
            "  <div class='lang-item'><span class='lang-dot' style='background:{}'></span>{} <span class='lang-percent'>{:.1}%</span></div>\n",
            color, lang, percent
        ));
    }
    html.push_str("</div>\n</body>\n</html>");

    let mut file = File::create(output)?;
    file.write_all(html.as_bytes())?;
    Ok(())
}

fn render_svg_ui(langs: &HashMap<String, f64>, output: &str) -> std::io::Result<()> {
    let sorted = sort_langs(langs);
    let width = 320.0;
    let bar_height = 8.0;
    let mut svg_height = 28.0;
    let line_height = 18.0;
    let per_row = 3;

    let total_rows = (sorted.len() as f64 / per_row as f64).ceil() as u32;
    svg_height += total_rows as f64 * line_height + 12.0;

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns='http://www.w3.org/2000/svg' width='{:.0}' height='{:.0}' viewBox='0 0 {:.0} {:.0}'>",
        width, svg_height, width, svg_height
    ));

    svg.push_str(
        r#"<style>
.text { font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif;
        font-size:12px; fill:#c9d1d9; }
.percent { fill:#8b949e; font-size:12px; }
</style>"#,
    );

    svg.push_str(&format!(
        "<rect x='0' y='0' width='{:.0}' height='{:.1}' rx='4' ry='4' fill='#21262d' />",
        width, bar_height
    ));

    let mut x_offset = 0.0;
    for (lang, percent) in &sorted {
        let color = get_color(lang);
        let bar_width = width * (**percent / 100.0);
        svg.push_str(&format!(
            "<rect x='{:.1}' y='0' width='{:.1}' height='{:.1}' fill='{}' />",
            x_offset, bar_width, bar_height, color
        ));
        x_offset += bar_width;
    }

    let mut y_offset = 22.0;
    let mut x_col = 0.0;
    let col_width = width / per_row as f64;

    for (i, (lang, percent)) in sorted.iter().enumerate() {
        let color = get_color(lang);
        if i % per_row == 0 && i > 0 {
            y_offset += line_height;
            x_col = 0.0;
        }
        let dot_y = y_offset - 5.0;
        svg.push_str(&format!(
            "<circle cx='{:.1}' cy='{:.1}' r='4' fill='{}' />",
            x_col + 6.0, dot_y, color
        ));
        svg.push_str(&format!(
            "<text class='text' x='{:.1}' y='{:.1}'>{}</text>",
            x_col + 16.0, y_offset, lang
        ));
        svg.push_str(&format!(
            "<text class='percent' x='{:.1}' y='{:.1}'>{:.1}%</text>",
            x_col + 16.0 + (lang.len() as f64 * 6.0), y_offset, percent
        ));
        x_col += col_width;
    }

    svg.push_str("</svg>");
    std::fs::write(output, svg)?;
    Ok(())
}
