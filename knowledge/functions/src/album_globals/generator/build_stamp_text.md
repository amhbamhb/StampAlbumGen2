---
type: Rust Function
title: build_stamp_text
resource: src/album_globals/generator.rs#L295-L849
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/clear
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/format_number_with_commas
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/colnect_details_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_colnect_detail_field_set
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/strip_bracket_tags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/is_effectively_duplicate_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/pad_centre_body_for_side_columns
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/stamp_content_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/colnect_height_uses_the_exact_emitted_text_for_every_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_stamp_text( stamp: &Stamp, config: &crate::config::Config, emit_width: f64, emit_height: f64, ) -> StampText`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [clear](../../../../functions/src/progress/ProgressIndicator/clear.md)
- [format_number_with_commas](../../../../functions/src/album_globals/generator/format_number_with_commas.md)
- [skip](../../../../functions/src/external_album/parser/Lexer/skip.md)
- [colnect_details_enabled](../../../../functions/src/config/Config/colnect_details_enabled.md)
- [effective_colnect_detail_field_set](../../../../functions/src/config/Config/effective_colnect_detail_field_set.md)
- [strip_bracket_tags](../../../../functions/src/album_globals/generator/strip_bracket_tags.md)
- [is_effectively_duplicate_text](../../../../functions/src/album_globals/generator/is_effectively_duplicate_text.md)
- [pad_centre_body_for_side_columns](../../../../functions/src/album_globals/generator/pad_centre_body_for_side_columns.md)

# Called by

- [stamp_content_height](../../../../functions/src/album_globals/generator/stamp_content_height.md)
- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [colnect_height_uses_the_exact_emitted_text_for_every_layout](../../../../functions/src/album_globals/generator/colnect_height_uses_the_exact_emitted_text_for_every_layout.md)
- [colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags](../../../../functions/src/album_globals/generator/colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags.md)