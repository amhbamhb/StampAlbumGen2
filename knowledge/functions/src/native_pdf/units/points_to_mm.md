---
type: Rust Function
title: points_to_mm
resource: src/native_pdf/units.rs#L5-L7
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/commands/page/album_pages_decorative_border/image_dimensions_mm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_title/AlbumPagesTitle/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/text/MeasuredText/new_with_font
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_border_colour/rendered_colours_match_reference_and_guide_stays_black
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/frozen_reference_hashes_and_media_sizes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/strokes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/actual_native_strokes_images_alpha_and_content_match_references
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_border/path_bounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_border/rendered_border_paths_match_frozen_references
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/footer_anchor_and_hint_share_layout_without_moving_glyphs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_footer/footer_multiline_padding_and_parity_are_independent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_header/multiline_padding_and_minimum_hint_share_resolved_measurements
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/four_page_sizes_render_and_read_back
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/spacing_state_matches_reference_content_anchors_and_gaps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/invalid_commands_geometry_and_output_fail_explicitly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/rendered_paths_match_margin_and_spacing_fixtures
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn points_to_mm(points: f64) -> f64`

# Called by

- [image_dimensions_mm](../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/image_dimensions_mm.md)
- [layout](../../../../functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/layout.md)
- [layout](../../../../functions/src/native_pdf/commands/page/album_pages_title/AlbumPagesTitle/layout.md)
- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [new_with_font](../../../../functions/src/native_pdf/text/MeasuredText/new_with_font.md)
- [rendered_colours_match_reference_and_guide_stays_black](../../../../functions/tests/native_pdf_border_colour/rendered_colours_match_reference_and_guide_stays_black.md)
- [frozen_reference_hashes_and_media_sizes](../../../../functions/tests/native_pdf_crop_marks/frozen_reference_hashes_and_media_sizes.md)
- [strokes](../../../../functions/tests/native_pdf_crop_marks/strokes.md)
- [actual_native_paths_match_all_repeated_reference_cases](../../../../functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases.md)
- [actual_native_strokes_images_alpha_and_content_match_references](../../../../functions/tests/native_pdf_decorative_border/actual_native_strokes_images_alpha_and_content_match_references.md)
- [path_bounds](../../../../functions/tests/native_pdf_page_border/path_bounds.md)
- [rendered_border_paths_match_frozen_references](../../../../functions/tests/native_pdf_page_border/rendered_border_paths_match_frozen_references.md)
- [footer_anchor_and_hint_share_layout_without_moving_glyphs](../../../../functions/tests/native_pdf_page_footer/footer_anchor_and_hint_share_layout_without_moving_glyphs.md)
- [footer_multiline_padding_and_parity_are_independent](../../../../functions/tests/native_pdf_page_footer/footer_multiline_padding_and_parity_are_independent.md)
- [multiline_padding_and_minimum_hint_share_resolved_measurements](../../../../functions/tests/native_pdf_page_header/multiline_padding_and_minimum_hint_share_resolved_measurements.md)
- [four_page_sizes_render_and_read_back](../../../../functions/tests/native_pdf_page_settings/four_page_sizes_render_and_read_back.md)
- [spacing_state_matches_reference_content_anchors_and_gaps](../../../../functions/tests/native_pdf_page_settings/spacing_state_matches_reference_content_anchors_and_gaps.md)
- [invalid_commands_geometry_and_output_fail_explicitly](../../../../functions/tests/native_pdf_page_settings/invalid_commands_geometry_and_output_fail_explicitly.md)
- [rendered_paths_match_margin_and_spacing_fixtures](../../../../functions/tests/native_pdf_page_settings/rendered_paths_match_margin_and_spacing_fixtures.md)
- [illustration_scale_changes_pdf_image_but_not_known_size_or_frame](../../../../functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame.md)