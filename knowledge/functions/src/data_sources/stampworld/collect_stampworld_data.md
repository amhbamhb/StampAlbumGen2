---
type: Rust Function
title: collect_stampworld_data
resource: src/data_sources/stampworld.rs#L5-L375
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/text_utils/sanitize_filename
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn collect_stampworld_data( sequencer: &mut i32, separator: &str, _site_name: &str, _images_directory: &str, month_number: &dyn Fn(&str) -> String, get_stampworld_image: &dyn Fn(&str, &str, bool) -> ImageStatus, base_url: &str, start_year: i32, end_year: i32, overwrite: bool, ) -> Vec<String>`

# Calls

- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [html](../../../../functions/src/review_ui/views/compare/html.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)
- [sanitize_filename](../../../../functions/src/album_utils/text_utils/sanitize_filename.md)