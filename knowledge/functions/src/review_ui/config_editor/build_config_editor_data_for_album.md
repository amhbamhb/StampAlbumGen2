---
type: Rust Function
title: build_config_editor_data_for_album
resource: src/review_ui/config_editor.rs#L770-L817
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/build_config_editor_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/load_current_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn build_config_editor_data_for_album( global_config_path: &str, template_path: &str, pdf_path: &str, album_dir: &std::path::Path, ) -> Result<(Vec<ConfigEditorSection>, bool), Box<dyn std::error::Error>>`

# Calls

- [build_config_editor_data](../../../../functions/src/review_ui/config_editor/build_config_editor_data.md)
- [load_current_config_values](../../../../functions/src/review_ui/config_editor/load_current_config_values.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)