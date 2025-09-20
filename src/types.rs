//! # Global State Module
//!
//! This module contains global state variables and enums used throughout
//! the StampAlbumGen2 application.

use std::sync::Mutex;

/// Global album file name
pub static ALBUM_FILE_NAME: Mutex<String> = Mutex::new(String::new());

/// Global album database path
pub static ALBUM_DB_PATH: Mutex<String> = Mutex::new(String::new());

/// File separator for the current OS
#[cfg(target_os = "windows")]
pub static FILE_SEPARATOR: &str = "\\";
#[cfg(not(target_os = "windows"))]
pub static FILE_SEPARATOR: &str = "/";

/// Struct representing series information
#[derive(Debug, Clone, PartialEq)]
pub struct SeriesInfo {
    pub count: i32,
    pub multi_perf: bool,
    pub width: String,
    pub height: String,
}

impl SeriesInfo {
    pub fn new(count: i32, multi_perf: bool, width: &str, height: &str) -> Self {
        Self {
            count,
            multi_perf,
            width: width.to_string(),
            height: height.to_string(),
        }
    }
}

/// Struct representing image status
#[derive(Debug, Clone, PartialEq)]
pub struct ImageStatus {
    pub link: String,
    pub exists: bool,
    pub width: i32,
    pub height: i32,
    pub filename: String,
}

impl ImageStatus {
    pub fn new(link: &str, exists: bool, width: i32, height: i32, filename: &str) -> Self {
        Self {
            link: link.to_string(),
            exists,
            width,
            height,
            filename: filename.to_string(),
        }
    }
}

/// Operating system type enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum OSType {
    Windows,
    MacOS,
    Linux,
    Unknown,
}

impl Default for OSType {
    fn default() -> Self {
        OSType::Unknown
    }
}

/// Gets the current operating system type
pub fn get_os_type() -> OSType {
    match std::env::consts::OS {
        "windows" => OSType::Windows,
        "macos" => OSType::MacOS,
        "linux" => OSType::Linux,
        _ => OSType::Unknown,
    }
}

/// Struct representing page header information for AlbumEasy output
#[derive(Debug, Clone, PartialEq)]
pub struct PageHeaderInfo {
    pub year: i32,
    pub month: String,
    pub day: String,
    pub series: String,
    pub perforation: String,
    pub design: String,
    pub engraving: String,
    pub watermark: String,
    pub currency: String,
}

impl Default for PageHeaderInfo {
    fn default() -> Self {
        Self {
            year: 0,
            month: String::new(),
            day: String::new(),
            series: String::new(),
            perforation: "14½".to_string(),
            design: String::new(),
            engraving: String::new(),
            watermark: String::new(),
            currency: String::new(),
        }
    }
}