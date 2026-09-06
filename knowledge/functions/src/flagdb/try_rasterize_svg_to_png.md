---
type: Rust Function
title: try_rasterize_svg_to_png
resource: src/flagdb.rs#L726-L965
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/ensure_flag_image_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_coa_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/try_fetch_nexperts_resource
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/test_try_rasterize_svg_to_png_roundtrip
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_rasterize_svg_to_png(svg: &Path, png: &Path) -> Result<bool, Box<dyn std::error::Error>>`

# Called by

- [ensure_flag_image_with_fetcher](../../../functions/src/flagdb/ensure_flag_image_with_fetcher.md)
- [ensure_coat_of_arms_with_fetcher](../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [copy_coa_for_album](../../../functions/src/flagdb/copy_coa_for_album.md)
- [try_fetch_nexperts_resource](../../../functions/src/flagdb/try_fetch_nexperts_resource.md)
- [test_try_rasterize_svg_to_png_roundtrip](../../../functions/src/flagdb/test_try_rasterize_svg_to_png_roundtrip.md)