---
type: Rust Function
title: bundled_map
resource: src/flagdb/countries.rs#L67-L73
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/countries/parse_iso_codes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/merge_overrides
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/countries/partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn bundled_map() -> Result<HashMap<String, String>, String>`

# Calls

- [parse_iso_codes](../../../../functions/src/flagdb/countries/parse_iso_codes.md)
- [merge_overrides](../../../../functions/src/flagdb/countries/merge_overrides.md)

# Called by

- [partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply](../../../../functions/src/flagdb/countries/partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply.md)