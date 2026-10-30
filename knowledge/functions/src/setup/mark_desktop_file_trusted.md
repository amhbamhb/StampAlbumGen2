---
type: Rust Function
title: mark_desktop_file_trusted
resource: src/setup.rs#L408-L424
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/command_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn mark_desktop_file_trusted(path: &std::path::Path)`

# Calls

- [command_exists](../../../functions/src/setup/command_exists.md)
- [status](../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)

# Called by

- [ensure_linux_desktop_integration](../../../functions/src/setup/ensure_linux_desktop_integration.md)