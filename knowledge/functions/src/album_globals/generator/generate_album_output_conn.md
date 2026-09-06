---
type: Rust Function
title: generate_album_output_conn
resource: src/album_globals/generator.rs#L501-L3375
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/with_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/calc_mm_per_pixel_embedded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/write_stamp_page_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/globals/compute_usable_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/for_each_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/parse_sort_day
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/colnect_details_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_colnect_detail_field_set
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/estimate_real_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/emit_stamp_diag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/ensure_placeholder_for_stamp_and_update_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/is_commemorative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/is_special_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/emit_commemorative_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/emit_special_stamps_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/commemorative_generator/emit_summary_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/stamp_dedupe_fingerprints
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/stamp_date_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/build_series_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/build_page_text_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/build_date_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/build_watermark_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/build_engraving_page_text_centre_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/clear
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/format_number_with_commas
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/estimate_stamp_body_line_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/estimate_colnect_centre_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/estimate_stamp_heading_line_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/start_stamp_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/write_header_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/strip_bracket_tags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/is_effectively_duplicate_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_default_catalog_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/pad_centre_body_for_side_columns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/emit_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/get_overwrite_album_from_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_album_output_conn<F>( conn: &Connection, album_name: &str, mut page_callback: Option<F>, start_year: Option<i32>, end_year: Option<i32>, ) -> std::path::PathBuf where F: FnMut(usize),`

# Calls

- [load_effective_config](../../../../functions/src/config/load_effective_config.md)
- [with_defaults](../../../../functions/src/config/Config/with_defaults.md)
- [calc_mm_per_pixel_embedded](../../../../functions/src/image_utils/calc_mm_per_pixel_embedded.md)
- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [write_stamp_page_header](../../../../functions/src/album_globals/generator_new/layout/write_stamp_page_header.md)
- [compute_usable_height](../../../../functions/src/album_globals/globals/compute_usable_height.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [for_each_series](../../../../functions/src/album_globals/generator_new/for_each_series.md)
- [fetch_and_filter_stamps](../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [parse_sort_day](../../../../functions/src/album_globals/generator/parse_sort_day.md)
- [colnect_details_enabled](../../../../functions/src/config/Config/colnect_details_enabled.md)
- [effective_colnect_detail_field_set](../../../../functions/src/config/Config/effective_colnect_detail_field_set.md)
- [resolve_album_image_path](../../../../functions/src/file_utils/resolve_album_image_path.md)
- [estimate_real_dimensions](../../../../functions/src/image_utils/estimate_real_dimensions.md)
- [emit_stamp_diag](../../../../functions/src/album_globals/generator_new/helpers/emit_stamp_diag.md)
- [ensure_placeholder_for_stamp_and_update_db](../../../../functions/src/album_globals/generator_new/helpers/ensure_placeholder_for_stamp_and_update_db.md)
- [is_commemorative](../../../../functions/src/album_globals/generator_new/commemorative_layout/is_commemorative.md)
- [is_special_stamp](../../../../functions/src/album_globals/commemorative_generator/is_special_stamp.md)
- [emit_commemorative_pages](../../../../functions/src/album_globals/commemorative_generator/emit_commemorative_pages.md)
- [emit_special_stamps_page](../../../../functions/src/album_globals/commemorative_generator/emit_special_stamps_page.md)
- [emit_summary_page](../../../../functions/src/album_globals/commemorative_generator/emit_summary_page.md)
- [stamp_dedupe_fingerprints](../../../../functions/src/album_globals/generator/stamp_dedupe_fingerprints.md)
- [stamp_date_key](../../../../functions/src/album_globals/generator/stamp_date_key.md)
- [build_series_page_text_centre_template](../../../../functions/src/album_globals/generator_new/helpers/build_series_page_text_centre_template.md)
- [build_page_text_template](../../../../functions/src/album_globals/generator_new/helpers/build_page_text_template.md)
- [build_date_page_text_centre_template](../../../../functions/src/album_globals/generator_new/helpers/build_date_page_text_centre_template.md)
- [build_watermark_page_text_centre_template](../../../../functions/src/album_globals/generator_new/helpers/build_watermark_page_text_centre_template.md)
- [build_engraving_page_text_centre_template](../../../../functions/src/album_globals/generator_new/helpers/build_engraving_page_text_centre_template.md)
- [resolve_stamp_heading_label](../../../../functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label.md)
- [clear](../../../../functions/src/progress/ProgressIndicator/clear.md)
- [format_number_with_commas](../../../../functions/src/album_globals/generator/format_number_with_commas.md)
- [estimate_stamp_body_line_count](../../../../functions/src/album_globals/generator/estimate_stamp_body_line_count.md)
- [estimate_colnect_centre_text](../../../../functions/src/album_globals/generator/estimate_colnect_centre_text.md)
- [estimate_stamp_heading_line_count](../../../../functions/src/album_globals/generator_new/emit/estimate_stamp_heading_line_count.md)
- [start_stamp_page](../../../../functions/src/album_globals/generator_new/layout/start_stamp_page.md)
- [write_header_block](../../../../functions/src/album_globals/generator_new/layout/write_header_block.md)
- [strip_bracket_tags](../../../../functions/src/album_globals/generator/strip_bracket_tags.md)
- [is_effectively_duplicate_text](../../../../functions/src/album_globals/generator/is_effectively_duplicate_text.md)
- [normalize_catalog_type](../../../../functions/src/catalog/normalize_catalog_type.md)
- [effective_default_catalog_code](../../../../functions/src/config/Config/effective_default_catalog_code.md)
- [pad_centre_body_for_side_columns](../../../../functions/src/album_globals/generator/pad_centre_body_for_side_columns.md)
- [emit_stamp](../../../../functions/src/album_globals/generator_new/emit/emit_stamp.md)
- [get_overwrite_album_from_cli](../../../../functions/src/file_utils/get_overwrite_album_from_cli.md)
- [formatted_now](../../../../functions/src/time_utils/formatted_now.md)