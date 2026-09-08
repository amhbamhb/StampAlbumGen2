---
type: Rust Function
title: emit_image_only
resource: src/album_globals/generator_new/emit.rs#L366-L389
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/emit/escape_ae_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_image_only<W: Write>( out: &mut W, ctx: &EmitContext, options: Option<&str>, ) -> std::io::Result<()>`

# Calls

- [escape_ae_text](../../../../../functions/src/album_globals/generator_new/emit/escape_ae_text.md)

# Called by

- [write_globals](../../../../../functions/src/album_utils/globals/write_globals.md)