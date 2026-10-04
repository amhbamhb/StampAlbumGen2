---
type: Rust Function
title: create_config_from_template
resource: src/config.rs#L1670-L1710
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_init_tests/test_create_config_from_template_creates_when_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_init_tests/test_create_config_from_template_force_backups_and_overwrites
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn create_config_from_template( path: &str, force: bool, ) -> Result<(), Box<dyn std::error::Error>>`

# Called by

- [main](../../../functions/src/main.md)
- [test_create_config_from_template_creates_when_missing](../../../functions/tests/config_init_tests/test_create_config_from_template_creates_when_missing.md)
- [test_create_config_from_template_force_backups_and_overwrites](../../../functions/tests/config_init_tests/test_create_config_from_template_force_backups_and_overwrites.md)