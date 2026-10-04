---
type: Rust Function
title: copy_flag_for_album
resource: src/flagdb.rs#L167-L190
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/country_to_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_flag_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/copy_to_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/local_album_flag
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/flagdb/unmapped_country_can_reuse_a_valid_album_local_flag_without_network
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn copy_flag_for_album( country: &str, album_images_dir: &str, size: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [country_to_code](../../../functions/src/flagdb/country_to_code.md)
- [ensure_flag_image](../../../functions/src/flagdb/ensure_flag_image.md)
- [copy_to_album](../../../functions/src/flagdb/assets/copy_to_album.md)
- [local_album_flag](../../../functions/src/flagdb/assets/local_album_flag.md)

# Called by

- [write_globals](../../../functions/src/album_utils/globals/write_globals.md)
- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)
- [unmapped_country_can_reuse_a_valid_album_local_flag_without_network](../../../functions/tests/flagdb/unmapped_country_can_reuse_a_valid_album_local_flag_without_network.md)