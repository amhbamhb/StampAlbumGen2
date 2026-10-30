---
type: Rust Function
title: align_process_working_dir_to_runtime_layout
resource: src/setup.rs#L304-L335
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/looks_like_runtime_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/runtime_root_from_executable_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/find_runtime_root_from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/fallback_runtime_root_from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/align_runtime_dir_noops_when_already_root
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn align_process_working_dir_to_runtime_layout() -> std::io::Result<Option<PathBuf>>`

# Calls

- [looks_like_runtime_root](../../../functions/src/setup/looks_like_runtime_root.md)
- [runtime_root_from_executable_path](../../../functions/src/setup/runtime_root_from_executable_path.md)
- [find_runtime_root_from](../../../functions/src/setup/find_runtime_root_from.md)
- [fallback_runtime_root_from](../../../functions/src/setup/fallback_runtime_root_from.md)

# Called by

- [main](../../../functions/src/main.md)
- [align_runtime_dir_noops_when_already_root](../../../functions/src/setup/align_runtime_dir_noops_when_already_root.md)