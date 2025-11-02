# Configuration Parameters Inventory

## Complete Config Parameters from config.rs

### Core Stamp Dimensions
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `reference_width_mm` | f32 | 20.0 | Default stamp width in mm |
| `reference_height_mm` | f32 | 24.0 | Default stamp height in mm |
| `stamp_box_height` | f32 | - | Maximum stamp height before scaling |
| `stamp_box_adjust` | f32 | - | Border adjustment around stamps |
| `max_stamps_per_row` | u32 | 5 | Maximum stamps per row (NEW) |

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
| `row_start_es_font` | String | "HN" | Font for ROW_START_ES |
| `row_start_es_size` | f32 | 6 | Font size for ROW_START_ES |
| `row_start_es_spacing` | f32 | 1.0 | Horizontal spacing between stamps |
| `row_start_es_vspace` | f32 | 5.0 | Vertical space after ROW_START_ES |
| `row_start_es_marker` | String | - | Custom marker for row starts |

### Spacing & Layout
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `image_spacing_mm` | f32 | - | Vertical spacing between images |
| `text_line_height_mm` | f32 | 2.82 | Height per text line |
| `header_vspace_before_mm` | f32 | - | Space before headers |
| `header_vspace_after_mm` | f32 | - | Space after headers |
| `packing_horizontal_safety_mm` | f32 | - | Horizontal safety margin |
| `packing_vertical_safety_mm` | f32 | - | Vertical safety margin |

### Page Title & Headers
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `album_pages_title_font` | String | - | Album title font |
| `album_pages_title_size` | f32 | - | Album title size |
| `album_pages_header_position` | String | - | Header position: L, R, C, LR, RL |

### Page Text Templates
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page_text_format` | String | - | Template for PAGE_TEXT |
| `page_text_centre_format` | String | - | Template for PAGE_TEXT_CENTRE |
| `page_text_right_format` | String | - | Template for PAGE_TEXT_RIGHT |
| `page_text_font` | String | - | Default page text font |
| `page_text_size` | f32 | - | Default page text size |
| `page_text_vspace` | f32 | - | Default page text vspace |

### Stamp Headings
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `stamp_heading_font` | String | "HN" | Font for stamp headings |
| `stamp_heading_size` | f32 | 12 | Size for stamp headings |
| `stamp_heading_label` | String | "SG : {}" | Template for stamp labels |
| `stamp_heading_in_row_height` | bool | - | Include heading in row height calc |

### Footer & Page Numbers
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `album_pages_footer_number_enabled` | bool | - | Enable page numbers |
| `album_pages_footer_number_font` | String | - | Page number font |
| `album_pages_footer_number_size` | f32 | - | Page number size |
| `album_pages_footer_number_position` | String | - | Page number position |
| `album_pages_footer_number_start` | i32 | - | Starting page number |
| `album_pages_footer_number_before` | String | - | Text before page number |
| `album_pages_footer_number_after` | String | - | Text after page number |

### Image Processing
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `force_dpi` | f32 | - | Override DPI for pixel->mm conversion |
| `preserve_real_image_size` | bool | false | Don't scale to stamp_box_height |
| `album_stamp_img_new_setting_w_mm` | f32 | - | New stamp width setting |
| `album_stamp_img_new_setting_h_mm` | f32 | - | New stamp height setting |
| `album_stamp_img_new_setting_preserve_aspect` | bool | - | Preserve aspect ratio |
| `preserve_aspect` | bool | - | Legacy preserve aspect alias |

### Content Control
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `include_design` | bool | - | Include design information |
| `include_engraving` | bool | - | Include engraving information |
| `include_watermark` | bool | - | Include watermark information |
| `include_perforation` | bool | - | Include perforation information |
| `include_catalog_numbers` | bool | - | Include catalog numbers |
| `include_series` | bool | - | Include series information |
| `include_country` | bool | - | Include country information |
| `include_date` | bool | - | Include date information |
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
| `flag_cover_img_w_mm` | f32 | - | Flag cover image width |
| `flag_cover_img_h_mm` | f32 | - | Flag cover image height |
| `flag_cover_page_vspace_top` | f32 | - | Top space on flag page |
| `flag_cover_image_vspace_after` | f32 | - | Space after flag image |
| `flag_cover_image_title_font` | String | - | Flag title font (with image) |
| `flag_cover_image_title_size` | f32 | - | Flag title size (with image) |
| `flag_cover_image_title_scale` | f32 | - | Flag title scale (with image) |
| `flag_cover_text_only_vspace_top` | f32 | - | Top space (text only) |
| `flag_cover_text_only_title_font` | String | - | Flag title font (text only) |
| `flag_cover_text_only_title_size` | f32 | - | Flag title size (text only) |
| `flag_cover_text_only_title_scale` | f32 | - | Flag title scale (text only) |
| `flag_cover_text_only_vspace_after` | f32 | - | Space after title (text only) |
| `flag_cover_background_color` | String | - | Flag cover background color |

### Advanced Layout
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `max_rows_per_page` | usize | - | Maximum rows per page |
| `allow_single_large_stamp_per_page` | bool | - | Allow large stamps on single page |
| `single_stamp_threshold_pct` | f32 | - | Threshold for single stamp pages |
| `usable_height_padding_mm` | f32 | - | Extra bottom padding |
| `page_fit_conservative_buffer_mm` | f32 | - | Conservative footer buffer |
| `pages_vspace_in_row_height` | bool | - | Include vspace in row height |

### Series Ordering
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `series_order` | String | "first_date" | Series ordering: first_date, first_sg, alpha |

### Debugging & Diagnostics
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `detailed_page_fit_diagnostics` | bool | false | Emit detailed PAGE_FIT diagnostics |
| `detailed_page_fit` | bool | false | Legacy alias for above |
| `emit_placeholder_diagnostics` | bool | - | Emit placeholder expansion diagnostics |

### Data Collection
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `expand_grouped_ranges` | String | - | How to expand ranges: placeholders, detail_fetch |
| `calibration_csv` | String | - | Path to calibration CSV file |

### Legacy/Compatibility
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `median_width_cap_3` | f32 | - | Legacy width cap for 3 stamps |
| `median_width_cap_4` | f32 | - | Legacy width cap for 4 stamps |
| `row_font_spacing` | f32 | - | Legacy row font spacing |
| `global_scale` | f32 | - | Legacy global scale |
| `header_font_spacing` | f32 | - | Legacy header font spacing |
| `page_vspace_marker` | String | - | Custom vspace marker |

## Header Formatting Sub-Section
The `[header_formatting]` section can contain its own parameters that may override top-level settings:

| Parameter | Type | Description |
|-----------|------|-------------|
| `date_format` | String | Date format string |
| `label_style` | String | Label styling |
| `series_style` | String | Series styling |
| `include_currency` | bool | Include currency in headers |
| `header_spacing` | f32 | Spacing for headers |
| *(Plus all include_* flags can be placed here)* | | |

## Configuration Testing Matrix

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