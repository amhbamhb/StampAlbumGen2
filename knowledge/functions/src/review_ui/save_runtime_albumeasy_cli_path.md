---
type: Rust Function
title: save_runtime_albumeasy_cli_path
resource: src/review_ui.rs#L643-L650
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/save_runtime_string_setting
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn save_runtime_albumeasy_cli_path(cli_path: &str) -> Result<(), String>`

# Calls

- [save_runtime_string_setting](../../../functions/src/review_ui/save_runtime_string_setting.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)