---
type: Rust Module
title: album_stamp_img_integration_test
resource: tests/album_stamp_img_integration_test.rs#L1-L215
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command
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

- [test_album_stamp_img_new_setting_affects_output](../../functions/tests/album_stamp_img_integration_test/test_album_stamp_img_new_setting_affects_output.md)
- [test_album_stamp_img_new_setting_command_format](../../functions/tests/album_stamp_img_integration_test/test_album_stamp_img_new_setting_command_format.md)
- [test_parameter_usage_consistency](../../functions/tests/album_stamp_img_integration_test/test_parameter_usage_consistency.md)
- [test_default_values_consistency](../../functions/tests/album_stamp_img_integration_test/test_default_values_consistency.md)
- [test_issue_resolution](../../functions/tests/album_stamp_img_integration_test/test_issue_resolution.md)

# Imports

- `std::fs`
- `std::process::Command`
- `tempfile::TempDir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)