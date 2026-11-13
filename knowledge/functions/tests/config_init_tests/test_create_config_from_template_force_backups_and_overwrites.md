---
type: Rust Function
title: test_create_config_from_template_force_backups_and_overwrites
resource: tests/config_init_tests.rs#L34-L73
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/create_config_from_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_create_config_from_template_force_backups_and_overwrites()`

# Calls

- [create_config_from_template](../../../functions/src/config/create_config_from_template.md)
- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)