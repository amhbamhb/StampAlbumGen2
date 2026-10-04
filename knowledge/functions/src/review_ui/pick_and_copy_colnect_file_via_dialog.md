---
type: Rust Function
title: pick_and_copy_colnect_file_via_dialog
resource: src/review_ui.rs#L2055-L2075
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/scan_bootstrap_countries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pick_and_copy_colnect_file_to_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pick_and_copy_colnect_file_via_dialog(country: &str) -> BootstrapPickColnectFileResponse`

# Calls

- [scan_bootstrap_countries](../../../functions/src/review_ui/scan_bootstrap_countries.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [pick_and_copy_colnect_file_to_dir](../../../functions/src/review_ui/pick_and_copy_colnect_file_to_dir.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)