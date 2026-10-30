---
type: Rust Method
title: effective_colnect_detail_field_set
resource: src/config.rs#L1202-L1377
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/Config/effective_colnect_detail_fields
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/build_stamp_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/colnect_fields_boolean_toggles_are_parsed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/colnect_printing_type_alias_is_supported
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/colnect_all_false_toggles_override_detail_fields_to_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/colnect_false_override_disables_global_true_field
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn effective_colnect_detail_field_set(&self) -> std::collections::HashSet<String>`

# Calls

- [effective_colnect_detail_fields](../../../../functions/src/config/Config/effective_colnect_detail_fields.md)

# Called by

- [build_stamp_text](../../../../functions/src/album_globals/generator/build_stamp_text.md)
- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [colnect_fields_boolean_toggles_are_parsed](../../../../functions/src/config/colnect_fields_boolean_toggles_are_parsed.md)
- [colnect_printing_type_alias_is_supported](../../../../functions/src/config/colnect_printing_type_alias_is_supported.md)
- [colnect_all_false_toggles_override_detail_fields_to_empty](../../../../functions/src/config/colnect_all_false_toggles_override_detail_fields_to_empty.md)
- [colnect_false_override_disables_global_true_field](../../../../functions/src/config/colnect_false_override_disables_global_true_field.md)