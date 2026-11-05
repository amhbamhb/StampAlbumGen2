---
type: Rust Module
title: native_pdf_crop_marks
resource: tests/native_pdf_crop_marks.rs#L1-L602
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-colour-namedcolour-commands-page-album-pages-crop-marks-albumpagescropmarks-album-pages-size-pagesize-config-nativepdfconfig-renderer-write-page-geometry-proof-units-points-to-mm
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

- [root](../../functions/tests/native_pdf_crop_marks/root.md)
- [near](../../functions/tests/native_pdf_crop_marks/near.md)
- [config](../../functions/tests/native_pdf_crop_marks/config.md)
- [explicit_cross_geometry_clipping_and_zero_hairlines](../../functions/tests/native_pdf_crop_marks/explicit_cross_geometry_clipping_and_zero_hairlines.md)
- [arguments_are_explicit_finite_nonnegative_and_pdf_safe](../../functions/tests/native_pdf_crop_marks/arguments_are_explicit_finite_nonnegative_and_pdf_safe.md)
- [marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges](../../functions/tests/native_pdf_crop_marks/marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges.md)
- [missing_invalid_disabled_keys_redaction_and_unknown_keys](../../functions/tests/native_pdf_crop_marks/missing_invalid_disabled_keys_redaction_and_unknown_keys.md)
- [templates_are_explicit_disabled_and_snapshot_is_independent](../../functions/tests/native_pdf_crop_marks/templates_are_explicit_disabled_and_snapshot_is_independent.md)
- [frozen_reference_hashes_and_media_sizes](../../functions/tests/native_pdf_crop_marks/frozen_reference_hashes_and_media_sizes.md)
- [invalid_layout_creates_no_pdf_and_preserves_existing_output](../../functions/tests/native_pdf_crop_marks/invalid_layout_creates_no_pdf_and_preserves_existing_output.md)
- [crop_marks_leave_all_measured_text_and_reservations_unchanged](../../functions/tests/native_pdf_crop_marks/crop_marks_leave_all_measured_text_and_reservations_unchanged.md)
- [cases](../../functions/tests/native_pdf_crop_marks/cases.md)
- [trace](../../functions/tests/native_pdf_crop_marks/trace.md)
- [strokes](../../functions/tests/native_pdf_crop_marks/strokes.md)
- [actual_native_paths_match_all_repeated_reference_cases](../../functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases.md)
- [rendered_native_colour_is_independent_of_album_border](../../functions/tests/native_pdf_crop_marks/rendered_native_colour_is_independent_of_album_border.md)

# Imports

- `sha2::{Digest, Sha256}`
- `stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    commands::page::{album_pages_crop_marks::AlbumPagesCropMarks, album_pages_size::PageSize},
    config::NativePdfConfig,
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