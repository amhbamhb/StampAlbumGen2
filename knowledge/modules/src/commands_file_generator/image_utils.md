---
type: Rust Module
title: image_utils
resource: src/commands_file_generator/image_utils.rs#L1-L168
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-types-imagestatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-debug-error
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-blocking-client
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-self
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-namedtempfile
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [SeriesInfo](../../../classes/src/commands_file_generator/image_utils/SeriesInfo.md)
- [download_to_path](../../../functions/src/commands_file_generator/image_utils/download_to_path.md)
- [get_colnect_image](../../../functions/src/commands_file_generator/image_utils/get_colnect_image.md)
- [get_image](../../../functions/src/commands_file_generator/image_utils/get_image.md)
- [get_image_blocking](../../../functions/src/commands_file_generator/image_utils/get_image_blocking.md)

# Imports

- `crate::types::ImageStatus`
- `log::{debug, error}`
- `reqwest::blocking::Client`
- `std::io::{self}`
- `std::time::Duration`
- `tempfile::NamedTempFile`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)