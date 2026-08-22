//! Data collection functionality for gathering stamp data from various sources.
//!
//! This module contains functions for collecting stamp data from websites
//! like StampWorld and processing the collected information.

use crate::types::ImageStatus;
use log::info;

/// Collect stamp data from StampWorld website
pub struct CollectStampworldParams<'a> {
    pub sequencer: &'a mut i32,
    pub separator: &'a str,
    pub site_name: &'a str,
    pub images_directory: &'a str,
    pub month_number_fn: &'a dyn Fn(&str) -> String,
    pub downloader: &'a dyn Fn(&str, &str, bool) -> ImageStatus,
    pub base_url: &'a str,
    pub start_year: i32,
    pub end_year: i32,
    pub overwrite_images: bool,
    pub page_callback: Option<&'a dyn Fn(usize)>,
    pub max_pages: i32,
    pub stop_after_no_hits: i32,
}

pub fn collect_stampworld_data<'a>(params: &mut CollectStampworldParams<'a>) -> Vec<String> {
    info!("commands_file_generator::data_collection::collect_stampworld_data invoked");
    crate::stampworld::collect_stampworld_data(
        params.sequencer,
        params.separator,
        params.site_name,
        params.images_directory,
        params.month_number_fn,
        params.downloader,
        params.base_url,
        params.start_year,
        params.end_year,
        params.overwrite_images,
        params.page_callback,
        params.max_pages,
        params.stop_after_no_hits,
    )
}
