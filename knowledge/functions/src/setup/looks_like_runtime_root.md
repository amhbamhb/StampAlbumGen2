---
type: Rust Function
title: looks_like_runtime_root
resource: src/setup.rs#L226-L233
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/setup/find_runtime_root_from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/runtime_root_from_executable_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/align_process_working_dir_to_runtime_layout
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn looks_like_runtime_root(path: &Path) -> bool`

# Called by

- [find_runtime_root_from](../../../functions/src/setup/find_runtime_root_from.md)
- [runtime_root_from_executable_path](../../../functions/src/setup/runtime_root_from_executable_path.md)
- [align_process_working_dir_to_runtime_layout](../../../functions/src/setup/align_process_working_dir_to_runtime_layout.md)