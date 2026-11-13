---
type: Rust Module
title: native_pdf_page_title
resource: tests/native_pdf_page_title.rs#L1-L463
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-native-pdf-config-nativepdfconfig-renderer-write-page-geometry-proof-units-points-to-mm
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-colour-namedcolour-commands-page-album-pages-title-albumpagestitle-document-bounds
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [frozen_title_reference_identities_are_unchanged](../../functions/tests/native_pdf_page_title/frozen_title_reference_identities_are_unchanged.md)
- [title_measures_once_and_reserves_font_dependent_height](../../functions/tests/native_pdf_page_title/title_measures_once_and_reserves_font_dependent_height.md)
- [title_fit_boundaries_and_hint_preserve_measured_text](../../functions/tests/native_pdf_page_title/title_fit_boundaries_and_hint_preserve_measured_text.md)
- [title_precedence_snapshot_and_optional_extension_are_explicit](../../functions/tests/native_pdf_page_title/title_precedence_snapshot_and_optional_extension_are_explicit.md)
- [minimum_hint_only_expands_reservation_and_never_resizes_text](../../functions/tests/native_pdf_page_title/minimum_hint_only_expands_reservation_and_never_resizes_text.md)
- [invalid_title_policy_fails_with_keyed_redacted_errors](../../functions/tests/native_pdf_page_title/invalid_title_policy_fails_with_keyed_redacted_errors.md)
- [title_pdf_positions_match_frozen_references_and_hint_controls_guide](../../functions/tests/native_pdf_page_title/title_pdf_positions_match_frozen_references_and_hint_controls_guide.md)

# Imports

- `stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
}`
- `std::{error::Error, fs}`
- `sha2::{Digest, Sha256}`
- `stamp_album_gen2::native_pdf::{
        colour::NamedColour, commands::page::album_pages_title::AlbumPagesTitle, document::Bounds,
    }`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)