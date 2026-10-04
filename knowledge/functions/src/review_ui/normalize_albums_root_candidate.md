---
type: Rust Function
title: normalize_albums_root_candidate
resource: src/review_ui.rs#L2387-L2407
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/set_bootstrap_albums_root_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/resolve_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn normalize_albums_root_candidate(path: &Path) -> Option<PathBuf>`

# Called by

- [set_bootstrap_albums_root_override](../../../functions/src/review_ui/set_bootstrap_albums_root_override.md)
- [resolve_bootstrap_albums_root](../../../functions/src/review_ui/resolve_bootstrap_albums_root.md)