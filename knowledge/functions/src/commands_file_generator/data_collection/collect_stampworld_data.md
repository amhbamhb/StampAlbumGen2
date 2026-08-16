---
type: Rust Function
title: collect_stampworld_data
resource: src/commands_file_generator/data_collection.rs#L10-L43
generated:
  by: okf-rs/0.4.0
---

# Signature

`pub fn collect_stampworld_data( sequencer: &mut i32, separator: &str, site_name: &str, images_directory: &str, month_number_fn: &dyn Fn(&str) -> String, _downloader: &dyn Fn(&str, &str, bool) -> ImageStatus, base_url: &str, start_year: i32, end_year: i32, overwrite_images: bool, page_callback: Option<&dyn Fn(usize)>, max_pages: i32, stop_after_no_hits: i32, ) -> Vec<String>`