---
type: Rust Function
title: deep_merge_toml_values
resource: src/config.rs#L2606-L2621
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/load_config_with_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/colnect_false_override_disables_global_true_field
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn deep_merge_toml_values(base: toml::Value, overlay: toml::Value) -> toml::Value`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [load_config_with_override](../../../functions/src/config/load_config_with_override.md)
- [colnect_false_override_disables_global_true_field](../../../functions/src/config/colnect_false_override_disables_global_true_field.md)