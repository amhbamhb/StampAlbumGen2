---
type: Rust Method
title: effective_albumeasy_cli_path
resource: src/config.rs#L1508-L1513
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn effective_albumeasy_cli_path(&self) -> Option<&str>`

# Called by

- [generated_artifacts_from_path](../../../../functions/src/album_globals/generated_artifacts_from_path.md)
- [main](../../../../functions/src/main.md)
- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)