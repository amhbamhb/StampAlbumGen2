# Album Stamp Image New Setting Parameters

## Overview

The `album_stamp_img_new_setting_*` parameters control how stamp images are sized and displayed in generated albums. These parameters work together to provide fine-grained control over stamp image dimensions and aspect ratio preservation.

## Parameters

### `album_stamp_img_new_setting_w_mm`

- **Type**: `f32` (Optional)
- **Default**: `6.0` (for image sizing), `0.0` (for layout calculations)
- **Purpose**: Additive width adjustment in millimeters

### `album_stamp_img_new_setting_h_mm`

- **Type**: `f32` (Optional)  
- **Default**: `6.0` (for image sizing), `0.0` (for layout calculations)
- **Purpose**: Additive height adjustment in millimeters

### `album_stamp_img_new_setting_preserve_aspect`

- **Type**: `bool` (Optional)
- **Default**: `true`
- **Purpose**: Controls aspect ratio preservation in the generated `ALBUM_STAMP_IMG_NEW_SETTING` command

## How They Work

### 1. Image Sizing (album_globals.rs)

The width and height parameters are added to the calculated stamp dimensions:

```rust
// Apply ALBUM_STAMP_IMG_NEW_SETTING adjustments to width/height (additive)
let img_new_w = config.album_stamp_img_new_setting_w_mm.unwrap_or(6.0) as f64;
let img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(6.0) as f64;
est_width_mm += img_new_w;
est_height_mm += img_new_h;
```

### 2. Layout Calculations (album_globals.rs)

For row height calculations, these parameters use `0.0` as default to avoid affecting layout unless explicitly configured:

```rust
let img_new_w = config.album_stamp_img_new_setting_w_mm.unwrap_or(0.0) as f64;
let img_new_h = config.album_stamp_img_new_setting_h_mm.unwrap_or(0.0) as f64;
```

### 3. AlbumEasy Command Generation (album_utils.rs)

All three parameters are used to generate the `ALBUM_STAMP_IMG_NEW_SETTING` command in the output file:

```rust
let img_new_w = cfg.album_stamp_img_new_setting_w_mm.unwrap_or(6.0);
let img_new_h = cfg.album_stamp_img_new_setting_h_mm.unwrap_or(6.0);
let img_preserve = cfg.album_stamp_img_new_setting_preserve_aspect.unwrap_or(true);

// Generates: ALBUM_STAMP_IMG_NEW_SETTING (6.0 6.0 true)
lines.push(format!(
    "ALBUM_STAMP_IMG_NEW_SETTING ({:.1} {:.1} {})",
    img_new_w, img_new_h, img_preserve
));
```

## Configuration Example

```toml
# Increase stamp box size by 3mm in both dimensions
album_stamp_img_new_setting_w_mm = 3.0
album_stamp_img_new_setting_h_mm = 3.0

# Preserve aspect ratio when AlbumEasy processes the images
album_stamp_img_new_setting_preserve_aspect = true
```

## Usage Locations

The parameters are used consistently across the codebase:

- **album_globals.rs**: Lines 870, 997, 1101, 1362 (width and height parameters)
- **album_utils.rs**: Lines 159-162, 421 (all three parameters)
- **config.rs**: Lines 111-113 (parameter definitions)

## Testing

Comprehensive tests verify:

- Parameter usage consistency across files
- Default value consistency
- Command generation with correct formatting
- Config parameter loading and validation

## Issue Resolution

**Fixed Issue**: Previously, `album_stamp_img_new_setting_w_mm` was missing from several locations in `album_globals.rs` where only the height parameter was being used. This has been resolved to ensure consistent usage of both width and height parameters throughout the codebase.

**Verification**: Tests confirm that width and height parameters are now used equally (4 times each) in `album_globals.rs`, providing consistent behavior for stamp image sizing and layout calculations.
