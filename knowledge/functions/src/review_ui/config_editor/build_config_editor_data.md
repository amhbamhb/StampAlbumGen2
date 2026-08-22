---
type: Rust Function
title: build_config_editor_data
resource: src/review_ui/config_editor.rs#L509-L606
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
  - target: functions/src/review_ui/config_editor/load_example_config_help_map
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/load_current_config_values
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/enrich_field_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/friendly_config_section_title
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/build_config_editor_data_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn build_config_editor_data( config_path: &str, template_path: &str, ) -> Result<Vec<ConfigEditorSection>, Box<dyn std::error::Error>>`

# Calls

- [ensure_config_and_template_files](../../../../functions/src/config/ensure_config_and_template_files.md)
- [load_template_content_with_fallback](../../../../functions/src/config/load_template_content_with_fallback.md)
- [parse_template_config_sections](../../../../functions/src/review_ui/config_editor/parse_template_config_sections.md)
- [load_example_config_help_map](../../../../functions/src/review_ui/config_editor/load_example_config_help_map.md)
- [load_current_config_values](../../../../functions/src/review_ui/config_editor/load_current_config_values.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [enrich_field_help](../../../../functions/src/review_ui/config_editor/enrich_field_help.md)
- [friendly_config_section_title](../../../../functions/src/review_ui/config_editor/friendly_config_section_title.md)

# Called by

- [build_config_editor_data_for_album](../../../../functions/src/review_ui/config_editor/build_config_editor_data_for_album.md)
- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)