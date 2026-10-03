---
type: Rust Function
title: create_album
resource: src/commands_file_generator/album_creation.rs#L26-L193
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/commands_file_generator/utils/get_catalog_num
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/new_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/new_year
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/album_creation/get_series_info_stub
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/new_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/utils/fix_color
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/add_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn create_album<'a>(params: CreateAlbumParams<'a>) -> usize`

# Calls

- [get_catalog_num](../../../../functions/src/commands_file_generator/utils/get_catalog_num.md)
- [new_page](../../../../functions/src/album_globals/page_functions/new_page.md)
- [new_year](../../../../functions/src/album_globals/page_functions/new_year.md)
- [get_series_info_stub](../../../../functions/src/commands_file_generator/album_creation/get_series_info_stub.md)
- [new_series](../../../../functions/src/album_globals/page_functions/new_series.md)
- [fix_color](../../../../functions/src/commands_file_generator/utils/fix_color.md)
- [add_stamp](../../../../functions/src/album_globals/page_functions/add_stamp.md)