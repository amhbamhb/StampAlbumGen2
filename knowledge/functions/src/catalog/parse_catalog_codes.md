---
type: Rust Function
title: parse_catalog_codes
resource: src/catalog.rs#L114-L127
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/catalog/preferred_catalog_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/infer_catalog_type_for_number
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_catalog_codes(raw: &str) -> Vec<(String, String)>`

# Calls

- [normalize_catalog_type](../../../functions/src/catalog/normalize_catalog_type.md)

# Called by

- [preferred_catalog_entry](../../../functions/src/catalog/preferred_catalog_entry.md)
- [infer_catalog_type_for_number](../../../functions/src/catalog/infer_catalog_type_for_number.md)