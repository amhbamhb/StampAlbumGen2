---
type: Rust Function
title: refresh_linux_desktop_caches
resource: src/setup.rs#L350-L393
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
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/ensure_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/uninstall_linux_desktop_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn refresh_linux_desktop_caches(applications_dir: &std::path::Path)`

# Calls

- [command_exists](../../../functions/src/setup/command_exists.md)
- [status](../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [ensure_linux_desktop_integration](../../../functions/src/setup/ensure_linux_desktop_integration.md)
- [uninstall_linux_desktop_integration](../../../functions/src/setup/uninstall_linux_desktop_integration.md)