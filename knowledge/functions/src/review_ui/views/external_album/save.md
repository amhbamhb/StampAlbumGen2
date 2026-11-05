---
type: JavaScript Function
title: save
resource: src/review_ui/views/external_album.js#L294-L307
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/api
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/awaitJob
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/renderAlbum
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/try_rasterize_svg_to_png
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/normalize_album_images_for_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof_with_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/generate_transparent_spacer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/generate_placeholder_image_with_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_colnect_pdf_filters/verify_filtered_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_cover_output_root/generated_text_and_local_coat_use_the_same_configured_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/backfill_image_dims/backfill_image_dimensions_persists_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/flagdb/unmapped_country_can_reuse_a_valid_album_local_flag_without_network
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/missing_corrupt_and_non_alpha_png_assets_fail_before_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function save()`

# Calls

- [api](../../../../../functions/src/review_ui/views/external_album/api.md)
- [awaitJob](../../../../../functions/src/review_ui/views/external_album/awaitJob.md)
- [renderAlbum](../../../../../functions/src/review_ui/views/external_album/renderAlbum.md)

# Called by

- [ensure_coat_of_arms_with_fetcher](../../../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [try_rasterize_svg_to_png](../../../../../functions/src/flagdb/try_rasterize_svg_to_png.md)
- [normalize_album_images_for_pdf](../../../../../functions/src/image_downloader/normalize_album_images_for_pdf.md)
- [write_page_geometry_proof_with_date](../../../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)
- [generate_placeholder_for_stamp_with_bg](../../../../../functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg.md)
- [generate_transparent_spacer](../../../../../functions/src/placeholder_image/generate_transparent_spacer.md)
- [generate_placeholder_image_with_options](../../../../../functions/src/placeholder_image/generate_placeholder_image_with_options.md)
- [ensure_placeholder_image_for_stamp](../../../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp.md)
- [ensure_placeholder_image_for_stamp_with_hint](../../../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)
- [verify_filtered_output](../../../../../functions/tests/album_colnect_pdf_filters/verify_filtered_output.md)
- [generated_text_and_local_coat_use_the_same_configured_root](../../../../../functions/tests/album_cover_output_root/generated_text_and_local_coat_use_the_same_configured_root.md)
- [backfill_image_dimensions_persists_to_db](../../../../../functions/tests/backfill_image_dims/backfill_image_dimensions_persists_to_db.md)
- [unmapped_country_can_reuse_a_valid_album_local_flag_without_network](../../../../../functions/tests/flagdb/unmapped_country_can_reuse_a_valid_album_local_flag_without_network.md)
- [missing_corrupt_and_non_alpha_png_assets_fail_before_output](../../../../../functions/tests/native_pdf_decorative_border/missing_corrupt_and_non_alpha_png_assets_fail_before_output.md)