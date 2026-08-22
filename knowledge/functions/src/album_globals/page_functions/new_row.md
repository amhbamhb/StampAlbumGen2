---
type: Rust Function
title: new_row
resource: src/album_globals/page_functions.rs#L84-L126
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/globals/compute_usable_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/new_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/new_year
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn new_row<W: std::io::Write>( writer: &mut W, year: &str, rows_written: &mut i32, stamps_written: &mut i32, used_width: &mut i32, used_height: &mut i32, max_row_height: &mut i32, )`

# Calls

- [load_effective_config](../../../../functions/src/config/load_effective_config.md)
- [compute_usable_height](../../../../functions/src/album_globals/globals/compute_usable_height.md)
- [new_page](../../../../functions/src/album_globals/page_functions/new_page.md)
- [new_year](../../../../functions/src/album_globals/page_functions/new_year.md)