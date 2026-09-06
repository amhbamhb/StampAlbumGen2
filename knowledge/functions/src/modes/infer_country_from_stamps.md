---
type: Rust Function
title: infer_country_from_stamps
resource: src/modes.rs#L266-L275
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn infer_country_from_stamps(stamps: &[crate::colnect::parser::ColnectStamp]) -> Option<String>`

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)