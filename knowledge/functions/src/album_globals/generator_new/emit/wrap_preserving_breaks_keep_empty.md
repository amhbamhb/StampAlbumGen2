---
type: Rust Function
title: wrap_preserving_breaks_keep_empty
resource: src/album_globals/generator_new/emit.rs#L57-L71
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/emit/wrap_segment_words
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/emit/build_stamp_heading_lines
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn wrap_preserving_breaks_keep_empty(text: &str, max_chars: usize) -> Vec<String>`

# Calls

- [wrap_segment_words](../../../../../functions/src/album_globals/generator_new/emit/wrap_segment_words.md)

# Called by

- [build_stamp_heading_lines](../../../../../functions/src/album_globals/generator_new/emit/build_stamp_heading_lines.md)