---
type: Rust Function
title: try_rasterize_svg_to_png
resource: src/flagdb.rs#L603-L842
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/release_publication_shell/success
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_coa_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/test_try_rasterize_svg_to_png_roundtrip
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_rasterize_svg_to_png(svg: &Path, png: &Path) -> Result<bool, Box<dyn std::error::Error>>`

# Calls

- [save](../../../functions/src/review_ui/views/external_album/save.md)
- [status](../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)
- [success](../../../functions/tests/release_publication_shell/success.md)
- [open](../../../functions/src/external_album/open.md)

# Called by

- [ensure_coat_of_arms_with_fetcher](../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [copy_coa_for_album](../../../functions/src/flagdb/copy_coa_for_album.md)
- [test_try_rasterize_svg_to_png_roundtrip](../../../functions/src/flagdb/test_try_rasterize_svg_to_png_roundtrip.md)