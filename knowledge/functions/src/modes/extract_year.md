---
type: Rust Function
title: extract_year
resource: src/modes.rs#L565-L567
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/remap_colnect_sg_to_existing_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_year(issued_on: &str) -> Option<i32>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [remap_colnect_sg_to_existing_stamps](../../../functions/src/modes/remap_colnect_sg_to_existing_stamps.md)