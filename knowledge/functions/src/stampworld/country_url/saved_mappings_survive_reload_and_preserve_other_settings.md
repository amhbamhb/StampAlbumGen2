---
type: Rust Function
title: saved_mappings_survive_reload_and_preserve_other_settings
resource: src/stampworld/country_url.rs#L228-L251
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

`fn saved_mappings_survive_reload_and_preserve_other_settings()`

# Calls

- [save_mapping_at](../../../../functions/src/stampworld/country_url/save_mapping_at.md)
- [try_load_slug_overrides](../../../../functions/src/config/try_load_slug_overrides.md)