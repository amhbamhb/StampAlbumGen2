---
type: Rust Function
title: normalize_album_images_for_pdf
resource: src/image_downloader.rs#L213-L273
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn normalize_album_images_for_pdf( album_name: &str, ) -> Result<usize, Box<dyn std::error::Error>>`

# Calls

- [resolve_album_dir](../../../functions/src/file_utils/resolve_album_dir.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)