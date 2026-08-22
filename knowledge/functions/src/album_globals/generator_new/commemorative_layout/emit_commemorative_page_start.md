---
type: Rust Function
title: emit_commemorative_page_start
resource: src/album_globals/generator_new/commemorative_layout.rs#L44-L58
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_commemorative_page_start<W: Write>( out: &mut W, _page_number: usize, _title: &str, ) -> std::io::Result<()>`

# Called by

- [emit_commemorative_page](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page.md)