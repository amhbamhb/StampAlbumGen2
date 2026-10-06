---
type: Rust Function
title: is_directory_empty
resource: src/review_ui.rs#L710-L714
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/set_bootstrap_albums_root_override
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_directory_empty(path: &Path) -> Result<bool, String>`

# Called by

- [set_bootstrap_albums_root_override](../../../functions/src/review_ui/set_bootstrap_albums_root_override.md)