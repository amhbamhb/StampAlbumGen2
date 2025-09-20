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
    pub row_font_spacing: Option<f32>,
    pub header_font_spacing: Option<f32>,
    pub header_formatting: Option<HeaderFormatting>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            reference_width_mm: None,
            reference_height_mm: None,
            stamp_box_height: None,
            row_font_spacing: None,
            header_font_spacing: None,
            header_formatting: None,
        }
    }
}

pub fn load_config(path: &str) -> Config {
    match fs::read_to_string(path) {
        Ok(content) => {
            toml::from_str(&content).unwrap_or_else(|_| {
                println!("[WARN] Failed to parse config.toml, using defaults");
                Config {
                    reference_width_mm: None,
                    reference_height_mm: None,
                    stamp_box_height: None,
                    row_font_spacing: None,
                    header_font_spacing: None,
                    header_formatting: None,
                }
            })
        }
        Err(_) => {
            println!("[INFO] config.toml not found, using default values");
            Config {
                reference_width_mm: None,
                reference_height_mm: None,
                stamp_box_height: None,
                row_font_spacing: None,
                header_font_spacing: None,
                header_formatting: None,
            }
        }
    }
}
