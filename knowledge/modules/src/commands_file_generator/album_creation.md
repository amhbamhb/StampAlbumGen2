---
type: Rust Module
title: album_creation
resource: src/commands_file_generator/album_creation.rs#L1-L327
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-album-globals
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-commands-file-generator-image-utils-seriesinfo
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-commands-file-generator-utils-fix-color-get-catalog-num-is-selected-emission-is-selected-format
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-error-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-file
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-bufwriter-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-self-file
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-os-unix-fs-permissionsext
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [create_album](../../../functions/src/commands_file_generator/album_creation/create_album.md)
- [write_globals](../../../functions/src/commands_file_generator/album_creation/write_globals.md)
- [save_stamp_data](../../../functions/src/commands_file_generator/album_creation/save_stamp_data.md)
- [get_series_info_stub](../../../functions/src/commands_file_generator/album_creation/get_series_info_stub.md)

# Imports

- `crate::album_globals`
- `crate::commands_file_generator::image_utils::SeriesInfo`
- `crate::commands_file_generator::utils::{
    fix_color, get_catalog_num, is_selected_emission, is_selected_format,
}`
- `log::{error, info}`
- `std::io::Write`
- `std::fs::File`
- `std::io::{BufWriter, Write}`
- `std::fs::{self, File}`
- `std::os::unix::fs::PermissionsExt`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)