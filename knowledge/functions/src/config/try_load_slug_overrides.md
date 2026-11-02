---
type: Rust Function
title: try_load_slug_overrides
resource: src/config.rs#L2822-L2845
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/slug_override_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/load_slug_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/country_url/load_country_url
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

`pub(crate) fn try_load_slug_overrides(path: &str) -> Result<HashMap<String, String>, String>`

# Calls

- [slug_override_path](../../../functions/src/config/slug_override_path.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [load_slug_overrides](../../../functions/src/config/load_slug_overrides.md)
- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)
- [load_country_url](../../../functions/src/stampworld/country_url/load_country_url.md)
- [saved_mappings_survive_reload_and_preserve_other_settings](../../../functions/src/stampworld/country_url/saved_mappings_survive_reload_and_preserve_other_settings.md)
- [first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten](../../../functions/src/stampworld/country_url/first_save_creates_a_mapping_file_and_invalid_table_is_not_overwritten.md)