---
type: Rust Function
title: try_fetch_nexperts_resource
resource: src/flagdb.rs#L969-L1003
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/try_rasterize_svg_to_png
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/ensure_flag_image_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn try_fetch_nexperts_resource( fetcher: &dyn Fetcher, alpha: &str, resource: &str, size: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [try_rasterize_svg_to_png](../../../functions/src/flagdb/try_rasterize_svg_to_png.md)

# Called by

- [ensure_flag_image_with_fetcher](../../../functions/src/flagdb/ensure_flag_image_with_fetcher.md)