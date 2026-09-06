---
type: Rust Function
title: get_series_info_stub
resource: src/commands_file_generator/album_creation.rs#L320-L332
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/commands_file_generator/album_creation/create_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_series_info_stub( _data: &[String], _idx: usize, width: &str, height: &str, ) -> SeriesInfo`

# Called by

- [create_album](../../../../functions/src/commands_file_generator/album_creation/create_album.md)