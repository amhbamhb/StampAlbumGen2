---
type: Rust Function
title: build_stamp_description
resource: src/album_globals/generator_new/commemorative_layout.rs#L171-L209
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/clean_for_output
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_description_boxes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_stamp_description(stamp: &Stamp) -> String`

# Calls

- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [clean_for_output](../../../../../functions/src/album_globals/generator_new/commemorative_layout/clean_for_output.md)

# Called by

- [emit_description_boxes](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_description_boxes.md)