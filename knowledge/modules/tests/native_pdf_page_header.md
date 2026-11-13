---
type: Rust Module
title: native_pdf_page_header
resource: tests/native_pdf_page_header.rs#L1-L484
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-native-pdf-config-nativepdfconfig-renderer-write-page-geometry-proof-units-points-to-mm
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error-fs-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-commands-page-album-pages-header-pad-headerpadding
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-commands-page-album-pages-marginse-evenmargins
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [frozen_header_reference_identities_are_unchanged](../../functions/tests/native_pdf_page_header/frozen_header_reference_identities_are_unchanged.md)
- [near](../../functions/tests/native_pdf_page_header/near.md)
- [header_uses_margin_coordinates_without_consuming_normal_content](../../functions/tests/native_pdf_page_header/header_uses_margin_coordinates_without_consuming_normal_content.md)
- [multiline_padding_and_minimum_hint_share_resolved_measurements](../../functions/tests/native_pdf_page_header/multiline_padding_and_minimum_hint_share_resolved_measurements.md)
- [parity_alignment_tracks_physical_page_and_even_margins](../../functions/tests/native_pdf_page_header/parity_alignment_tracks_physical_page_and_even_margins.md)
- [missing_invalid_and_unusable_header_values_fail_with_redacted_errors](../../functions/tests/native_pdf_page_header/missing_invalid_and_unusable_header_values_fail_with_redacted_errors.md)
- [defaults_snapshots_and_unknown_header_keys_are_explicit](../../functions/tests/native_pdf_page_header/defaults_snapshots_and_unknown_header_keys_are_explicit.md)
- [header_fit_spill_and_all_standard_fonts_reserve_without_clipping](../../functions/tests/native_pdf_page_header/header_fit_spill_and_all_standard_fonts_reserve_without_clipping.md)
- [trace](../../functions/tests/native_pdf_page_header/trace.md)
- [serialized_headers_and_title_interactions_match_references](../../functions/tests/native_pdf_page_header/serialized_headers_and_title_interactions_match_references.md)

# Imports

- `stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
}`
- `std::{error::Error, fs, path::Path}`
- `sha2::{Digest, Sha256}`
- `stamp_album_gen2::native_pdf::commands::page::album_pages_header_pad::HeaderPadding`
- `stamp_album_gen2::native_pdf::commands::page::album_pages_marginse::EvenMargins`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)