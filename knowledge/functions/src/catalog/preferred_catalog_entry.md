---
type: Rust Function
title: preferred_catalog_entry
resource: src/catalog.rs#L129-L144
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/catalog/parse_catalog_codes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/preferred_catalog_number
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn preferred_catalog_entry(raw: &str, default_catalog_code: &str) -> Option<(String, String)>`

# Calls

- [parse_catalog_codes](../../../functions/src/catalog/parse_catalog_codes.md)
- [normalize_catalog_type](../../../functions/src/catalog/normalize_catalog_type.md)

# Called by

- [fetch_and_filter_stamps_impl](../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl.md)
- [preferred_catalog_number](../../../functions/src/catalog/preferred_catalog_number.md)