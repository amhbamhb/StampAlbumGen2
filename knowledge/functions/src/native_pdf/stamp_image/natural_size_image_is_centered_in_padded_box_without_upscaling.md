---
type: Rust Function
title: natural_size_image_is_centered_in_padded_box_without_upscaling
resource: src/native_pdf/stamp_image.rs#L288-L361
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
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

`fn natural_size_image_is_centered_in_padded_box_without_upscaling() -> Result<()>`

# Calls

- [save](../../../../functions/src/review_ui/views/external_album/save.md)
- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [page_layout](../../../../functions/src/native_pdf/stamp_image/page_layout.md)