---
type: Rust Function
title: enrich_field_help
resource: src/review_ui/config_editor.rs#L155-L319
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/build_config_editor_data
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn enrich_field_help(field: &mut ConfigEditorField, help_map: &HashMap<String, Vec<String>>)`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [build_config_editor_data](../../../../functions/src/review_ui/config_editor/build_config_editor_data.md)