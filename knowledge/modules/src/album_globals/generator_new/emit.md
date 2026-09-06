---
type: Rust Module
title: emit
resource: src/album_globals/generator_new/emit.rs#L1-L389
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/log-debug
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-self-write
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [escape_ae_text](../../../../functions/src/album_globals/generator_new/emit/escape_ae_text.md)
- [heading_max_chars_per_line](../../../../functions/src/album_globals/generator_new/emit/heading_max_chars_per_line.md)
- [wrap_segment_words](../../../../functions/src/album_globals/generator_new/emit/wrap_segment_words.md)
- [wrap_preserving_breaks](../../../../functions/src/album_globals/generator_new/emit/wrap_preserving_breaks.md)
- [wrap_preserving_breaks_keep_empty](../../../../functions/src/album_globals/generator_new/emit/wrap_preserving_breaks_keep_empty.md)
- [EmitContext](../../../../classes/src/album_globals/generator_new/emit/EmitContext.md)
- [build_stamp_heading_lines](../../../../functions/src/album_globals/generator_new/emit/build_stamp_heading_lines.md)
- [format_stamp_heading](../../../../functions/src/album_globals/generator_new/emit/format_stamp_heading.md)
- [estimate_stamp_heading_line_count](../../../../functions/src/album_globals/generator_new/emit/estimate_stamp_heading_line_count.md)
- [emit_stamp_lines](../../../../functions/src/album_globals/generator_new/emit/emit_stamp_lines.md)
- [resolve_stamp_heading_label](../../../../functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label.md)
- [emit_stamp](../../../../functions/src/album_globals/generator_new/emit/emit_stamp.md)
- [emit_image_only](../../../../functions/src/album_globals/generator_new/emit/emit_image_only.md)

# Imports

- `log::debug`
- `std::io::{self, Write}`

# Member of

- [stamp_album_gen2](../../../../packages/stamp_album_gen2.md)