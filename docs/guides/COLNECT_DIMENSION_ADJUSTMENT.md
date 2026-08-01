# Colnect Dimension Adjustment Feature

## Overview

Added configurable **separate** width and height adjustments for Colnect stamp dimensions to compensate for printing size differences.

## Problem

Stamps with Colnect dimensions were printing at incorrect sizes. Initial implementation used a single adjustment for both dimensions, but testing revealed that width and height need **independent adjustments** because printing distortion affects them differently (typically height needs 2-3mm more adjustment than width).

## Solution

Implemented separate configuration parameters `colnect_width_adjust_mm` and `colnect_height_adjust_mm` that add specified amounts (in mm) independently to width and height when using Colnect dimension data.

## Changes Made

### 1. Configuration Files

- **config.toml**: Added `colnect_width_adjust_mm = 6.0` and `colnect_height_adjust_mm = 9.0` with documentation
- **config_template.toml**: Added both parameters with detailed explanation
- **config_full_example.toml**: Added both parameters to the full example

### 2. Configuration Struct

- **src/config.rs**: Added `pub colnect_width_adjust_mm: Option<f32>` and `pub colnect_height_adjust_mm: Option<f32>` fields to Config struct

### 3. Dimension Calculation Logic

- **src/album_globals/generator.rs** (lines ~386-401):
  - Retrieves separate width and height adjustment values from config (defaults to 2.0mm each if not set)
  - Applies independent adjustments when using Colnect dimensions in the pre-pass calculation
  - Formula: `s.est_width_mm = Some(w_mm + colnect_w_adjust);` and `s.est_height_mm = Some(h_mm + colnect_h_adjust);`

- **src/album_globals/generator_new/helpers.rs** (lines ~299-315):
  - Added config parameter to `estimate_and_ensure_placeholders()` function
  - Retrieves adjustment value from config (defaults to 2.0mm if not set)  
  - Applies adjustment when using Colnect dimensions
  - Formula: `s.est_width_mm = Some(w_mm + colnect_adjust);`

## Configuration Example

```toml
# Colnect-specific dimension adjustment (mm)
# When using Colnect CSV data, the width and height from Colnect can be adjusted
# to compensate for printing differences. This adds the specified amount (in mm)
# to both width and height. Default is 2.0mm to account for stamps appearing
# smaller when printed. Set to 0.0 to use exact Colnect dimensions.
colnect_dimension_adjust_mm = 2.0
```

## Usage

### Default Behavior

If not specified in config.toml, the adjustment defaults to **2.0mm** per dimension.

### Custom Values

- Set to `2.0` (default): Adds 2mm to both width and height (e.g., 26mm → 28mm, 37mm → 39mm)
- Set to `0.0`: Uses exact Colnect dimensions without adjustment
- Can be any positive or negative value to fine-tune output size

### Example Results

With `colnect_dimension_adjust_mm = 2.0`:

- Input: SG 586 (26mm x 37mm from Colnect)
- After adjustment: 28mm x 39mm
- Final output (with stamp_box_adjust = -2.5): ~25.5mm x ~36.5mm

## Technical Details

### Dimension Priority Order

1. **Colnect dimensions** (with adjustment) - highest priority
2. Database pixel dimensions converted to mm
3. Image file measurements
4. Series average dimensions
5. Reference dimensions from config

### Application Points

The adjustment is applied at two critical points:

1. In `generator.rs` during the pre-pass dimension calculation
2. In `generator_new/helpers.rs` during stamp enrichment

### Backward Compatibility

The feature is fully backward compatible:

- Existing configurations without `colnect_dimension_adjust_mm` use the default 2.0mm
- Non-Colnect stamps are unaffected (use existing dimension logic)
- The adjustment only applies when Colnect width/height data is present

## Testing

Successfully compiled in both debug and release modes. The adjustment can be verified by:

1. Setting `colnect_dimension_adjust_mm` to different values in config.toml
2. Generating albums with Colnect data
3. Examining the diagnostic output showing `est_w_mm` and `est_h_mm` values
4. Measuring the printed output to confirm size accuracy

## Related Files

- Configuration: config.toml, config_template.toml, config_full_example.toml
- Code: src/config.rs, src/album_globals/generator.rs, src/album_globals/generator_new/helpers.rs
- Documentation: This file (COLNECT_DIMENSION_ADJUSTMENT.md)
