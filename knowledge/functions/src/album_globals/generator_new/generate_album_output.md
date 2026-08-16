---
type: Rust Function
title: generate_album_output
resource: src/album_globals/generator_new/mod.rs#L84-L100
generated:
  by: okf-rs/0.4.0
---

# Signature

`pub fn generate_album_output<F>( _db_path: &str, _album_name: &str, _page_callback: Option<F>, _start_year: Option<i32>, _end_year: Option<i32>, ) -> std::path::PathBuf where F: FnMut(usize),`