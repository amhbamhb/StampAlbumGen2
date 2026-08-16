---
type: Rust Function
title: write_globals
resource: src/album_globals/globals.rs#L5-L15
generated:
  by: okf-rs/0.4.0
---

# Signature

`pub fn write_globals<W: std::io::Write>( writer: &mut W, album_name: &str, conn_opt: Option<&rusqlite::Connection>, start_year: Option<i32>, end_year: Option<i32>, ) -> usize`