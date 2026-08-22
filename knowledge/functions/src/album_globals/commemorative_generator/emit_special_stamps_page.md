---
type: Rust Function
title: emit_special_stamps_page
resource: src/album_globals/commemorative_generator.rs#L247-L340
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_special_stamps_page<W: Write>(out: &mut W, stamps: &[Stamp]) -> std::io::Result<()>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)