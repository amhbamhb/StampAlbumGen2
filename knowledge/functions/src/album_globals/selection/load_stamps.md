---
type: Rust Function
title: load_stamps
resource: src/album_globals/selection.rs#L62-L73
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_for_selection
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/selection/colnect_pdf_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/prepare_pdf_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_stamps( conn: &Connection, start_year: Option<i32>, end_year: Option<i32>, ) -> rusqlite::Result<PreparedPdfStamps>`

# Calls

- [fetch_and_filter_stamps_for_selection](../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_for_selection.md)

# Called by

- [colnect_pdf_options](../../../../functions/src/album_globals/selection/colnect_pdf_options.md)
- [prepare_pdf_stamps](../../../../functions/src/album_globals/selection/prepare_pdf_stamps.md)