use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct HeaderFormatting {
    pub date_format: Option<String>,
    pub label_style: Option<String>,
    pub series_style: Option<String>,
    pub include_currency: Option<bool>,
    pub header_spacing: Option<f32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub reference_width_mm: Option<f32>,
    pub reference_height_mm: Option<f32>,
    pub stamp_box_height: Option<f32>,
    pub stamp_box_adjust: Option<f32>,
    pub median_width_cap_3: Option<f32>,
    pub median_width_cap_4: Option<f32>,
    pub row_font_spacing: Option<f32>,
    pub global_scale: Option<f32>,
    pub header_font_spacing: Option<f32>,
    pub header_formatting: Option<HeaderFormatting>,
    // Optional toggles to include certain header fields when true
    pub include_design: Option<bool>,
    pub include_engraving: Option<bool>,
    pub include_watermark: Option<bool>,
    pub include_perforation: Option<bool>,
    pub include_catalog_numbers: Option<bool>,
    pub include_series: Option<bool>,
    pub include_country: Option<bool>,
    pub include_date: Option<bool>,
    // Allow overriding maximum rows per page when header is larger
    pub max_rows_per_page: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            reference_width_mm: None,
            reference_height_mm: None,
            stamp_box_height: None,
            stamp_box_adjust: None,
            median_width_cap_3: None,
            median_width_cap_4: None,
            global_scale: None,
            row_font_spacing: None,
            header_font_spacing: None,
            header_formatting: None,
            include_design: None,
            include_engraving: None,
            include_watermark: None,
            include_perforation: None,
            include_catalog_numbers: None,
            include_series: None,
            include_country: None,
            include_date: None,
            max_rows_per_page: None,
        }
    }
}

pub fn load_config(path: &str) -> Config {
    // Try to find the config file starting at the current working directory
    // and searching upward through parent directories. This ensures that
    // running the binary from a subdirectory (e.g. Albums/Canada) will still
    // pick up the project's top-level `config.toml`.
    let try_paths = {
        let mut paths = Vec::new();
        if let Ok(mut dir) = std::env::current_dir() {
            for _ in 0..8 {
                paths.push(dir.join(path));
                if !dir.pop() { break; }
            }
        }
        // Also try the literal path as a last resort
        paths.push(std::path::PathBuf::from(path));
        paths
    };

    for candidate in try_paths.iter() {
        if let Ok(content) = fs::read_to_string(candidate) {
            match toml::from_str::<Config>(&content) {
                Ok(cfg) => {
                    println!("[INFO] Loaded config from {}", candidate.display());
                    return cfg;
                }
                Err(e) => {
                    println!("[WARN] Failed to parse {}: {} -- using defaults", candidate.display(), e);
                    break; // If parsing fails for a found file, don't keep searching
                }
            }
        }
    }

    println!("[INFO] config.toml not found in parent dirs or failed to parse, using default values");
    Config::default()
}
