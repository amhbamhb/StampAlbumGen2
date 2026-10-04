---
type: Rust Function
title: generate_album_output_conn
resource: src/album_globals/generator_new/mod.rs#L17-L58
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_dir
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_album_output_conn<F>( _conn: &Connection, _album_name: &str, _page_callback: Option<F>, _start_year: Option<i32>, _end_year: Option<i32>, ) -> std::path::PathBuf where F: FnMut(usize),`

# Calls

- [resolve_album_dir](../../../../functions/src/file_utils/resolve_album_dir.md)