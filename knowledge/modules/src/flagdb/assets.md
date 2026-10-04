---
type: Rust Module
title: assets
resource: src/flagdb/assets.rs#L1-L429
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-countries-fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-io-write-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-header-headermap-statuscode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-mutex
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [flag_directories](../../../functions/src/flagdb/assets/flag_directories.md)
- [validate](../../../functions/src/flagdb/assets/validate.md)
- [valid_file](../../../functions/src/flagdb/assets/valid_file.md)
- [write_atomic](../../../functions/src/flagdb/assets/write_atomic.md)
- [rasterize](../../../functions/src/flagdb/assets/rasterize.md)
- [ensure_flag](../../../functions/src/flagdb/assets/ensure_flag.md)
- [local_album_flag](../../../functions/src/flagdb/assets/local_album_flag.md)
- [is_missing](../../../functions/src/flagdb/assets/is_missing.md)
- [copy_to_album](../../../functions/src/flagdb/assets/copy_to_album.md)
- [FakeFetcher](../../../classes/src/flagdb/assets/FakeFetcher.md)
- [new](../../../functions/src/flagdb/assets/FakeFetcher/new.md)
- [requests](../../../functions/src/flagdb/assets/FakeFetcher/requests.md)
- [fetch_text](../../../functions/src/flagdb/assets/FakeFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../../functions/src/flagdb/assets/FakeFetcher/fetcher/fetch_bytes.md)
- [png](../../../functions/src/flagdb/assets/png.md)
- [failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors](../../../functions/src/flagdb/assets/failed_http_and_invalid_images_continue_to_flagcdn_without_caching_errors.md)
- [valid_svg_is_rendered_to_a_real_png_at_the_requested_size](../../../functions/src/flagdb/assets/valid_svg_is_rendered_to_a_real_png_at_the_requested_size.md)
- [valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes](../../../functions/src/flagdb/assets/valid_cache_skips_network_corrupt_cache_is_repaired_and_overwrite_refreshes.md)
- [failed_refresh_preserves_good_cache_and_no_error_body_is_written](../../../functions/src/flagdb/assets/failed_refresh_preserves_good_cache_and_no_error_body_is_written.md)
- [cached_svg_is_supported_and_invalid_inputs_do_not_access_network](../../../functions/src/flagdb/assets/cached_svg_is_supported_and_invalid_inputs_do_not_access_network.md)
- [album_local_fallback_checks_real_images_and_preserves_svg_extension](../../../functions/src/flagdb/assets/album_local_fallback_checks_real_images_and_preserves_svg_extension.md)

# Imports

- `super::{countries, Fetcher}`
- `std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
}`
- `super::*`
- `async_trait::async_trait`
- `reqwest::{header::HeaderMap, StatusCode}`
- `std::sync::Mutex`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)