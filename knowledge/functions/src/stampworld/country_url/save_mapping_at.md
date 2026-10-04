---
type: Rust Function
title: save_mapping_at
resource: src/stampworld/country_url.rs#L120-L166
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/stampworld/country_url/save_country_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/saved_mappings_survive_reload_and_preserve_other_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn save_mapping_at(path: &Path, country: &str, slug: Option<&str>) -> Result<(), String>`

# Called by

- [save_country_url](../../../../functions/src/stampworld/country_url/save_country_url.md)
- [saved_mappings_survive_reload_and_preserve_other_settings](../../../../functions/src/stampworld/country_url/saved_mappings_survive_reload_and_preserve_other_settings.md)
- [first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten](../../../../functions/src/stampworld/country_url/first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten.md)