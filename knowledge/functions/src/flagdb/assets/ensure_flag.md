---
type: Rust Function
title: ensure_flag
resource: src/flagdb/assets.rs#L89-L199
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/countries/valid_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/valid_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/rasterize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/write_atomic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/is_missing
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/ensure_flag_image_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/valid_svg_is_rendered_to_a_real_png_at_the_requested_size
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/failed_refresh_preserves_good_cache_and_no_error_body_is_written
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/cached_svg_is_supported_and_invalid_inputs_do_not_access_network
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) async fn ensure_flag( alpha: &str, size: &str, fetcher: &dyn Fetcher, directories: &[PathBuf], overwrite: bool, ) -> AssetResult<String>`

# Calls

- [valid_code](../../../../functions/src/flagdb/countries/valid_code.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)
- [valid_file](../../../../functions/src/flagdb/assets/valid_file.md)
- [rasterize](../../../../functions/src/flagdb/assets/rasterize.md)
- [write_atomic](../../../../functions/src/flagdb/assets/write_atomic.md)
- [is_missing](../../../../functions/src/flagdb/assets/is_missing.md)

# Called by

- [ensure_flag_image_with_fetcher](../../../../functions/src/flagdb/ensure_flag_image_with_fetcher.md)
- [failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors](../../../../functions/src/flagdb/assets/failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors.md)
- [valid_svg_is_rendered_to_a_real_png_at_the_requested_size](../../../../functions/src/flagdb/assets/valid_svg_is_rendered_to_a_real_png_at_the_requested_size.md)
- [valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes](../../../../functions/src/flagdb/assets/valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes.md)
- [failed_refresh_preserves_good_cache_and_no_error_body_is_written](../../../../functions/src/flagdb/assets/failed_refresh_preserves_good_cache_and_no_error_body_is_written.md)
- [cached_svg_is_supported_and_invalid_inputs_do_not_access_network](../../../../functions/src/flagdb/assets/cached_svg_is_supported_and_invalid_inputs_do_not_access_network.md)