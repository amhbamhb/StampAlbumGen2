---
type: Rust Module
title: native_pdf_header_date
resource: tests/native_pdf_header_date.rs#L1-L816
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
  - target: external/stamp-album-gen2-native-pdf-renderer-write-page-geometry-proof-units-points-to-mm
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-renderer-write-page-geometry-proof-write-page-geometry-proof-with-date
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-commands-page-album-pages-marginse-evenmargins-renderer-write-page-geometry-proof
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-colour-namedcolour-commands-page-album-pages-header-albumpagesheader-album-pages-header-date-albumpagesheaderdate-album-pages-header-pad-headerpadding-date-dateformat-datesource-document-bounds-text-textposition
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [trace](../../functions/tests/native_pdf_header_date/trace.md)
- [serialized_dates_and_coexistence_match_frozen_reference_pdfs](../../functions/tests/native_pdf_header_date/serialized_dates_and_coexistence_match_frozen_reference_pdfs.md)
- [generation_date_requires_one_explicit_document_context](../../functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context.md)
- [defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit](../../functions/tests/native_pdf_header_date/defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit.md)
- [invalid_missing_date_fields_return_redacted_keyed_errors](../../functions/tests/native_pdf_header_date/invalid_missing_date_fields_return_redacted_keyed_errors.md)
- [date_hints_parity_and_multiline_reuse_measured_header_layout](../../functions/tests/native_pdf_header_date/date_hints_parity_and_multiline_reuse_measured_header_layout.md)
- [date_collisions_with_text_and_numbers_identify_correction_keys](../../functions/tests/native_pdf_header_date/date_collisions_with_text_and_numbers_identify_correction_keys.md)
- [title_separation_and_invalid_later_page_preserve_output](../../functions/tests/native_pdf_header_date/title_separation_and_invalid_later_page_preserve_output.md)
- [all_font_variants_validate_exact_date_width_and_year_range](../../functions/tests/native_pdf_header_date/all_font_variants_validate_exact_date_width_and_year_range.md)
- [frozen_reference_date_and_hashes_are_unchanged](../../functions/tests/native_pdf_header_date/frozen_reference_date_and_hashes_are_unchanged.md)
- [fixed_date_uses_explicit_formats_and_shared_header_geometry](../../functions/tests/native_pdf_header_date/fixed_date_uses_explicit_formats_and_shared_header_geometry.md)

# Imports

- `stamp_album_gen2::native_pdf::config::NativePdfConfig`
- `std::{error::Error, fs, path::Path}`
- `stamp_album_gen2::native_pdf::{renderer::write_page_geometry_proof, units::points_to_mm}`
- `stamp_album_gen2::native_pdf::renderer::{
        write_page_geometry_proof, write_page_geometry_proof_with_date,
    }`
- `stamp_album_gen2::native_pdf::{
        commands::page::album_pages_marginse::EvenMargins, renderer::write_page_geometry_proof,
    }`
- `stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::page::{
            album_pages_header::AlbumPagesHeader, album_pages_header_date::AlbumPagesHeaderDate,
            album_pages_header_pad::HeaderPadding,
        },
        date::{DateFormat, DateSource},
        document::Bounds,
        text::TextPosition,
    }`
- `sha2::{Digest, Sha256}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)