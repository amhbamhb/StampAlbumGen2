//! Core CommandsFileGenerator struct and main functionality.
//!
//! This module contains the main CommandsFileGenerator struct and its
//! core methods for generating stamp album command files.

use crate::album_globals::Stamp;
use crate::commands_file_generator::album_creation::save_stamp_data;
use crate::commands_file_generator::utils::{
    is_selected_emission, is_selected_format, month_number, snooze,
};
use crate::config::Config;
use crate::types::SeriesInfo;
use log::info;
use std::collections::HashMap;

/// Type alias for country (string)
pub type Country = String;

/// Parameters for `CommandsFileGenerator::collect_stampworld_data` to reduce
/// the number of direct arguments and make calls more readable.
pub struct CollectStampworldCallParams<'a> {
    pub base_url: &'a str,
    pub overwrite_images: bool,
    pub page_callback: Option<&'a dyn Fn(usize)>,
    pub max_pages: i32,
    pub stop_after_no_hits: i32,
    pub start_year: i32,
    pub end_year: i32,
}

/// Main struct for generating stamp album command files
pub struct CommandsFileGenerator {
    pub album_name: String,
    pub country: Country,
    pub year: String,
    pub month: String,
    pub day: String,
    pub series: String,
    pub catalog_name: String,
    pub formats: Vec<String>,
    pub emissions: Vec<String>,
    pub separator: String,
    pub fix_sizes: bool,
    pub overwrite_album: bool,
    pub data_source: String,
    pub stampworld_url: String,
    pub colnect_url: String,
    pub touchstamps_url: String,
    pub image_path: String,
    pub album_path: String,
    pub extract_path: String,
    pub config: Config,
    pub stamps: Vec<Stamp>,
    pub stamp_data: Vec<String>,
    pub series_info: HashMap<String, SeriesInfo>,
    pub image_urls: HashMap<String, String>,
    pub image_paths: HashMap<String, String>,
    pub catalog_nums: HashMap<String, String>,
    pub stampworld_data: Vec<String>,
    pub colnect_data: Vec<String>,
    pub touchstamps_data: Vec<String>,
}

impl CommandsFileGenerator {
    /// Create a new CommandsFileGenerator instance
    pub fn new(
        album_name: String,
        country: Country,
        year: String,
        month: String,
        day: String,
        series: String,
        catalog_name: String,
        formats: Vec<String>,
        emissions: Vec<String>,
        separator: String,
        fix_sizes: bool,
        overwrite_album: bool,
        data_source: String,
        stampworld_url: String,
        colnect_url: String,
        touchstamps_url: String,
        image_path: String,
        album_path: String,
        extract_path: String,
        config: Config,
    ) -> Self {
        Self {
            album_name,
            country,
            year,
            month,
            day,
            series,
            catalog_name,
            formats,
            emissions,
            separator,
            fix_sizes,
            overwrite_album,
            data_source,
            stampworld_url,
            colnect_url,
            touchstamps_url,
            image_path,
            album_path,
            extract_path,
            config,
            stamps: Vec::new(),
            stamp_data: Vec::new(),
            series_info: HashMap::new(),
            image_urls: HashMap::new(),
            image_paths: HashMap::new(),
            catalog_nums: HashMap::new(),
            stampworld_data: Vec::new(),
            colnect_data: Vec::new(),
            touchstamps_data: Vec::new(),
        }
    }

    /// Get stampworld image for a stamp
    pub fn get_stampworld_image(&mut self, _stamp: &Stamp) -> Option<String> {
        // TODO: Implement proper image fetching
        None
    }

    /// Get image from colnect
    pub fn get_colnect_image(&mut self, _stamp: &Stamp) -> Option<String> {
        // TODO: Implement proper image fetching
        None
    }

    /// Get image for a stamp
    pub fn get_image(&mut self, _stamp: &Stamp) -> Option<String> {
        // TODO: Implement proper image fetching
        None
    }

    /// Collect stampworld data
    pub fn collect_stampworld_data<'a>(
        &mut self,
        params: CollectStampworldCallParams<'a>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        use crate::commands_file_generator::data_collection::collect_stampworld_data;
        use crate::types::ImageStatus;

        let mut sequencer = 0;
        let month_number_fn =
            |month: &str| crate::commands_file_generator::utils::month_number(month);

