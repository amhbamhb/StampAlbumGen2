use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub reference_width_mm: Option<f32>,
    pub reference_height_mm: Option<f32>,
    // Add more config fields as needed
}

impl Default for Config {
    fn default() -> Self {
        Config {
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
                    reference_width_mm: None,
                    reference_height_mm: None,
                }
            })
        }
        Err(_) => {
            println!("[INFO] config.toml not found, using default values");
            Config {
                reference_width_mm: None,
                reference_height_mm: None,
            }
        }
    }
}
