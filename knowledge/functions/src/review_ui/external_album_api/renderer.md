---
type: Rust Function
title: renderer
resource: src/review_ui/external_album_api.rs#L75-L88
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/Config/effective_albumeasy_cli_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/albumeasy_pdf_timeout_secs
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn renderer() -> Result<(String, Duration), String>`

# Calls

- [effective_albumeasy_cli_path](../../../../functions/src/config/Config/effective_albumeasy_cli_path.md)
- [albumeasy_pdf_timeout_secs](../../../../functions/src/config/Config/albumeasy_pdf_timeout_secs.md)