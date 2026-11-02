---
type: Rust Function
title: emit_stamp_diag
resource: src/album_globals/generator_new/helpers.rs#L86-L98
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_stamp_diag<W: Write>(output: &mut W, diag: &StampDiag<'_>) -> std::io::Result<()>`

# Called by

- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)