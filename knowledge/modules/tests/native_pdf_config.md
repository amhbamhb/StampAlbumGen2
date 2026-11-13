---
type: Rust Module
title: native_pdf_config
resource: tests/native_pdf_config.rs#L1-L338
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/oxidize-pdf-parser-pdfdocument
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-config-nativepdfconfig-renderer-write-page-geometry-proof-units-points-to-mm
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error-fs
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [near](../../functions/tests/native_pdf_config/near.md)
- [copied_template_values_reach_every_existing_command](../../functions/tests/native_pdf_config/copied_template_values_reach_every_existing_command.md)
- [explicit_file_changes_preserve_precision_even_margins_and_native_output](../../functions/tests/native_pdf_config/explicit_file_changes_preserve_precision_even_margins_and_native_output.md)
- [legacy_root_and_section_precedence_are_explicit](../../functions/tests/native_pdf_config/legacy_root_and_section_precedence_are_explicit.md)
- [borders_are_required_and_even_margins_are_complete_or_absent](../../functions/tests/native_pdf_config/borders_are_required_and_even_margins_are_complete_or_absent.md)
- [invalid_files_keys_and_geometry_return_actionable_redacted_errors](../../functions/tests/native_pdf_config/invalid_files_keys_and_geometry_return_actionable_redacted_errors.md)
- [unapplied_settings_are_reported_by_key_not_value](../../functions/tests/native_pdf_config/unapplied_settings_are_reported_by_key_not_value.md)
- [user_config_controls_actual_rendered_paths](../../functions/tests/native_pdf_config/user_config_controls_actual_rendered_paths.md)

# Imports

- `oxidize_pdf::parser::PdfDocument`
- `stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
}`
- `std::{error::Error, fs}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)