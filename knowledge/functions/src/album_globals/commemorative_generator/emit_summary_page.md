---
type: Rust Function
title: emit_summary_page
resource: src/album_globals/commemorative_generator.rs#L343-L398
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_summary_page<W: Write>( out: &mut W, year: i32, stamps: &[Stamp], ) -> std::io::Result<()>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)