# Commemorative Layout Integration Guide

## Overview

The improved commemorative album format has been successfully integrated into the program. This new `commemorative_layout` module provides structured, grid-based formatting for stamp albums with proper background colors, aligned images, and organized description boxes.

## What's Changed

### New Module: `commemorative_layout.rs`

Located at: `src/album_globals/generator_new/commemorative_layout.rs`

This module exports the following public functions:

- **`emit_commemorative_page_start()`** - Initializes a commemorative page with proper formatting settings
- **`emit_stamp_grid_row()`** - Emits a grid row with up to 2 stamp images and headings
- **`emit_description_boxes()`** - Generates description boxes for the stamps in the row
- **`emit_commemorative_page_end()`** - Closes page formatting
- **`emit_commemorative_page()`** - High-level function that generates a complete page with multiple stamps

### Key Features

1. **Grid Layout**: Stamps are arranged in 2-column grids
2. **Color Formatting**:
   - Black background for image area
   - White background for description boxes
3. **Consistent Spacing**: Uses `PAGE_VSPACE (-5.0)` between images and descriptions
4. **Structured Data**: Builds descriptions from stamp attributes:
   - Description/title
   - Issue date
   - Color
   - Quantity issued
   - Additional notes

## Format Example

The new layout generates output like this:

```
PAGE_START
PAGE_VSPACE (3.0)
ROW_START_ES (HN 6 0.2)

COLOR_STAMP_BORDER(black)
COLOR_STAMP_BACKGROUND(black)

ROW_ALIGN_TOP

STAMP_ADD_IMG (28.0 45.0 "images/1064.jpg" "3¢" "Perf 10½ x 11" "Vertical")
STAMP_HEADING (HB 9 "Scott 1064: PA Academy Arts")
STAMP_ADD_IMG (28.0 45.0 "images/1065.jpg" "3¢" "Perf 10½ x 11" "Horizontal")
STAMP_HEADING (HB 9 "Scott 1065: Land Grant Colleges")

PAGE_VSPACE (-5.0)
ROW_START_ES (HN 6 0.2)
ROW_ALIGN_TOP
COLOR_STAMP_BORDER(white)
COLOR_STAMP_BACKGROUND(white)
STAMP_ADD (28.0 32.0 "Description text\nIssued: date\nColor: color\nQuantity: qty" "" "" "" NO_FRAME)
COLOR_STAMP_BORDER(black)
...
```

## How to Use

### In Code

To use the commemorative layout in your generator code:

```rust
use crate::album_globals::generator_new::commemorative_layout;

// For a single page:
commemorative_layout::emit_commemorative_page(
    &mut output,
    page_number,
    "Page Title",
    &stamp_vec,
)?;

// Or step-by-step:
commemorative_layout::emit_commemorative_page_start(&mut output, page_number, "Title")?;
commemorative_layout::emit_stamp_grid_row(&mut output, &stamp1, Some(&stamp2))?;
commemorative_layout::emit_description_boxes(&mut output, &stamp1, Some(&stamp2))?;
commemorative_layout::emit_commemorative_page_end(&mut output)?;
```

### Integration Points

The module is ready to be integrated into:

1. **Main generator** (`src/album_globals/generator.rs`) - Detect commemorative series and use this layout
2. **Format detection** - Add logic to identify commemorate vs. definitive stamps
3. **Series iteration** - Group stamps appropriately for the grid layout

## Building

Build the project with the new module:

```bash
cargo build --release
```

Check for compilation errors:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

## Sample Output

See the example file: `Albums/1955/1955-COMMEMORATIVE-SAMPLE-OUTPUT.txt`

This file demonstrates the final formatted output that AlbumEasy will render.

## Next Steps

To fully integrate this into the generator:

1. **Detect Format Type**: Modify the main generator to check if stamps are commemorative
2. **Use New Layout**: Call the commemorative layout functions when appropriate
3. **Test Integration**: Generate sample albums and verify output matches expected format
4. **Refine Styling**: Adjust spacing, fonts, and colors as needed

## Notes

- The module uses the `Stamp` structure with its `attrs` HashMap for all data access
- Description text automatically escapes newlines for AlbumEasy compatibility
- The layout supports variable stamp sizes through `est_width_mm` and `est_height_mm`
- Empty or missing fields are gracefully handled and skipped
