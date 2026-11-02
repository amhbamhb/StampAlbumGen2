---
type: Rust Function
title: normalize_for_text_dedupe
resource: src/album_globals/generator.rs#L133-L151
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator/strip_bracket_tags
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/is_effectively_duplicate_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn normalize_for_text_dedupe(input: &str) -> String`

# Calls

- [strip_bracket_tags](../../../../functions/src/album_globals/generator/strip_bracket_tags.md)

# Called by

- [is_effectively_duplicate_text](../../../../functions/src/album_globals/generator/is_effectively_duplicate_text.md)