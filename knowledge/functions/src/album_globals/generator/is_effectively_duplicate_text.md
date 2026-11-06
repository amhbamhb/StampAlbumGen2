---
type: Rust Function
title: is_effectively_duplicate_text
resource: src/album_globals/generator.rs#L153-L167
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator/normalize_for_text_dedupe
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/build_stamp_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_effectively_duplicate_text(base_text: &str, candidate_text: &str) -> bool`

# Calls

- [normalize_for_text_dedupe](../../../../functions/src/album_globals/generator/normalize_for_text_dedupe.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [build_stamp_text](../../../../functions/src/album_globals/generator/build_stamp_text.md)