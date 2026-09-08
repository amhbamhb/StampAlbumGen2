---
type: Rust Function
title: wrap_preserving_breaks
resource: src/album_globals/generator_new/emit.rs#L45-L56
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/emit/wrap_segment_words
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn wrap_preserving_breaks(text: &str, max_chars: usize) -> Vec<String>`

# Calls

- [wrap_segment_words](../../../../../functions/src/album_globals/generator_new/emit/wrap_segment_words.md)