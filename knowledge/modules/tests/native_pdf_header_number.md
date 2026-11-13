---
type: Rust Module
title: native_pdf_header_number
resource: tests/native_pdf_header_number.rs#L1-L709
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-native-pdf-config-nativepdfconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error-fs-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-colour-namedcolour-commands-page-album-pages-header-albumpagesheader-album-pages-header-pad-headerpadding-renderer-write-page-geometry-proof-text-textposition
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-colour-namedcolour-commands-page-album-pages-header-albumpagesheader-album-pages-header-number-albumpagesheadernumber-album-pages-header-pad-headerpadding-document-bounds-text-textposition
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-renderer-write-page-geometry-proof-units-points-to-mm
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [header_numbers_use_explicit_count_and_independent_style](../../functions/tests/native_pdf_header_number/header_numbers_use_explicit_count_and_independent_style.md)
- [overlapping_header_values_report_page_and_correction_keys](../../functions/tests/native_pdf_header_number/overlapping_header_values_report_page_and_correction_keys.md)
- [defaults_missing_invalid_values_and_snapshot_are_explicit](../../functions/tests/native_pdf_header_number/defaults_missing_invalid_values_and_snapshot_are_explicit.md)
- [signed_tokens_parity_multiline_and_shared_hints_are_measured](../../functions/tests/native_pdf_header_number/signed_tokens_parity_multiline_and_shared_hints_are_measured.md)
- [late_width_collision_preflight_preserves_existing_output](../../functions/tests/native_pdf_header_number/late_width_collision_preflight_preserves_existing_output.md)
- [wide_and_tall_titles_remain_below_both_header_elements](../../functions/tests/native_pdf_header_number/wide_and_tall_titles_remain_below_both_header_elements.md)
- [all_fonts_remeasure_changing_digit_width_and_reject_overflow](../../functions/tests/native_pdf_header_number/all_fonts_remeasure_changing_digit_width_and_reject_overflow.md)
- [frozen_header_number_reference_identities_are_unchanged](../../functions/tests/native_pdf_header_number/frozen_header_number_reference_identities_are_unchanged.md)
- [trace](../../functions/tests/native_pdf_header_number/trace.md)
- [serialized_header_numbers_match_reference_fonts_glyphs_and_geometry](../../functions/tests/native_pdf_header_number/serialized_header_numbers_match_reference_fonts_glyphs_and_geometry.md)

# Imports

- `stamp_album_gen2::native_pdf::config::NativePdfConfig`
- `std::{error::Error, fs, path::Path}`
- `stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::page::{
            album_pages_header::AlbumPagesHeader, album_pages_header_pad::HeaderPadding,
        },
        renderer::write_page_geometry_proof,
        text::TextPosition,
    }`
- `stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::page::{
            album_pages_header::AlbumPagesHeader,
            album_pages_header_number::AlbumPagesHeaderNumber,
            album_pages_header_pad::HeaderPadding,
        },
        document::Bounds,
        text::TextPosition,
    }`
- `sha2::{Digest, Sha256}`
- `stamp_album_gen2::native_pdf::{renderer::write_page_geometry_proof, units::points_to_mm}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)