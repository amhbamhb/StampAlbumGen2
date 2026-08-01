# Configuration Parameters Inventory

## Complete Config Parameters from config.rs (UPDATED)

### Core Stamp Dimensions

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `reference_width_mm` | f32 | 25.0 | Reference stamp width in mm for scaling calculations |
| `reference_height_mm` | f32 | 30.0 | Reference stamp height in mm for scaling calculations |
| `stamp_box_height` | f32 | 30.0 | Maximum stamp height before scaling |
| `stamp_box_adjust` | f32 | 3.0 | Border adjustment around stamps |
| `max_stamps_per_row` | u32 | 5 | Maximum stamps per row |
| `album_stamp_img_new_setting_w_mm` | f32 | - | New stamp image width setting |
| `album_stamp_img_new_setting_h_mm` | f32 | - | New stamp image height setting |
| `album_stamp_img_new_setting_preserve_aspect` | bool | - | Preserve aspect ratio in new stamp settings |
| `preserve_aspect` | bool | - | Backwards-compatible alias for preserve aspect ratio |
| `preserve_real_image_size` | bool | false | Don't scale images to stamp_box_height |
| `force_dpi` | f32 | - | Override DPI calculation for pixel->mm conversion |

### Page Layout & Dimensions

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `album_page_width_mm` | f32 | 216.0 | Page width in mm (US Letter) |
| `album_page_height_mm` | f32 | 279.4 | Page height in mm (US Letter) |
| `page_top_margin_mm` | f32 | 10.0 | Top page margin |
| `page_bottom_margin_mm` | f32 | 10.0 | Bottom page margin |
| `page_left_margin_mm` | f32 | 10.0 | Left page margin |
| `page_right_margin_mm` | f32 | 10.0 | Right page margin |
| `album_pages_spacing_h` | f32 | 0.0 | Horizontal spacing between album pages |
| `album_pages_spacing_v` | f32 | - | Vertical spacing between album pages |
| `page_fit_conservative_buffer_mm` | f32 | 11.0 | Buffer reserved at bottom of page |
| `usable_height_padding_mm` | f32 | - | Extra padding subtracted from usable height |
| `allow_single_large_stamp_per_page` | bool | false | Allow single large stamp to occupy full page |
| `single_stamp_threshold_pct` | f32 | - | Fraction above which single-row page permitted |
| `max_rows_per_page` | usize | - | Override maximum rows per page |

### Page Borders

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page_border_outer` | f32 | - | Outer border width |
| `page_border_inner1` | f32 | - | First inner border width |
| `page_border_inner2` | f32 | - | Second inner border width |
| `page_border_spacing` | f32 | - | Spacing between borders |

### AlbumEasy Row Commands

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `row_start_es_enabled` | bool | true | Enable automatic ROW_START_ES |
| `row_start_es_font` | String | "6" | Font for ROW_START_ES |
| `row_start_es_size` | f32 | 0.10 | Font size for ROW_START_ES |
| `row_start_es_spacing` | f32 | 1.0 | Horizontal spacing between stamps |
| `row_start_es_vspace` | f32 | 5.0 | Vertical space after ROW_START_ES |
| `row_start_es_marker` | String | "ROW_START_ES" | Custom marker for row starts |

### Header & Footer Configuration

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `album_pages_title_font` | String | "Helvetica-Bold" | Album title font |
| `album_pages_title_size` | f32 | 16.0 | Album title font size |
| `album_pages_header_position` | String | "centre" | Header position: L, R, C, LR, RL |
| `album_pages_footer_number_enabled` | bool | true | Enable page numbering |
| `album_pages_footer_number_font` | String | - | Page number font |
| `album_pages_footer_number_size` | f32 | - | Page number font size |
| `album_pages_footer_number_position` | String | - | Page number position |
| `album_pages_footer_number_start` | i32 | 1 | Starting page number |
| `album_pages_footer_number_before` | String | - | Text before page number |
| `album_pages_footer_number_after` | String | - | Text after page number |

### Text & Spacing Configuration

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `row_font_spacing` | f32 | 2.0 | Spacing between rows |
| `header_font_spacing` | f32 | 3.0 | Header font spacing |
| `global_scale` | f32 | 1.0 | Global scaling factor |
| `header_vspace_before_mm` | f32 | - | Vertical space before headers |
| `header_vspace_after_mm` | f32 | - | Vertical space after headers |
| `text_line_height_mm` | f32 | - | Line height for text |
| `image_spacing_mm` | f32 | - | Spacing between images |
| `median_width_cap_3` | f32 | 36.0 | Width threshold for 3 stamps per row |
| `median_width_cap_4` | f32 | 30.0 | Width threshold for 4 stamps per row |

### Packing & Safety Margins

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `packing_horizontal_safety_mm` | f32 | - | Horizontal safety padding |
| `packing_vertical_safety_mm` | f32 | - | Vertical safety padding |
### Page Text Templates

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page_text_format` | String | - | Template for PAGE_TEXT commands |
| `page_text_centre_format` | String | - | Template for PAGE_TEXT_CENTRE commands |
| `page_text_right_format` | String | - | Template for PAGE_TEXT_RIGHT commands |
| `page_text_font` | String | "HN" | Default page text font |
| `page_text_size` | f32 | 10.0 | Default page text size |
| `page_text_vspace` | f32 | 1.0 | Default page text vspace |
| `page_text_centre_font` | String | "TN" | Centre text font |
| `page_text_centre_size` | f32 | 8.0 | Centre text size |
| `page_text_centre_vspace` | f32 | 1.0 | Centre text vspace |
| `page_text_right_font` | String | "HN" | Right text font |
| `page_text_right_size` | f32 | 10.0 | Right text size |
| `page_text_right_vspace` | f32 | 1.0 | Right text vspace |

