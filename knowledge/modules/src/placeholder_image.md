---
type: Rust Module
title: placeholder_image
resource: src/placeholder_image.rs#L1-L836
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-image-utils-calc-mm-per-pixel-embedded
    resolved_by: tree-sitter
    confidence: exact
  - target: external/ab-glyph-point-font-fontarc-pxscale-scalefont
    resolved_by: tree-sitter
    confidence: exact
  - target: external/image-genericimageview-imagebuffer-rgba
    resolved_by: tree-sitter
    confidence: exact
  - target: external/log-error-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-file
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [luminance](../../functions/src/placeholder_image/luminance.md)
- [is_color_dark](../../functions/src/placeholder_image/is_color_dark.md)
- [contrasting_color](../../functions/src/placeholder_image/contrasting_color.md)
- [compute_placeholder_size_for_series](../../functions/src/placeholder_image/compute_placeholder_size_for_series.md)
- [generate_placeholder_for_stamp_with_bg](../../functions/src/placeholder_image/generate_placeholder_for_stamp_with_bg.md)
- [generate_transparent_spacer](../../functions/src/placeholder_image/generate_transparent_spacer.md)
- [load_fallback_font](../../functions/src/placeholder_image/load_fallback_font.md)
- [draw_text](../../functions/src/placeholder_image/draw_text.md)
- [PlaceholderOptions](../../classes/src/placeholder_image/PlaceholderOptions.md)
- [generate_placeholder_image_with_options](../../functions/src/placeholder_image/generate_placeholder_image_with_options.md)
- [generate_placeholder_image](../../functions/src/placeholder_image/generate_placeholder_image.md)
- [ensure_placeholder_image_for_album](../../functions/src/placeholder_image/ensure_placeholder_image_for_album.md)
- [ensure_placeholder_image_for_stamp](../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp.md)
- [PlaceholderImageOptions](../../classes/src/placeholder_image/PlaceholderImageOptions.md)
- [ensure_placeholder_image_for_stamp_with_hint](../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)
- [ensure_placeholder_image](../../functions/src/placeholder_image/ensure_placeholder_image.md)
- [get_placeholder_image_path](../../functions/src/placeholder_image/get_placeholder_image_path.md)

# Imports

- `crate::image_utils::calc_mm_per_pixel_embedded`
- `ab_glyph::{point, Font, FontArc, PxScale, ScaleFont}`
- `image::{GenericImageView, ImageBuffer, Rgba}`
- `log::{error, info}`
- `std::fs::File`
- `std::io::Read`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)