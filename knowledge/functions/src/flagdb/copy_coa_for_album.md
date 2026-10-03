---
type: Rust Function
title: copy_coa_for_album
resource: src/flagdb.rs#L672-L721
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
  - target: functions/src/flagdb/ensure_coat_of_arms
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/try_rasterize_svg_to_png
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn copy_coa_for_album( country: &str, album_images_dir: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [normalize_country_input](../../../functions/src/flagdb/normalize_country_input.md)
- [country_to_code](../../../functions/src/flagdb/country_to_code.md)
- [ensure_coat_of_arms](../../../functions/src/flagdb/ensure_coat_of_arms.md)
- [try_rasterize_svg_to_png](../../../functions/src/flagdb/try_rasterize_svg_to_png.md)

# Called by

- [write_globals](../../../functions/src/album_utils/globals/write_globals.md)