### Stamp Headings

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `stamp_heading_font` | String | "HN" | Font for stamp headings |
| `stamp_heading_size` | f32 | 10.0 | Size for stamp headings |
| `stamp_heading_label` | String | "auto" | Label prefix for stamp headings. Use `auto` to follow catalog family dynamically per stamp; set empty string to hide heading labels. |
| `stamp_heading_in_row_height` | bool | false | Include heading in row height calc |

### Review UI Catalog Families

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `review_ui.catalog_family_options` | String[] | Built-in list | Optional searchable catalog family master list for Review UI selectors. Entry format: `code|label|count`. Stamp-specific families are shown first; remaining families appear under "Other catalogs". Use `sn` label as `Scott Number`. |

### Content Control & Header Information

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `include_design` | bool | false | Include design information in headers |
| `include_engraving` | bool | false | Include engraving information in headers |
| `include_watermark` | bool | false | Include watermark information in headers |
| `include_perforation` | bool | false | Include perforation information in headers |
| `include_catalog_numbers` | bool | false | Include catalog numbers in headers |
| `include_series` | bool | true | Include series information in headers |
| `include_country` | bool | true | Include country information in headers |
| `include_date` | bool | true | Include date information in headers |
| `print_flag_cover` | bool | true | Print flag cover page |
| `fetch_wikipedia` | bool | false | Fetch Wikipedia summary |
| `include_wikipedia_content` | bool | false | Legacy Wikipedia alias |

### Color Customization

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `color_album_border` | String | - | Album border color |
| `color_album_footer` | String | - | Album footer color |
| `color_album_header` | String | - | Album header color |
| `color_album_title` | String | - | Album title color |
| `color_album_margin_txt` | String | - | Album margin text color |
| `color_stamp_border` | String | - | Stamp border color |
| `color_stamp_heading` | String | - | Stamp heading color |
| `color_stamp_text` | String | - | Stamp text color |
| `color_stamp_background` | String | - | Stamp background color |

### Flag Cover Settings

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `flag_cover_img_w_mm` | f32 | 120.0 | Flag cover image width |
| `flag_cover_img_h_mm` | f32 | 60.0 | Flag cover image height |
| `flag_cover_page_vspace_top` | f32 | 18.0 | Top space on flag page |
| `flag_cover_image_vspace_after` | f32 | 8.0 | Space after flag image |
| `flag_cover_image_title_font` | String | "Helvetica-Bold" | Flag title font (with image) |
| `flag_cover_image_title_size` | f32 | 36.0 | Flag title size (with image) |
| `flag_cover_image_title_scale` | f32 | 1.0 | Flag title scale (with image) |
| `flag_cover_text_only_vspace_top` | f32 | 24.0 | Top space (text only) |
| `flag_cover_text_only_title_font` | String | "Helvetica-Bold" | Flag title font (text only) |
| `flag_cover_text_only_title_size` | f32 | 40.0 | Flag title size (text only) |
| `flag_cover_text_only_title_scale` | f32 | 1.0 | Flag title scale (text only) |
| `flag_cover_text_only_vspace_after` | f32 | 8.0 | Space after title (text only) |
| `flag_cover_background_color` | String | - | Flag cover background color |

### Series Ordering

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `series_order` | String | "first_date" | Series ordering: first_date, first_sg, alpha, *_desc variants |

### Advanced Layout Control

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `pages_vspace_in_row_height` | bool | false | Include vspace in row height calculations |

