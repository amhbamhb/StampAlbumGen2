---
type: Rust Function
title: is_special_stamp
resource: src/album_globals/commemorative_generator.rs#L51-L61
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_special_stamp(stamp: &Stamp) -> bool`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)