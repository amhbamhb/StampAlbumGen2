---
type: Rust Function
title: ensure_flag_image_with_fetcher
resource: src/flagdb.rs#L196-L281
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/try_rasterize_svg_to_png
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/ensure_flag_image
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_flag_image_with_fetcher( alpha2: &str, size: &str, fetcher: &dyn Fetcher, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [try_rasterize_svg_to_png](../../../functions/src/flagdb/try_rasterize_svg_to_png.md)

# Called by

- [ensure_flag_image](../../../functions/src/flagdb/ensure_flag_image.md)