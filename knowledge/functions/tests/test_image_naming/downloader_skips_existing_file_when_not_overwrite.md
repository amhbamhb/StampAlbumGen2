---
type: Rust Function
title: downloader_skips_existing_file_when_not_overwrite
resource: tests/test_image_naming.rs#L32-L92
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_utils/text_utils/sanitize_filename
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn downloader_skips_existing_file_when_not_overwrite()`

# Calls

- [sanitize_filename](../../../functions/src/album_utils/text_utils/sanitize_filename.md)