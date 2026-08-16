---
type: Rust Function
title: update_default_catalog_code_for_album
resource: src/review_ui/config_editor.rs#L8-L52
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/load_current_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/save_album_config_diff
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn update_default_catalog_code_for_album( album_dir: &std::path::Path, album_name: &str, global_config_path: &str, template_path: &str, requested_code: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [normalize_catalog_type](../../../../functions/src/catalog/normalize_catalog_type.md)
- [load_current_config_values](../../../../functions/src/review_ui/config_editor/load_current_config_values.md)
- [save_album_config_diff](../../../../functions/src/review_ui/config_editor/save_album_config_diff.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)