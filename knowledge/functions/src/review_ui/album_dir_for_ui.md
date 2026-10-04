---
type: Rust Function
title: album_dir_for_ui
resource: src/review_ui.rs#L2610-L2619
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/build_matcher_payload
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/load_album_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/store_uploaded_image_from_data_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/colnect_import_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn album_dir_for_ui(db_path: &str, _album_name: &str) -> PathBuf`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [build_matcher_payload](../../../functions/src/review_ui/build_matcher_payload.md)
- [load_album_config](../../../functions/src/review_ui/load_album_config.md)
- [store_uploaded_image_from_data_url](../../../functions/src/review_ui/store_uploaded_image_from_data_url.md)
- [colnect_import_data](../../../functions/src/review_ui/colnect_import_data.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)