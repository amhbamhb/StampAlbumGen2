---
type: Rust Function
title: rich_stamp
resource: src/album_globals/generator.rs#L2836-L2867
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/set
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/colnect_height_uses_the_exact_emitted_text_for_every_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn rich_stamp() -> Stamp`

# Calls

- [set](../../../../functions/src/album_globals/structs/GenericAttrs/set.md)

# Called by

- [colnect_height_uses_the_exact_emitted_text_for_every_layout](../../../../functions/src/album_globals/generator/colnect_height_uses_the_exact_emitted_text_for_every_layout.md)
- [colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags](../../../../functions/src/album_globals/generator/colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags.md)