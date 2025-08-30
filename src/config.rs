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

pub fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path).expect("Failed to read config.toml");
    toml::from_str(&content).expect("Failed to parse config.toml")
}
