---
type: Rust Function
title: calculate_stamp_dimensions
resource: src/album_utils/dimensions.rs#L42-L268
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/dimensions/load_calibration_map
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/estimate_real_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/calc_mm_per_pixel_embedded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/release_publication_shell/success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/get_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn calculate_stamp_dimensions( image_path: &str, album_name: &str, cfg: &crate::config::Config, ) -> (f64, f64)`

# Calls

- [resolve_album_image_path](../../../../functions/src/file_utils/resolve_album_image_path.md)
- [load_calibration_map](../../../../functions/src/album_utils/dimensions/load_calibration_map.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [estimate_real_dimensions](../../../../functions/src/image_utils/estimate_real_dimensions.md)
- [calc_mm_per_pixel_embedded](../../../../functions/src/image_utils/calc_mm_per_pixel_embedded.md)
- [args](../../../../functions/tests/native_pdf_page_settings/args.md)
- [success](../../../../functions/tests/release_publication_shell/success.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)
- [get_image_dimensions](../../../../functions/src/image_utils/get_image_dimensions.md)