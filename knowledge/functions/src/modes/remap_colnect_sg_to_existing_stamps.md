---
type: Rust Function
title: remap_colnect_sg_to_existing_stamps
resource: src/modes.rs#L617-L735
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/modes/extract_year
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/numeric_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/score_candidate
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/sync_missing_stamps_from_colnect
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn remap_colnect_sg_to_existing_stamps( conn: &rusqlite::Connection, ) -> Result<usize, Box<dyn std::error::Error>>`

# Calls

- [extract_year](../../../functions/src/modes/extract_year.md)
- [numeric_tokens](../../../functions/src/modes/numeric_tokens.md)
- [score_candidate](../../../functions/src/modes/score_candidate.md)

# Called by

- [sync_missing_stamps_from_colnect](../../../functions/src/modes/sync_missing_stamps_from_colnect.md)