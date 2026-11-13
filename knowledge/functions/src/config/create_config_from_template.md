---
type: Rust Function
title: create_config_from_template
resource: src/config.rs#L1642-L1682
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/date/DateFormat/format
    resolved_by: tree-sitter
    confidence: exact
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

# Calls

- [format](../../../functions/src/native_pdf/date/DateFormat/format.md)

# Called by

- [main](../../../functions/src/main.md)
- [test_create_config_from_template_creates_when_missing](../../../functions/tests/config_init_tests/test_create_config_from_template_creates_when_missing.md)
- [test_create_config_from_template_force_backups_and_overwrites](../../../functions/tests/config_init_tests/test_create_config_from_template_force_backups_and_overwrites.md)