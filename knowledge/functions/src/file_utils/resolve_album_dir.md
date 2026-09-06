---
type: Rust Function
title: resolve_album_dir
resource: src/file_utils.rs#L30-L32
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/normalize_album_images_for_pdf
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_album_dir(album_name: &str) -> PathBuf`

# Called by

- [resolve_album_image_path](../../../functions/src/file_utils/resolve_album_image_path.md)
- [normalize_album_images_for_pdf](../../../functions/src/image_downloader/normalize_album_images_for_pdf.md)