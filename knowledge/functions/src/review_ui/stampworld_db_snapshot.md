---
type: Rust Function
title: stampworld_db_snapshot
resource: src/review_ui.rs#L610-L649
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/pdf_parser/extraction/metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn stampworld_db_snapshot(db_path: &str) -> String`

# Calls

- [metadata](../../../functions/src/review_ui/pdf_parser/extraction/metadata.md)
- [open](../../../functions/src/external_album/open.md)