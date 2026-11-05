---
type: Rust Module
title: native_pdf_explicit_settings
resource: tests/native_pdf_explicit_settings.rs#L1-L370
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
  - target: external/stamp-album-gen2-native-pdf-commands-page-album-pages-border-borderline
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [toml_defaults_and_independent_border_colours_are_explicit](../../functions/tests/native_pdf_explicit_settings/toml_defaults_and_independent_border_colours_are_explicit.md)
- [independent_colours_reach_pdf_for_full_and_sparse_borders](../../functions/tests/native_pdf_explicit_settings/independent_colours_reach_pdf_for_full_and_sparse_borders.md)
- [rendering_policy_has_no_missing_key_fallbacks](../../functions/tests/native_pdf_explicit_settings/rendering_policy_has_no_missing_key_fallbacks.md)
- [proof_options_are_validated_without_output_or_secret_echoes](../../functions/tests/native_pdf_explicit_settings/proof_options_are_validated_without_output_or_secret_echoes.md)
- [all_proof_policy_controls_actual_output](../../functions/tests/native_pdf_explicit_settings/all_proof_policy_controls_actual_output.md)

# Imports

- `stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
}`
- `std::{error::Error, fs}`
- `stamp_album_gen2::native_pdf::commands::page::album_pages_border::BorderLine`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)