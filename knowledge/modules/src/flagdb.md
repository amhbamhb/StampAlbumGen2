---
type: Rust Module
title: flagdb
resource: src/flagdb.rs#L1-L761
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-stampworld-fetch-fetcher-reqwestfetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/image-genericimageview
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
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

- [default_flag_map](../../functions/src/flagdb/default_flag_map.md)
- [load_flag_map_from_fs](../../functions/src/flagdb/load_flag_map_from_fs.md)
- [load_embedded_flag_map_if_enabled](../../functions/src/flagdb/load_embedded_flag_map_if_enabled.md)
- [load_embedded_flag_map_if_enabled](../../functions/src/flagdb/load_embedded_flag_map_if_enabled-2.md)
- [country_to_code](../../functions/src/flagdb/country_to_code.md)
- [normalize_country_input](../../functions/src/flagdb/normalize_country_input.md)
- [ensure_flag_image](../../functions/src/flagdb/ensure_flag_image.md)
- [ensure_flag_image_with_fetcher](../../functions/src/flagdb/ensure_flag_image_with_fetcher.md)
- [copy_flag_for_album](../../functions/src/flagdb/copy_flag_for_album.md)
- [ensure_coat_of_arms](../../functions/src/flagdb/ensure_coat_of_arms.md)
- [ensure_coat_of_arms_with_fetcher](../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [copy_coa_for_album](../../functions/src/flagdb/copy_coa_for_album.md)
- [try_rasterize_svg_to_png](../../functions/src/flagdb/try_rasterize_svg_to_png.md)
- [test_try_rasterize_svg_to_png_roundtrip](../../functions/src/flagdb/test_try_rasterize_svg_to_png_roundtrip.md)

# Imports

- `std::collections::HashMap`
- `std::fs`
- `std::io::Write`
- `std::path::{Path, PathBuf}`
- `crate::stampworld::fetch::{Fetcher, ReqwestFetcher}`
- `image::GenericImageView`
- `std::process::Command`
- `super::*`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)