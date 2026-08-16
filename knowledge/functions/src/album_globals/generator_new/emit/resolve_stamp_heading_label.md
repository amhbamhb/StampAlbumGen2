---
type: Rust Function
title: resolve_stamp_heading_label
resource: src/album_globals/generator_new/emit.rs#L224-L282
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/Config/effective_default_catalog_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/catalog_display_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/emit_stamp_lines
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_stamp_heading_label( config: &crate::config::Config, catalog_code: Option<&str>, ) -> String`

# Calls

- [effective_default_catalog_code](../../../../../functions/src/config/Config/effective_default_catalog_code.md)
- [catalog_display_name](../../../../../functions/src/catalog/catalog_display_name.md)
- [as_str](../../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [normalize_catalog_type](../../../../../functions/src/catalog/normalize_catalog_type.md)

# Called by

- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [emit_stamp_lines](../../../../../functions/src/album_globals/generator_new/emit/emit_stamp_lines.md)