### Debugging & Diagnostics

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `detailed_page_fit_diagnostics` | bool | false | Emit detailed PAGE_FIT diagnostics |
| `detailed_page_fit` | bool | false | Legacy alias for above |
| `emit_placeholder_diagnostics` | bool | false | Emit placeholder expansion diagnostics |
| `placeholder_always_overwrite` | bool | true | When true, placeholders generated with explicit mm/mm_per_px mappings will be recreated/overwritten so their pixel dimensions exactly match emitted STAMP_ADD_IMG box sizes. Set to false to preserve existing files where pixel dimensions already match the desired size. |

### Data Collection & Processing

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `expand_grouped_ranges` | String | "none" | How to expand ranges: none, placeholders, detail_fetch |
| `calibration_csv` | String | - | Path to offline calibration CSV file |

### Custom Markers

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page_vspace_marker` | String | "(6)" | Custom marker for page vspace |

## Header Formatting Sub-Section

The `[header_formatting]` section can contain parameters that may override top-level settings:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `date_format` | String | - | Date format string |
| `label_style` | String | - | Label styling |
| `series_style` | String | - | Series styling |
| `include_currency` | bool | false | Include currency in headers |
| `header_spacing` | f32 | - | Spacing for headers |

*Note: All include_* flags can be placed in header_formatting and will be promoted to top-level*

## Configuration Testing Status

| Parameter | Tested | Valid Values | Invalid Values | Default Verified |
|-----------|--------|--------------|----------------|------------------|
| `reference_width_mm` | ⏳ | 10.0-100.0 | -1, 0, "abc" | ⏳ |
| `reference_height_mm` | ⏳ | 10.0-100.0 | -1, 0, "abc" | ⏳ |
| `stamp_box_height` | ⏳ | 10.0-50.0 | -1, 0, "abc" | ⏳ |
| `max_stamps_per_row` | ⏳ | 1-10 | 0, -1, "abc" | ⏳ |
| `print_flag_cover` | ⏳ | true, false | "yes", 1 | ⏳ |
| `force_dpi` | ⏳ | 72.0-600.0 | -1, 0, "abc" | ⏳ |
| *(All other parameters pending testing)* | | | | |

## Missing Features Analysis

### Potentially Missing Config Parameters

- [ ] `--config PATH` CLI option for custom config file path
- [ ] `--verbose` / `--quiet` CLI logging control  
- [ ] `page_size_preset` - A4, Letter, Legal presets
- [ ] `output_format` - Different output formats
- [ ] `backup_enabled` - Auto-backup before overwrite

### Implementation Priority

1. **High**: Complete config parameter validation
2. **Medium**: Add missing CLI options for config control
3. **Low**: Advanced preset and formatting options

---
**Status**: Config audit in progress - all parameters documented from config.rs
**Next**: Build comprehensive testing matrix for validation
**Last Updated**: 2025-01-11

### Critical Parameters (High Priority)

- [ ] `max_stamps_per_row` - Controls row grouping
- [ ] `row_start_es_spacing` - Controls stamp spacing
- [ ] `reference_width_mm` / `reference_height_mm` - Default dimensions
- [ ] `album_page_width_mm` / `album_page_height_mm` - Page layout
- [ ] `row_start_es_enabled` - Controls ROW_START_ES generation

### Layout Parameters (Medium Priority)  

- [ ] All margin settings (page_*_margin_mm)
- [ ] Border settings (page_border_*)
- [ ] Spacing settings (image_spacing_mm, packing_*_safety_mm)
- [ ] Font and size settings (stamp_heading_*, page_text_*)

### Advanced Features (Lower Priority)

- [ ] Color customization parameters
- [ ] Flag cover settings
- [ ] Wikipedia integration
- [ ] Debugging/diagnostic options

### Parameter Validation Tests

| Test Type | Description | Status |
|-----------|-------------|---------|
| Type validation | Ensure f32/bool/String types accepted | ⏳ |
| Range validation | Test negative/extreme values | ⏳ |
| Missing defaults | Verify fallbacks when None | ⏳ |
| Invalid TOML | Test malformed config files | ⏳ |
| Case sensitivity | Test parameter name variations | ⏳ |

### Parameter Interaction Tests

| Interaction | Expected Behavior | Status |
|-------------|------------------|---------|
| `max_stamps_per_row` + `row_start_es_spacing` | Proper row grouping | ✅ |
| `reference_*_mm` + `album_page_*_mm` | Stamps fit on page | ✅ |
| `force_dpi` + `preserve_real_image_size` | DPI override works | ⏳ |
| Color parameters + AlbumEasy output | Colors applied | ⏳ |

---
**Status**: Config audit complete - 80+ parameters documented  
**Next**: AlbumEasy PDF cross-reference
**Last Updated**: 2025-11-01
