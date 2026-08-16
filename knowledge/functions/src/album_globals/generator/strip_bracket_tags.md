---
type: Rust Function
title: strip_bracket_tags
resource: src/album_globals/generator.rs#L110-L127
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/normalize_for_text_dedupe
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn strip_bracket_tags(input: &str) -> String`

# Called by

- [normalize_for_text_dedupe](../../../../functions/src/album_globals/generator/normalize_for_text_dedupe.md)
- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)