---
type: Rust Module
title: test_image_naming
resource: tests/test_image_naming.rs#L1-L92
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-album-utils-sanitize-filename
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-imagestatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [deterministic_target_name_is_sanitized](../../functions/tests/test_image_naming/deterministic_target_name_is_sanitized.md)
- [downloader_skips_existing_file_when_not_overwrite](../../functions/tests/test_image_naming/downloader_skips_existing_file_when_not_overwrite.md)

# Imports

- `stamp_album_gen2::album_utils::sanitize_filename`
- `stamp_album_gen2::ImageStatus`
- `std::path::Path`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)