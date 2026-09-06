---
type: Rust Function
title: escape_ae_text
resource: src/album_globals/generator_new/emit.rs#L4-L8
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator_new/emit/emit_stamp_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/emit_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/emit_image_only
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn escape_ae_text(v: &str) -> String`

# Called by

- [emit_stamp_lines](../../../../../functions/src/album_globals/generator_new/emit/emit_stamp_lines.md)
- [emit_stamp](../../../../../functions/src/album_globals/generator_new/emit/emit_stamp.md)
- [emit_image_only](../../../../../functions/src/album_globals/generator_new/emit/emit_image_only.md)