---
type: Rust Function
title: partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply
resource: src/flagdb/countries.rs#L109-L124
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/countries/bundled_map
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/merge_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply()`

# Calls

- [bundled_map](../../../../functions/src/flagdb/countries/bundled_map.md)
- [merge_overrides](../../../../functions/src/flagdb/countries/merge_overrides.md)
- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)