//! Data collection functionality for gathering stamp data from various sources.
//!
//! This module contains functions for collecting stamp data from websites
//! like StampWorld and processing the collected information.

use crate::types::ImageStatus;
use log::info;

/// Collect stamp data from StampWorld website
pub fn collect_stampworld_data(
    sequencer: &mut i32,
    separator: &str,
    site_name: &str,
    images_directory: &str,
    month_number_fn: &dyn Fn(&str) -> String,
    _downloader: &dyn Fn(&str, &str, bool) -> ImageStatus,
    base_url: &str,
    start_year: i32,
    end_year: i32,
    overwrite_images: bool,
    page_callback: Option<&dyn Fn(usize)>,
    max_pages: i32,
    stop_after_no_hits: i32,
) -> Vec<String> {
    info!("commands_file_generator::data_collection::collect_stampworld_data invoked");
    // Use the downloader provided by the caller so callers (like the
    // CommandsFileGenerator) can implement actual image fetching.
    crate::stampworld::collect_stampworld_data(
        sequencer,
        separator,
        site_name,
        images_directory,
        month_number_fn,
        _downloader,
        base_url,
        start_year,
        end_year,
        overwrite_images,
        page_callback,
        max_pages,
        stop_after_no_hits,
    )
}
