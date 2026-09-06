---
type: Rust Function
title: collect_stampworld_data
resource: src/stampworld/data_collector.rs#L1538-L1571
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn collect_stampworld_data( sequencer: &mut i32, _separator: &str, site_name: &str, images_directory: &str, month_number: &dyn Fn(&str) -> String, get_stampworld_image: &dyn Fn(&str, &str, bool) -> ImageStatus, base_url: &str, start_year: i32, end_year: i32, overwrite_images: bool, page_callback: Option<&dyn Fn(usize)>, max_pages: i32, stop_after_no_hits: i32, ) -> Vec<String>`

# Calls

- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)