---
type: Rust Function
title: generate_album_output
resource: src/album_globals/generator.rs#L2800-L2812
generated:
  by: okf-rs/0.4.0
---

# Signature

`pub async fn generate_album_output<F>( db_path: &str, album_name: &str, page_callback: Option<F>, start_year: Option<i32>, end_year: Option<i32>, ) -> std::path::PathBuf where F: FnMut(usize),`