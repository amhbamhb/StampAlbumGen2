---
type: Rust Function
title: ensure_coat_of_arms_with_fetcher
resource: src/flagdb.rs#L199-L546
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
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/cache_country_info
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/extract_coa_from_nexperts_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/extract_from_obj
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/ensure_coat_of_arms
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn ensure_coat_of_arms_with_fetcher( alpha2: &str, fetcher: &dyn Fetcher, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [try_rasterize_svg_to_png](../../../functions/src/flagdb/try_rasterize_svg_to_png.md)
- [load_effective_config](../../../functions/src/config/load_effective_config.md)
- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)
- [cache_country_info](../../../functions/src/flagdb/cache_country_info.md)
- [extract_coa_from_nexperts_html](../../../functions/src/flagdb/extract_coa_from_nexperts_html.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [save](../../../functions/src/review_ui/views/external_album/save.md)
- [extract_from_obj](../../../functions/src/flagdb/extract_from_obj.md)

# Called by

- [ensure_coat_of_arms](../../../functions/src/flagdb/ensure_coat_of_arms.md)