---
type: Rust Module
title: native_pdf_footer_number
resource: tests/native_pdf_footer_number.rs#L1-L637
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
  - target: external/stamp-album-gen2-native-pdf-commands-page-album-pages-marginse-evenmargins-renderer-write-page-geometry-proof
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-colour-namedcolour-commands-page-album-pages-footer-albumpagesfooter-album-pages-footer-number-albumpagesfooternumber-album-pages-footer-pad-footerpadding-document-bounds-text-textposition
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

- [numbering_requires_known_total_and_uses_printed_start_not_parity](../../functions/tests/native_pdf_footer_number/numbering_requires_known_total_and_uses_printed_start_not_parity.md)
- [signed_starts_tokens_and_hints_do_not_change_number_glyphs](../../functions/tests/native_pdf_footer_number/signed_starts_tokens_and_hints_do_not_change_number_glyphs.md)
- [numbering_defaults_enablement_precedence_and_snapshots_are_explicit](../../functions/tests/native_pdf_footer_number/numbering_defaults_enablement_precedence_and_snapshots_are_explicit.md)
- [invalid_missing_and_overflow_values_fail_with_redacted_keys](../../functions/tests/native_pdf_footer_number/invalid_missing_and_overflow_values_fail_with_redacted_keys.md)
- [coexistence_collisions_and_later_page_preflight_are_checked](../../functions/tests/native_pdf_footer_number/coexistence_collisions_and_later_page_preflight_are_checked.md)
- [frozen_number_reference_identities_are_unchanged](../../functions/tests/native_pdf_footer_number/frozen_number_reference_identities_are_unchanged.md)
- [changing_digit_width_and_font_metrics_are_validated_at_final_count](../../functions/tests/native_pdf_footer_number/changing_digit_width_and_font_metrics_are_validated_at_final_count.md)
- [trace](../../functions/tests/native_pdf_footer_number/trace.md)
- [serialized_numbers_tokens_and_footer_coexistence_match_references](../../functions/tests/native_pdf_footer_number/serialized_numbers_tokens_and_footer_coexistence_match_references.md)

# Imports

- `stamp_album_gen2::native_pdf::config::NativePdfConfig`
- `std::{error::Error, fs, path::Path}`
- `stamp_album_gen2::native_pdf::{
        commands::page::album_pages_marginse::EvenMargins, renderer::write_page_geometry_proof,
    }`
- `sha2::{Digest, Sha256}`
- `stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::page::{
            album_pages_footer::AlbumPagesFooter,
            album_pages_footer_number::AlbumPagesFooterNumber,
            album_pages_footer_pad::FooterPadding,
        },
        document::Bounds,
        text::TextPosition,
    }`
- `stamp_album_gen2::native_pdf::{renderer::write_page_geometry_proof, units::points_to_mm}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)