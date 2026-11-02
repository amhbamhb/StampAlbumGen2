---
type: Rust Function
title: stamp_content_height
resource: src/album_globals/generator.rs#L852-L903
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator/build_stamp_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/count_albumeasy_text_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/estimate_stamp_heading_line_count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
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

`fn stamp_content_height( stamp: &Stamp, config: &crate::config::Config, emit_width: f64, emit_height: f64, ) -> f64`

# Calls

- [build_stamp_text](../../../../functions/src/album_globals/generator/build_stamp_text.md)
- [count_albumeasy_text_lines](../../../../functions/src/album_globals/generator/count_albumeasy_text_lines.md)
- [resolve_stamp_heading_label](../../../../functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [estimate_stamp_heading_line_count](../../../../functions/src/album_globals/generator_new/emit/estimate_stamp_heading_line_count.md)

# Called by

- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [colnect_height_uses_the_exact_emitted_text_for_every_layout](../../../../functions/src/album_globals/generator/colnect_height_uses_the_exact_emitted_text_for_every_layout.md)
- [colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags](../../../../functions/src/album_globals/generator/colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags.md)