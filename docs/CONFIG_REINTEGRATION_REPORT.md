Config reintegration mapping report
=================================

This report maps keys found in `config_full_example.toml` to the existing `Config` struct in `src/config.rs`.

Summary:

- Keys that map directly to `Config` fields (present):
  - stamp_box_height -> `stamp_box_height`
  - stamp_box_adjust -> `stamp_box_adjust`
  - row_font_spacing -> `row_font_spacing`
  - header_font_spacing -> `header_font_spacing`
  - reference_width_mm -> `reference_width_mm`
  - reference_height_mm -> `reference_height_mm`
  - album_page_width_mm -> `album_page_width_mm`
  - album_page_height_mm -> `album_page_height_mm`
  - page_top_margin_mm -> `page_top_margin_mm`
  - page_bottom_margin_mm -> `page_bottom_margin_mm`
  - page_left_margin_mm -> `page_left_margin_mm`
  - page_right_margin_mm -> `page_right_margin_mm`
  - page_border_outer -> `page_border_outer`
  - page_border_inner1 -> `page_border_inner1`
  - page_border_inner2 -> `page_border_inner2`
  - page_border_spacing -> `page_border_spacing`
  - album_pages_spacing_h -> `album_pages_spacing_h`
  - album_pages_spacing_v -> `album_pages_spacing_v`
  - median_width_cap_3 -> `median_width_cap_3`
  - median_width_cap_4 -> `median_width_cap_4`
  - page_fit_conservative_buffer_mm -> `page_fit_conservative_buffer_mm`
  - album_stamp_img_new_setting_w_mm -> `album_stamp_img_new_setting_w_mm`
  - album_stamp_img_new_setting_h_mm -> `album_stamp_img_new_setting_h_mm`
  - album_stamp_img_new_setting_preserve_aspect -> `album_stamp_img_new_setting_preserve_aspect`
  - image_spacing_mm -> `image_spacing_mm`
  - text_line_height_mm -> `text_line_height_mm`
  - include_design -> `include_design`
  - include_engraving -> `include_engraving`
  - include_watermark -> `include_watermark`
  - include_perforation -> `include_perforation`
  - include_catalog_numbers -> `include_catalog_numbers`
  - include_series -> `include_series`
  - include_country -> `include_country`
  - include_date -> `include_date`
  - print_flag_cover -> `print_flag_cover`
  - expand_grouped_ranges -> `expand_grouped_ranges`
  - detailed_page_fit_diagnostics -> `detailed_page_fit_diagnostics`
  - fetch_wikipedia -> `fetch_wikipedia`
  - row_start_es_enabled -> `row_start_es_enabled`
  - max_rows_per_page -> `max_rows_per_page`
  - allow_single_large_stamp_per_page -> `allow_single_large_stamp_per_page`
  - single_stamp_threshold_pct -> `single_stamp_threshold_pct`
  - usable_height_padding_mm -> `usable_height_padding_mm`
  - header_vspace_before_mm -> `header_vspace_before_mm`
  - header_vspace_after_mm -> `header_vspace_after_mm`

- Keys present in the example but NOT present in `Config` (none found in this example):
  - None. All keys in the example map to existing `Config` fields.

Notes:

- `load_config()` promotes keys under `[header_formatting]` into top-level. If your real config places include_* toggles inside `[header_formatting]` they will still be propagated.
- Because `Config` uses `Option<T>` for many fields, deserialization will happily ignore extra unknown keys. However, we verified no missing required keys in this example file.

Recommended next step: add a unit test that writes `config_full_example.toml` into a temp directory, calls `load_config("config.toml")` after chdir into that directory and asserts a few representative values.
