---
type: Rust Function
title: save_stamp_data
resource: src/commands_file_generator/album_creation.rs#L198-L312
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn save_stamp_data<P: AsRef<std::path::Path>>( file_path: P, data: &[String], _source: &str, overwrite_album: bool, data_source: &str, )`

# Calls

- [formatted_now](../../../../functions/src/time_utils/formatted_now.md)