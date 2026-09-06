---
type: Rust Function
title: collect_colnect_csv_files_rec
resource: src/modes.rs#L137-L154
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/modes/is_colnect_export_filename
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn collect_colnect_csv_files_rec( root: &std::path::Path, out: &mut Vec<std::path::PathBuf>, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [is_colnect_export_filename](../../../functions/src/modes/is_colnect_export_filename.md)

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)