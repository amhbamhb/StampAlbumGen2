---
type: Rust Function
title: emit_stamp_diag
resource: src/album_globals/generator_new/helpers.rs#L66-L87
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_stamp_diag<W: Write>( output: &mut W, sg: &str, source: &str, est_w_mm: f64, est_h_mm: f64, px_w: i32, px_h: i32, img: &str, ) -> std::io::Result<()>`

# Called by

- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)