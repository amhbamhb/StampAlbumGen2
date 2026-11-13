---
type: Rust Module
title: native_pdf_decorative_border
resource: tests/native_pdf_decorative_border.rs#L1-L824
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-colour-namedcolour-commands-page-album-pages-decorative-border-albumpagesdecorativeborder-decorativeborderpolicy-config-nativepdfconfig-document-bounds-renderer-write-page-geometry-proof-units-points-to-mm
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error-fs-path-path-pathbuf-process-command
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [root](../../functions/tests/native_pdf_decorative_border/root.md)
- [policy](../../functions/tests/native_pdf_decorative_border/policy.md)
- [near](../../functions/tests/native_pdf_decorative_border/near.md)
- [loaded](../../functions/tests/native_pdf_decorative_border/loaded.md)
- [mirrored_corner_bounds_and_fixed_slot_line_spacing](../../functions/tests/native_pdf_decorative_border/mirrored_corner_bounds_and_fixed_slot_line_spacing.md)
- [edge_only_and_mixed_layouts_match_margin_and_padding_semantics](../../functions/tests/native_pdf_decorative_border/edge_only_and_mixed_layouts_match_margin_and_padding_semantics.md)
- [explicit_policy_validation_and_definition_overrides](../../functions/tests/native_pdf_decorative_border/explicit_policy_validation_and_definition_overrides.md)
- [strict_unsupported_invalid_and_redacted_definition_errors](../../functions/tests/native_pdf_decorative_border/strict_unsupported_invalid_and_redacted_definition_errors.md)
- [files_resolve_relative_to_config_and_definition_and_are_snapshotted](../../functions/tests/native_pdf_decorative_border/files_resolve_relative_to_config_and_definition_and_are_snapshotted.md)
- [explicit_config_disabled_compatibility_and_templates](../../functions/tests/native_pdf_decorative_border/explicit_config_disabled_compatibility_and_templates.md)
- [missing_corrupt_and_non_alpha_png_assets_fail_before_output](../../functions/tests/native_pdf_decorative_border/missing_corrupt_and_non_alpha_png_assets_fail_before_output.md)
- [numeric_content_is_unchanged_and_invalid_geometry_preserves_output](../../functions/tests/native_pdf_decorative_border/numeric_content_is_unchanged_and_invalid_geometry_preserves_output.md)
- [frozen_reference_hashes_and_text](../../functions/tests/native_pdf_decorative_border/frozen_reference_hashes_and_text.md)
- [actual_decorative_pixels_and_transparency_match](../../functions/tests/native_pdf_decorative_border/actual_decorative_pixels_and_transparency_match.md)
- [trace](../../functions/tests/native_pdf_decorative_border/trace.md)
- [nums](../../functions/tests/native_pdf_decorative_border/nums.md)
- [actual_native_strokes_images_alpha_and_content_match_references](../../functions/tests/native_pdf_decorative_border/actual_native_strokes_images_alpha_and_content_match_references.md)

# Imports

- `sha2::{Digest, Sha256}`
- `stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    commands::page::album_pages_decorative_border::{
        AlbumPagesDecorativeBorder, DecorativeBorderPolicy,
    },
    config::NativePdfConfig,
    document::Bounds,
    renderer::write_page_geometry_proof,
    units::points_to_mm,
}`
- `std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)