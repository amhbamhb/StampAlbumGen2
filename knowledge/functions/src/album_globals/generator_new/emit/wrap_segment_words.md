---
type: Rust Function
title: wrap_segment_words
resource: src/album_globals/generator_new/emit.rs#L18-L41
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator_new/emit/wrap_preserving_breaks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/wrap_preserving_breaks_keep_empty
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn wrap_segment_words(segment: &str, max_chars: usize) -> Vec<String>`

# Called by

- [wrap_preserving_breaks](../../../../../functions/src/album_globals/generator_new/emit/wrap_preserving_breaks.md)
- [wrap_preserving_breaks_keep_empty](../../../../../functions/src/album_globals/generator_new/emit/wrap_preserving_breaks_keep_empty.md)