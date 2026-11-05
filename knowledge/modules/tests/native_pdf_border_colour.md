---
type: Rust Module
title: native_pdf_border_colour
resource: tests/native_pdf_border_colour.rs#L1-L337
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-commands-colour-colour-album-border-albumbordercolour-config-nativepdfconfig-renderer-units-points-to-mm
    resolved_by: tree-sitter
    confidence: exact
  - target: external/proof-write-page-geometry-proof
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error-fs-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [root](../../functions/tests/native_pdf_border_colour/root.md)
- [manifest](../../functions/tests/native_pdf_border_colour/manifest.md)
- [source_colour](../../functions/tests/native_pdf_border_colour/source_colour.md)
- [near](../../functions/tests/native_pdf_border_colour/near.md)
- [named_colours_defaults_aliases_and_frozen_identities_match](../../functions/tests/native_pdf_border_colour/named_colours_defaults_aliases_and_frozen_identities_match.md)
- [colour_validation_rejects_unsupported_forms_without_echoing_values](../../functions/tests/native_pdf_border_colour/colour_validation_rejects_unsupported_forms_without_echoing_values.md)
- [config_colour_is_applied_with_section_precedence_and_no_geometry_changes](../../functions/tests/native_pdf_border_colour/config_colour_is_applied_with_section_precedence_and_no_geometry_changes.md)
- [trace](../../functions/tests/native_pdf_border_colour/trace.md)
- [rendered_colours_match_reference_and_guide_stays_black](../../functions/tests/native_pdf_border_colour/rendered_colours_match_reference_and_guide_stays_black.md)

# Imports

- `serde_json::Value`
- `sha2::{Digest, Sha256}`
- `stamp_album_gen2::native_pdf::{
    commands::colour::colour_album_border::AlbumBorderColour, config::NativePdfConfig, renderer,
    units::points_to_mm,
}`
- `proof::write_page_geometry_proof`
- `std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)