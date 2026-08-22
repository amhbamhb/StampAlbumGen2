---
type: Rust Module
title: config_init_tests
resource: tests/config_init_tests.rs#L1-L82
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serial-test-serial
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [test_create_config_from_template_creates_when_missing](../../functions/tests/config_init_tests/test_create_config_from_template_creates_when_missing.md)
- [test_create_config_from_template_force_backups_and_overwrites](../../functions/tests/config_init_tests/test_create_config_from_template_force_backups_and_overwrites.md)
- [test_template_defaults_use_fit_box_sizing](../../functions/tests/config_init_tests/test_template_defaults_use_fit_box_sizing.md)

# Imports

- `std::fs`
- `serial_test::serial`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)