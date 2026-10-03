---
type: Rust Function
title: pick_albumeasy_cli_path_via_dialog
resource: src/review_ui.rs#L2061-L2129
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pick_albumeasy_cli_path_via_dialog( current_path: Option<&str>, ) -> Result<Option<String>, String>`

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)