---
type: Rust Function
title: current_bootstrap_albums_root
resource: src/review_ui.rs#L1973-L1975
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/resolve_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/pick_bootstrap_albums_root_via_dialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/set_bootstrap_albums_root_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn current_bootstrap_albums_root() -> Option<String>`

# Calls

- [resolve_bootstrap_albums_root](../../../functions/src/review_ui/resolve_bootstrap_albums_root.md)

# Called by

- [pick_bootstrap_albums_root_via_dialog](../../../functions/src/review_ui/pick_bootstrap_albums_root_via_dialog.md)
- [set_bootstrap_albums_root_override](../../../functions/src/review_ui/set_bootstrap_albums_root_override.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)