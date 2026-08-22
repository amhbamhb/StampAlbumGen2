---
type: Rust Module
title: image_fetcher
resource: src/colnect/image_fetcher.rs#L1-L342
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/image-genericimageview
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-debug-info-warn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/scraper-html-selector
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

- [ColnectImageFetchStats](../../../classes/src/colnect/image_fetcher/ColnectImageFetchStats.md)
- [extract_file_extension](../../../functions/src/colnect/image_fetcher/extract_file_extension.md)
- [sanitize_for_filename](../../../functions/src/colnect/image_fetcher/sanitize_for_filename.md)
- [is_likely_stamp_image](../../../functions/src/colnect/image_fetcher/is_likely_stamp_image.md)
- [resolve_url](../../../functions/src/colnect/image_fetcher/resolve_url.md)
- [extract_colnect_image_url](../../../functions/src/colnect/image_fetcher/extract_colnect_image_url.md)
- [resolve_colnect_image_url](../../../functions/src/colnect/image_fetcher/resolve_colnect_image_url.md)
- [populate_missing_images_from_colnect_pages](../../../functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages.md)

# Imports

- `image::GenericImageView`
- `log::{debug, info, warn}`
- `rusqlite::{params, Connection}`
- `scraper::{Html, Selector}`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)