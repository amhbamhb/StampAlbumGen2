---
type: Rust Function
title: emit_stamp_lines
resource: src/album_globals/generator_new/emit.rs#L175-L231
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/emit/escape_ae_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_stamp_lines<W: Write>( out: &mut W, ctx: &EmitContext, config: &crate::config::Config, is_image: bool, ) -> io::Result<()>`

# Calls

- [escape_ae_text](../../../../../functions/src/album_globals/generator_new/emit/escape_ae_text.md)
- [resolve_stamp_heading_label](../../../../../functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label.md)