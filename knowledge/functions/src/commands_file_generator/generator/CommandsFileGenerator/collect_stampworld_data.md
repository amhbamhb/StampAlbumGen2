---
type: Rust Method
title: collect_stampworld_data
resource: src/commands_file_generator/generator.rs#L128-L201
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_utils/text_utils/sanitize_filename
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn collect_stampworld_data( &mut self, base_url: &str, overwrite_images: bool, page_callback: Option<&dyn Fn(usize)>, max_pages: i32, stop_after_no_hits: i32, start_year: i32, end_year: i32, ) -> Result<Vec<String>, Box<dyn std::error::Error>>`

# Calls

- [sanitize_filename](../../../../../functions/src/album_utils/text_utils/sanitize_filename.md)