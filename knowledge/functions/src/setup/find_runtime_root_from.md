---
type: Rust Function
title: find_runtime_root_from
resource: src/setup.rs#L233-L265
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
  - target: functions/src/setup/runtime_root_search_prefers_exe_directory_markers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_runtime_root_from(current_dir: &Path, exe_path: &Path) -> Option<PathBuf>`

# Calls

- [looks_like_runtime_root](../../../functions/src/setup/looks_like_runtime_root.md)

# Called by

- [align_process_working_dir_to_runtime_layout](../../../functions/src/setup/align_process_working_dir_to_runtime_layout.md)
- [runtime_root_search_prefers_exe_directory_markers](../../../functions/src/setup/runtime_root_search_prefers_exe_directory_markers.md)