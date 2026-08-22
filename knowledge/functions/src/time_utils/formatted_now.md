---
type: Rust Function
title: formatted_now
resource: src/time_utils.rs#L6-L12
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/migrate_fix_block_metadata/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/migrate_stamps/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/album_creation/save_stamp_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_startup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/setup_logging
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/write_startup_probe
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/time_utils/timestamp_compact
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn formatted_now(fmt: &str) -> String`

# Called by

- [generate_album_output_conn](../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [write_globals](../../../functions/src/album_utils/globals/write_globals.md)
- [main](../../../functions/src/bin/backfill_image_dims/main.md)
- [main](../../../functions/src/bin/migrate_fix_block_metadata/main.md)
- [main](../../../functions/src/bin/migrate_stamps/main.md)
- [save_stamp_data](../../../functions/src/commands_file_generator/album_creation/save_stamp_data.md)
- [log_startup](../../../functions/src/review_ui/log_startup.md)
- [setup_logging](../../../functions/src/setup/setup_logging.md)
- [write_startup_probe](../../../functions/src/setup/write_startup_probe.md)
- [timestamp_compact](../../../functions/src/time_utils/timestamp_compact.md)