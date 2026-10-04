---
type: Rust Function
title: load_config_with_override
resource: src/config.rs#L2664-L2790
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/deep_merge_toml_values
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/albumeasy_pdf_timeout_album_runtime_override_wins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/album_override_section_value_wins_over_root_for_stamp_visibility_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/load_album_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_config_with_override(global_path: &str, album_dir: &std::path::Path) -> Config`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [deep_merge_toml_values](../../../functions/src/config/deep_merge_toml_values.md)

# Called by

- [load_effective_config](../../../functions/src/config/load_effective_config.md)
- [albumeasy_pdf_timeout_album_runtime_override_wins](../../../functions/src/config/albumeasy_pdf_timeout_album_runtime_override_wins.md)
- [album_override_section_value_wins_over_root_for_stamp_visibility_keys](../../../functions/src/config/album_override_section_value_wins_over_root_for_stamp_visibility_keys.md)
- [main](../../../functions/src/main.md)
- [load_album_config](../../../functions/src/review_ui/load_album_config.md)