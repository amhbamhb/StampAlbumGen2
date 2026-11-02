---
type: Rust Function
title: command_exists
resource: src/setup.rs#L341-L348
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/refresh_linux_desktop_caches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/mark_desktop_file_trusted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn command_exists(name: &str) -> bool`

# Calls

- [status](../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)

# Called by

- [refresh_linux_desktop_caches](../../../functions/src/setup/refresh_linux_desktop_caches.md)
- [mark_desktop_file_trusted](../../../functions/src/setup/mark_desktop_file_trusted.md)