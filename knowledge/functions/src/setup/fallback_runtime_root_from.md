---
type: Rust Function
title: fallback_runtime_root_from
resource: src/setup.rs#L267-L272
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/setup/align_process_working_dir_to_runtime_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fallback_runtime_root_from(exe_path: &Path) -> Option<PathBuf>`

# Called by

- [align_process_working_dir_to_runtime_layout](../../../functions/src/setup/align_process_working_dir_to_runtime_layout.md)