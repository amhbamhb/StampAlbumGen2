---
type: Rust Module
title: native_pdf_footer_date
resource: tests/native_pdf_footer_date.rs#L1-L919
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
  - target: external/stamp-album-gen2-native-pdf-colour-namedcolour-commands-page-album-pages-footer-albumpagesfooter-album-pages-footer-date-albumpagesfooterdate-album-pages-footer-pad-footerpadding-date-dateformat-datesource-document-bounds-text-textposition
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

- [trace](../../functions/tests/native_pdf_footer_date/trace.md)
- [serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry](../../functions/tests/native_pdf_footer_date/serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry.md)
- [header_and_footer_share_one_generation_date_with_independent_fixed_overrides](../../functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides.md)
- [defaults_enablement_snapshots_and_colour_precedence_are_explicit](../../functions/tests/native_pdf_footer_date/defaults_enablement_snapshots_and_colour_precedence_are_explicit.md)
- [missing_invalid_dates_and_fragments_report_footer_keys_without_values](../../functions/tests/native_pdf_footer_date/missing_invalid_dates_and_fragments_report_footer_keys_without_values.md)
- [measured_footer_hints_parity_and_multiline_do_not_move_glyphs](../../functions/tests/native_pdf_footer_date/measured_footer_hints_parity_and_multiline_do_not_move_glyphs.md)
- [footer_date_text_and_number_collisions_are_actionable_and_page_specific](../../functions/tests/native_pdf_footer_date/footer_date_text_and_number_collisions_are_actionable_and_page_specific.md)
- [late_footer_date_failure_preserves_existing_output_and_title_clearance](../../functions/tests/native_pdf_footer_date/late_footer_date_failure_preserves_existing_output_and_title_clearance.md)
- [all_font_variants_validate_exact_width_and_keyed_generation_range](../../functions/tests/native_pdf_footer_date/all_font_variants_validate_exact_width_and_keyed_generation_range.md)
- [frozen_footer_date_reference_identities_are_unchanged](../../functions/tests/native_pdf_footer_date/frozen_footer_date_reference_identities_are_unchanged.md)
- [fixed_footer_date_uses_formats_and_verified_bottom_geometry](../../functions/tests/native_pdf_footer_date/fixed_footer_date_uses_formats_and_verified_bottom_geometry.md)

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
            album_pages_footer::AlbumPagesFooter, album_pages_footer_date::AlbumPagesFooterDate,
            album_pages_footer_pad::FooterPadding,
        },
        date::{DateFormat, DateSource},
        document::Bounds,
        text::TextPosition,
    }`
- `sha2::{Digest, Sha256}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)