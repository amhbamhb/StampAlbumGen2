---
type: Rust Function
title: emit_stamp
resource: src/album_globals/generator_new/emit.rs#L298-L361
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/emit/escape_ae_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/format_stamp_heading
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_stamp<W: Write>( out: &mut W, has_image: bool, ctx: &EmitContext, ) -> std::io::Result<()>`

# Calls

- [escape_ae_text](../../../../../functions/src/album_globals/generator_new/emit/escape_ae_text.md)
- [format_stamp_heading](../../../../../functions/src/album_globals/generator_new/emit/format_stamp_heading.md)

# Called by

- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)