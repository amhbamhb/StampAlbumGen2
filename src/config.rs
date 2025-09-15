use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub stamp_box_height: Option<f32>,
    pub row_font_spacing: Option<f32>,
    pub header_font_spacing: Option<f32>,
    pub reference_width_mm: Option<f32>,
    pub reference_height_mm: Option<f32>,
    // Add more config fields as needed
}

impl Default for Config {
    fn default() -> Self {
        Config {
            stamp_box_height: None,
            row_font_spacing: None,
            header_font_spacing: None,
            reference_width_mm: None,
            reference_height_mm: None,
        }
    }
}

pub fn load_config(path: &str) -> Config {
    match fs::read_to_string(path) {
        Ok(content) => {
            toml::from_str(&content).unwrap_or_else(|_| {
                println!("[WARN] Failed to parse config.toml, using defaults");
                Config {
                    stamp_box_height: None,
                    row_font_spacing: None,
                    header_font_spacing: None,
                    reference_width_mm: None,
                    reference_height_mm: None,
                }
            })
        }
        Err(_) => {
            println!("[INFO] config.toml not found, using default values");
            Config {
                stamp_box_height: None,
                row_font_spacing: None,
                header_font_spacing: None,
                reference_width_mm: None,
                reference_height_mm: None,
            }
        }
    }
}