        // Downloader: attempt to fetch and save the image into the configured
        // images directory. Falls back to a no-op ImageStatus when URL is empty.
        let images_dir = self.image_path.clone();
        let downloader = move |url: &str, desired_name: &str, _overwrite: bool| {
            if url.trim().is_empty() {
                return ImageStatus {
                    link: String::new(),
                    exists: false,
                    width: 0,
                    height: 0,
                    filename: String::new(),
                };
            }
            // Derive basename from the URL last segment (drop query string)
            let last = url.rsplit('/').next().unwrap_or("image.jpg");
            let base = last.split('?').next().unwrap_or(last);
            let sanitized_desired = crate::album_utils::sanitize_filename(desired_name);
            let sanitized_base = crate::album_utils::sanitize_filename(base);
            let target_name = format!("{}-{}", sanitized_desired, sanitized_base);
            let file_path = format!("{}/{}", images_dir.trim_end_matches('/'), target_name);
            // If file already exists and overwrite not requested, skip downloading
            if std::path::Path::new(&file_path).exists() && !_overwrite {
                return ImageStatus {
                    link: url.to_string(),
                    exists: true,
                    width: 0,
                    height: 0,
                    filename: target_name,
                };
            }
            crate::commands_file_generator::image_utils::get_image(url, &target_name, &file_path)
        };

        info!("Invoking commands_file_generator::data_collection::collect_stampworld_data with base_url='{}'", params.base_url);
        let mut dparams =
            crate::commands_file_generator::data_collection::CollectStampworldParams {
                sequencer: &mut sequencer,
                separator: &self.separator,
                site_name: &self.country,
                images_directory: &self.image_path,
                month_number_fn: &month_number_fn,
                downloader: &downloader,
                base_url: params.base_url,
                start_year: params.start_year,
                end_year: params.end_year,
                overwrite_images: params.overwrite_images,
                page_callback: params.page_callback,
                max_pages: params.max_pages,
                stop_after_no_hits: params.stop_after_no_hits,
            };
        let stamp_data = collect_stampworld_data(&mut dparams);

        info!(
            "Returned from collect_stampworld_data; received {} stamp data lines",
            stamp_data.len()
        );
        self.stamp_data = stamp_data.clone();
        Ok(stamp_data)
    }

    /// Get touchstamps stamps
    pub fn get_touchstamps_stamps(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement proper touchstamps collection
        Ok(())
    }

    /// Create album file
    pub fn create_album(&self) -> usize {
        let params = crate::commands_file_generator::album_creation::CreateAlbumParams {
            file_path: std::path::Path::new(&self.album_path),
            data: &self.stamp_data,
            album_name: &self.album_name,
            separator: &self.separator,
            fix_sizes: self.fix_sizes,
            formats: &self.formats,
            emissions: &self.emissions,
            catalog_name: &self.catalog_name,
        };
        crate::commands_file_generator::album_creation::create_album(params)
    }

    /// Save stamp data to file
    pub fn save_stamp_data<P: AsRef<std::path::Path>>(
        &self,
        file_path: P,
        data: &[String],
        source: &str,
        overwrite_album: bool,
    ) {
        save_stamp_data(file_path, data, source, overwrite_album, &self.data_source);
    }

    /// Process stamps and generate album data
    pub fn process_stamps(&mut self) {
        // This would contain the main processing logic
        // For now, it's a placeholder to maintain the API
        info!("Processing stamps for album generation");
    }

    /// Validate configuration
    pub fn validate_config(&self) -> Result<(), String> {
        if self.album_name.is_empty() {
            return Err("Album name cannot be empty".to_string());
        }
        if self.album_path.is_empty() {
            return Err("Album path cannot be empty".to_string());
        }
        if self.extract_path.is_empty() {
            return Err("Extract path cannot be empty".to_string());
        }
        Ok(())
    }

    /// Get month number from name
    pub fn month_number(&self, month: &str) -> String {
        month_number(month)
    }

    /// Check if emission is selected
    pub fn is_selected_emission(&self, emission: &str) -> bool {
        is_selected_emission(&self.emissions, emission)
    }

    /// Check if format is selected
    pub fn is_selected_format(&self, format: &str) -> bool {
        is_selected_format(&self.formats, format)
    }

    /// Add a delay
    pub fn snooze(&self, millis: u64) {
        snooze(millis);
    }
}

impl Default for CommandsFileGenerator {
    fn default() -> Self {
        Self::new(
            String::new(),
            Country::default(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            Vec::new(),
            Vec::new(),
            ",".to_string(),
            false,
            false,
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            Config::default(),
        )
    }
}
