---
type: Rust Module
title: image_downloader
resource: src/image_downloader.rs#L1-L273
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/log-debug-info-warn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [download_image](../../functions/src/image_downloader/download_image.md)
- [download_stamp_image_if_missing](../../functions/src/image_downloader/download_stamp_image_if_missing.md)
- [download_missing_stamp_images](../../functions/src/image_downloader/download_missing_stamp_images.md)
- [normalize_album_images_for_pdf](../../functions/src/image_downloader/normalize_album_images_for_pdf.md)

# Imports

- `log::{debug, info, warn}`
- `reqwest`
- `std::fs`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)