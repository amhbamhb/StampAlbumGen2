---
type: Rust Function
title: runtime_root_from_executable_path
resource: src/setup.rs#L274-L300
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/looks_like_runtime_root
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/align_process_working_dir_to_runtime_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn runtime_root_from_executable_path(exe_path: &Path) -> Option<PathBuf>`

# Calls

- [looks_like_runtime_root](../../../functions/src/setup/looks_like_runtime_root.md)

# Called by

- [align_process_working_dir_to_runtime_layout](../../../functions/src/setup/align_process_working_dir_to_runtime_layout.md)