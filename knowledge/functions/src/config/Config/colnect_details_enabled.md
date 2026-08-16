---
type: Rust Method
title: colnect_details_enabled
resource: src/config.rs#L1014-L1019
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/estimate_colnect_centre_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn colnect_details_enabled(&self) -> bool`

# Called by

- [estimate_colnect_centre_text](../../../../functions/src/album_globals/generator/estimate_colnect_centre_text.md)
- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)