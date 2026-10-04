---
type: Rust Function
title: first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten
resource: src/stampworld/country_url.rs#L254-L270
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/country_url/save_mapping_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/try_load_slug_overrides
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten()`

# Calls

- [save_mapping_at](../../../../functions/src/stampworld/country_url/save_mapping_at.md)
- [try_load_slug_overrides](../../../../functions/src/config/try_load_slug_overrides.md)