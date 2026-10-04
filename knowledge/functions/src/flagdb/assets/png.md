---
type: Rust Function
title: png
resource: src/flagdb/assets.rs#L277-L284
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/assets/failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/failed_refresh_preserves_good_cache_and_no_error_body_is_written
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn png(color: u8) -> Vec<u8>`

# Called by

- [failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors](../../../../functions/src/flagdb/assets/failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors.md)
- [valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes](../../../../functions/src/flagdb/assets/valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes.md)
- [failed_refresh_preserves_good_cache_and_no_error_body_is_written](../../../../functions/src/flagdb/assets/failed_refresh_preserves_good_cache_and_no_error_body_is_written.md)