---
type: Rust Function
title: catalog_display_name
resource: src/catalog.rs#L16-L90
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/all_known_catalog_codes_have_non_catalog_display_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn catalog_display_name(code: &str) -> &'static str`

# Calls

- [normalize_catalog_type](../../../functions/src/catalog/normalize_catalog_type.md)
- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [resolve_stamp_heading_label](../../../functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label.md)
- [all_known_catalog_codes_have_non_catalog_display_names](../../../functions/src/catalog/all_known_catalog_codes_have_non_catalog_display_names.md)
- [effective_stamp_heading_label](../../../functions/src/config/Config/effective_stamp_heading_label.md)