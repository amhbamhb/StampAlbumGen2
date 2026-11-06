---
type: Rust Function
title: is_commemorative
resource: src/album_globals/generator_new/commemorative_layout.rs#L25-L41
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_commemorative(stamp: &Stamp, _year: Option<i32>) -> bool`

# Calls

- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [contains](../../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)