---
type: Rust Function
title: generate_album_output_conn
resource: src/album_globals/mod.rs#L274-L294
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generate_album_output_conn_with_pdf_override
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn generate_album_output_conn<F>( conn: &Connection, album_name: &str, page_callback: Option<F>, start_year: Option<i32>, end_year: Option<i32>, ) -> std::path::PathBuf where F: FnMut(usize),`

# Calls

- [generate_album_output_conn_with_pdf_override](../../../functions/src/album_globals/generate_album_output_conn_with_pdf_override.md)