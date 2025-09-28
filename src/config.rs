use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct HeaderFormatting {
    pub date_format: Option<String>,
    pub label_style: Option<String>,
    pub series_style: Option<String>,
    pub include_currency: Option<bool>,
    // Optional toggles that may be placed inside the [header_formatting]
    // table in older or alternate config layouts. We accept these here
    // and propagate them up into the top-level `Config` so either
    // placement works.
    pub include_design: Option<bool>,
    pub include_engraving: Option<bool>,
    pub include_watermark: Option<bool>,
    pub include_perforation: Option<bool>,
    pub include_catalog_numbers: Option<bool>,
    pub include_series: Option<bool>,
    pub include_country: Option<bool>,
    pub include_date: Option<bool>,
    pub print_flag_cover: Option<bool>,
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
    // Optional path to an offline calibration CSV file. CSV format:
    // filename,mm_per_px_w,mm_per_px_h
    pub calibration_csv: Option<String>,
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
    // If false, the generator will skip emitting the large flag/title cover page
    // Default: true (when None or not present)
    pub print_flag_cover: Option<bool>,
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
            calibration_csv: None,
            max_rows_per_page: None,
            print_flag_cover: None,
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
            // First parse into a generic toml::Value to inspect raw mapping
            if let Ok(value) = toml::from_str::<toml::Value>(&content) {
                // Debug: report whether top-level key `print_flag_cover` exists in the parsed value
                if value.get("print_flag_cover").is_some() {
                    println!("[DEBUG] toml::Value contains top-level 'print_flag_cover'");
                } else {
                    println!("[DEBUG] toml::Value does NOT contain top-level 'print_flag_cover'");
                }
            } else {
                println!("[DEBUG] Failed to parse into toml::Value (this shouldn't happen when parsing into Config succeeded)");
            }

            match toml::from_str::<Config>(&content) {
                Ok(cfg) => {
                    // If the file places the include_* toggles inside the
                    // `[header_formatting]` table (common), propagate those
                    // values into the top-level `Config` fields when the
                    // top-level ones are not present. This keeps both
                    // layouts working.
                    let mut cfg = cfg;
                    if let Some(hf) = cfg.header_formatting.clone() {
                        cfg.include_design = cfg.include_design.or(hf.include_design);
                        cfg.include_engraving = cfg.include_engraving.or(hf.include_engraving);
                        cfg.include_watermark = cfg.include_watermark.or(hf.include_watermark);
                        cfg.include_perforation = cfg.include_perforation.or(hf.include_perforation);
                        cfg.include_catalog_numbers = cfg.include_catalog_numbers.or(hf.include_catalog_numbers);
                        cfg.include_series = cfg.include_series.or(hf.include_series);
                        cfg.include_country = cfg.include_country.or(hf.include_country);
                        cfg.include_date = cfg.include_date.or(hf.include_date);
                        // Allow `print_flag_cover` to be placed inside `[header_formatting]`
                        // in older or mistaken configs; propagate it to the top-level field
                        // so either placement works.
                        cfg.print_flag_cover = cfg.print_flag_cover.or(hf.print_flag_cover);
                    }

                    println!("[INFO] Loaded config from {}", candidate.display());
                    // Debug: show raw content and parsed config to help diagnose missing fields
                    println!("[DEBUG] Raw config content from {}:\n{}", candidate.display(), &content);
                    println!("[DEBUG] Parsed Config struct: {:?}", cfg);
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

use std::collections::HashMap;

/// Load a simple mapping file that allows users to override site_name -> canonical_slug
/// Example TOML:
/// [slug_overrides]
/// "British-ColumbiaIsland" = "British-Columbia!Vancouver-Island"
pub fn load_slug_overrides(path: &str) -> HashMap<String, String> {
    // Search upward from cwd like load_config
    let try_paths = {
        let mut paths = Vec::new();
        if let Ok(mut dir) = std::env::current_dir() {
            for _ in 0..8 {
                paths.push(dir.join(path));
                if !dir.pop() { break; }
            }
        }
        paths.push(std::path::PathBuf::from(path));
        paths
    };

    for candidate in try_paths.iter() {
        if let Ok(content) = fs::read_to_string(candidate) {
            if let Ok(value) = toml::from_str::<toml::Value>(&content) {
                if let Some(table) = value.get("slug_overrides") {
                    if let Some(tbl) = table.as_table() {
                        let mut out = HashMap::new();
                        for (k, v) in tbl.iter() {
                            if let Some(s) = v.as_str() {
                                out.insert(k.clone(), s.to_string());
                            }
                        }
                        println!("[INFO] Loaded slug_overrides from {} ({} entries)", candidate.display(), out.len());
                        return out;
                    }
                }
            }
        }
    }
    HashMap::new()
}
