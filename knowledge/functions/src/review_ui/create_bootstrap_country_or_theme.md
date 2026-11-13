---
type: Rust Function
title: create_bootstrap_country_or_theme
resource: src/review_ui.rs#L1941-L2008
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/resolve_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn create_bootstrap_country_or_theme( req: &BootstrapCreateCountryRequest, ) -> Result<BootstrapCreateCountryResponse, String>`

# Calls

- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)
- [resolve_bootstrap_albums_root](../../../functions/src/review_ui/resolve_bootstrap_albums_root.md)
- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)