---
type: Rust Module
title: native_pdf_page_footer
resource: tests/native_pdf_page_footer.rs#L1-L491
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
  - target: external/stamp-album-gen2-native-pdf-commands-page-album-pages-footer-pad-footerpadding-album-pages-marginse-evenmargins
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [frozen_footer_reference_identities_are_unchanged](../../functions/tests/native_pdf_page_footer/frozen_footer_reference_identities_are_unchanged.md)
- [near](../../functions/tests/native_pdf_page_footer/near.md)
- [footer_anchor_and_hint_share_layout_without_moving_glyphs](../../functions/tests/native_pdf_page_footer/footer_anchor_and_hint_share_layout_without_moving_glyphs.md)
- [footer_multiline_padding_and_parity_are_independent](../../functions/tests/native_pdf_page_footer/footer_multiline_padding_and_parity_are_independent.md)
- [missing_invalid_and_clipped_footer_values_fail_explicitly](../../functions/tests/native_pdf_page_footer/missing_invalid_and_clipped_footer_values_fail_explicitly.md)
- [footer_defaults_colour_precedence_and_snapshots_preserve_number_settings](../../functions/tests/native_pdf_page_footer/footer_defaults_colour_precedence_and_snapshots_preserve_number_settings.md)
- [footer_fonts_fit_spill_and_no_partial_output](../../functions/tests/native_pdf_page_footer/footer_fonts_fit_spill_and_no_partial_output.md)
- [footer_and_header_hints_share_usable_content_and_title_fit](../../functions/tests/native_pdf_page_footer/footer_and_header_hints_share_usable_content_and_title_fit.md)
- [trace](../../functions/tests/native_pdf_page_footer/trace.md)
- [serialized_footer_positions_colours_and_content_match_references](../../functions/tests/native_pdf_page_footer/serialized_footer_positions_colours_and_content_match_references.md)

# Imports

- `stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
}`
- `std::{error::Error, fs, path::Path}`
- `sha2::{Digest, Sha256}`
- `stamp_album_gen2::native_pdf::commands::page::{
        album_pages_footer_pad::FooterPadding, album_pages_marginse::EvenMargins,
    }`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)