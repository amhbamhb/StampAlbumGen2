---
type: Rust Function
title: copy_flag_for_album
resource: src/flagdb.rs#L297-L313
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/normalize_country_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/country_to_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_flag_image
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn copy_flag_for_album( country: &str, album_images_dir: &str, size: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [normalize_country_input](../../../functions/src/flagdb/normalize_country_input.md)
- [country_to_code](../../../functions/src/flagdb/country_to_code.md)
- [ensure_flag_image](../../../functions/src/flagdb/ensure_flag_image.md)

# Called by

- [write_globals](../../../functions/src/album_utils/globals/write_globals.md)
- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)