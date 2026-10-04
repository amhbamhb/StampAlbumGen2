---
type: Rust Method
title: colnect_details_enabled
resource: src/config.rs#L1180-L1185
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/build_stamp_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn colnect_details_enabled(&self) -> bool`

# Called by

- [build_stamp_text](../../../../functions/src/album_globals/generator/build_stamp_text.md)
- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)