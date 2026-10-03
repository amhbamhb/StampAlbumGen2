---
type: Rust Function
title: strip_bracket_tags
resource: src/album_globals/generator.rs#L113-L130
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/normalize_for_text_dedupe
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/build_stamp_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn strip_bracket_tags(input: &str) -> String`

# Called by

- [normalize_for_text_dedupe](../../../../functions/src/album_globals/generator/normalize_for_text_dedupe.md)
- [build_stamp_text](../../../../functions/src/album_globals/generator/build_stamp_text.md)