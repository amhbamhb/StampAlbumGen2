---
type: Rust Module
title: utils
resource: src/commands_file_generator/utils.rs#L1-L95
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-commands-file-generator-image-utils-seriesinfo
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-file
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [month_number](../../../functions/src/commands_file_generator/utils/month_number.md)
- [is_selected_format](../../../functions/src/commands_file_generator/utils/is_selected_format.md)
- [is_selected_emission](../../../functions/src/commands_file_generator/utils/is_selected_emission.md)
- [get_catalog_num](../../../functions/src/commands_file_generator/utils/get_catalog_num.md)
- [get_series_info](../../../functions/src/commands_file_generator/utils/get_series_info.md)
- [is_valid_jpeg](../../../functions/src/commands_file_generator/utils/is_valid_jpeg.md)
- [fix_color](../../../functions/src/commands_file_generator/utils/fix_color.md)
- [snooze](../../../functions/src/commands_file_generator/utils/snooze.md)

# Imports

- `crate::commands_file_generator::image_utils::SeriesInfo`
- `std::fs::File`
- `std::io::Read`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)