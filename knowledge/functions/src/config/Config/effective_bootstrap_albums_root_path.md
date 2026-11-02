---
type: Rust Method
title: effective_bootstrap_albums_root_path
resource: src/config.rs#L1589-L1594
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/file_utils/resolve_runtime_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/resolve_runtime_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/resolve_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn effective_bootstrap_albums_root_path(&self) -> Option<&str>`

# Called by

- [resolve_runtime_albums_root](../../../../functions/src/file_utils/resolve_runtime_albums_root.md)
- [resolve_runtime_albums_root](../../../../functions/src/modes/resolve_runtime_albums_root.md)
- [resolve_bootstrap_albums_root](../../../../functions/src/review_ui/resolve_bootstrap_albums_root.md)