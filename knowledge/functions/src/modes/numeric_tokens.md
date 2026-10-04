---
type: Rust Function
title: numeric_tokens
resource: src/modes.rs#L571-L577
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/remap_colnect_sg_to_existing_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn numeric_tokens(v: &str) -> Vec<String>`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [remap_colnect_sg_to_existing_stamps](../../../functions/src/modes/remap_colnect_sg_to_existing_stamps.md)