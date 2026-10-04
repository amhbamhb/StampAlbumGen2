---
type: Rust Function
title: valid_code
resource: src/flagdb/countries.rs#L14-L16
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/assets/ensure_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/parse_iso_codes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/merge_overrides
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn valid_code(code: &str) -> bool`

# Called by

- [ensure_flag](../../../../functions/src/flagdb/assets/ensure_flag.md)
- [parse_iso_codes](../../../../functions/src/flagdb/countries/parse_iso_codes.md)
- [merge_overrides](../../../../functions/src/flagdb/countries/merge_overrides.md)