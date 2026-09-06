---
type: Rust Function
title: move_to_country_inbox
resource: src/modes.rs#L277-L303
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/modes/canonical_album_dir_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn move_to_country_inbox( file_path: &std::path::Path, country: &str, ) -> Result<std::path::PathBuf, Box<dyn std::error::Error>>`

# Calls

- [canonical_album_dir_name](../../../functions/src/modes/canonical_album_dir_name.md)

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)