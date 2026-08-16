---
type: Rust Function
title: save_album_config_diff
resource: src/review_ui/config_editor.rs#L816-L923
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_template_content_with_fallback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/parse_template_config_sections
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/normalize_value_for_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/load_current_config_values
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/update_default_catalog_code_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn save_album_config_diff( album_dir: &std::path::Path, album_name: &str, global_config_path: &str, template_path: &str, all_ui_values: &HashMap<String, String>, ) -> Result<usize, Box<dyn std::error::Error>>`

# Calls

- [load_template_content_with_fallback](../../../../functions/src/config/load_template_content_with_fallback.md)
- [parse_template_config_sections](../../../../functions/src/review_ui/config_editor/parse_template_config_sections.md)
- [normalize_value_for_template](../../../../functions/src/review_ui/config_editor/normalize_value_for_template.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [ensure_config_and_template_files](../../../../functions/src/config/ensure_config_and_template_files.md)
- [load_current_config_values](../../../../functions/src/review_ui/config_editor/load_current_config_values.md)

# Called by

- [update_default_catalog_code_for_album](../../../../functions/src/review_ui/config_editor/update_default_catalog_code_for_album.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)