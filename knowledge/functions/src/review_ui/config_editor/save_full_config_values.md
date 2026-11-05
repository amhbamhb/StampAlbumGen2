---
type: Rust Function
title: save_full_config_values
resource: src/review_ui/config_editor.rs#L737-L771
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
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
  - target: functions/src/review_ui/config_editor/render_config_from_template_values
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn save_full_config_values( config_path: &str, template_path: &str, pdf_path: &str, values: &HashMap<String, String>, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [ensure_config_and_template_files](../../../../functions/src/config/ensure_config_and_template_files.md)
- [load_template_content_with_fallback](../../../../functions/src/config/load_template_content_with_fallback.md)
- [parse_template_config_sections](../../../../functions/src/review_ui/config_editor/parse_template_config_sections.md)
- [normalize_value_for_template](../../../../functions/src/review_ui/config_editor/normalize_value_for_template.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [render_config_from_template_values](../../../../functions/src/review_ui/config_editor/render_config_from_template_values.md)

# Called by

- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)