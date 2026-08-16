---
type: Rust Function
title: estimate_colnect_centre_text
resource: src/album_globals/generator.rs#L369-L430
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/Config/colnect_details_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_colnect_detail_field_set
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn estimate_colnect_centre_text(stamp: &Stamp, config: &crate::config::Config) -> String`

# Calls

- [colnect_details_enabled](../../../../functions/src/config/Config/colnect_details_enabled.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [effective_colnect_detail_field_set](../../../../functions/src/config/Config/effective_colnect_detail_field_set.md)

# Called by

- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)