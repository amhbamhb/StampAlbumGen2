---
type: Rust Function
title: sanitize_filename
resource: src/album_utils/text_utils.rs#L68-L87
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/commands_file_generator/generator/CommandsFileGenerator/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/data_sources/stampworld/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/test_image_naming/deterministic_target_name_is_sanitized
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/test_image_naming/downloader_skips_existing_file_when_not_overwrite
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn sanitize_filename(filename: &str) -> String`

# Called by

- [collect_stampworld_data](../../../../functions/src/commands_file_generator/generator/CommandsFileGenerator/collect_stampworld_data.md)
- [collect_stampworld_data](../../../../functions/src/data_sources/stampworld/collect_stampworld_data.md)
- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [deterministic_target_name_is_sanitized](../../../../functions/tests/test_image_naming/deterministic_target_name_is_sanitized.md)
- [downloader_skips_existing_file_when_not_overwrite](../../../../functions/tests/test_image_naming/downloader_skips_existing_file_when_not_overwrite.md)