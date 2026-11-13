---
type: Rust Function
title: embedded_reference_scale_matches_great_britain_albumeasy_footprints
resource: src/native_pdf/stamp_image.rs#L364-L402
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/image_utils/calc_mm_per_pixel_embedded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/page_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn embedded_reference_scale_matches_great_britain_albumeasy_footprints() -> Result<()>`

# Calls

- [calc_mm_per_pixel_embedded](../../../../functions/src/image_utils/calc_mm_per_pixel_embedded.md)
- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [save](../../../../functions/src/review_ui/views/external_album/save.md)
- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [page_layout](../../../../functions/src/native_pdf/stamp_image/page_layout.md)