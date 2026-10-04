---
type: Rust Function
title: resolve_stamp_heading_label
resource: src/album_globals/generator_new/emit.rs#L233-L291
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
  - target: functions/src/album_globals/generator/stamp_content_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
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

- [stamp_content_height](../../../../../functions/src/album_globals/generator/stamp_content_height.md)
- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [emit_stamp_lines](../../../../../functions/src/album_globals/generator_new/emit/emit_stamp_lines.md)