---
type: Rust Function
title: pick_bootstrap_albums_root_via_dialog
resource: src/review_ui.rs#L1977-L2040
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/current_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pick_bootstrap_albums_root_via_dialog() -> Result<Option<String>, String>`

# Calls

- [current_bootstrap_albums_root](../../../functions/src/review_ui/current_bootstrap_albums_root.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)