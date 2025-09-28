# Stamp Album Generator API Documentation

This document provides detailed API documentation for the key functions and modules in StampAlbumGen2.

## Core Modules

### `album_globals`

#### `calculate_stamp_dimensions(image_path: &str, album_name: &str) -> (f64, f64)`

Calculates appropriate stamp box dimensions based on image dimensions and series-specific scaling.

**Parameters:**
- `image_path`: Relative path to the stamp image (e.g., "L-s.jpg")
- `album_name`: Name of the album for path construction

**Returns:**
- Tuple of `(width_mm, height_mm)` representing the calculated stamp dimensions

**Algorithm:**
1. Load actual image pixel dimensions from disk
2. Calculate mm-per-pixel ratios using embedded B4-s.jpg reference
3. Apply series-specific scaling factors (L/M/N series)
4. Apply bounds checking (15mm min, 50mm max width, 30mm max height)
5. Return final dimensions or defaults if image loading fails

**Examples:**
```rust
// L-series stamp (registration/large)
let (w, h) = calculate_stamp_dimensions("L-s.jpg", "Canada");
// Result: (49.0, 21.0) - exact target dimensions

// M-series stamp (medium definitive)
let (w, h) = calculate_stamp_dimensions("M-456.jpg", "Canada");
// Result: (31.1, 24.1) - optimized for medium stamps

// N-series stamp (small definitive)
let (w, h) = calculate_stamp_dimensions("N-123.jpg", "Canada");
// Result: (26.1, 21.0) - compact dimensions
```

#### `generate_album_output_from_stamps(stamps, album_name, page_callback)`

Generates complete AlbumEasy output from a pre-filtered list of stamps.

**Parameters:**
- `stamps`: Vector of `StampStub` objects to include in the album
- `album_name`: Name for the output album
- `page_callback`: Optional callback function called for each page

**Output:**
- Creates `{album_name}.txt` in the `Albums/{album_name}/` directory
- AlbumEasy-compatible format with proper page layout
- Includes global settings, headers, and stamp positioning

### `image_utils`

#### `calc_mm_per_pixel_embedded(real_width_mm, real_height_mm) -> Option<(f32, f32)>`

Calculates mm-per-pixel ratio using the embedded B4-s.jpg reference image.

**Parameters:**
- `real_width_mm`: Real-world width of reference in millimeters
- `real_height_mm`: Real-world height of reference in millimeters

**Returns:**
- `Some((mm_per_pixel_width, mm_per_pixel_height))` or `None` if loading fails

**Reference Values:**
- Standard reference: `(200.0, 240.0)` mm
- Returns: `(0.208, 0.107)` mm/px ratio

#### `estimate_real_dimensions(image_path, mm_per_pixel_w, mm_per_pixel_h) -> Option<(f32, f32)>`

Converts pixel dimensions to real-world millimeters.

**Parameters:**
- `image_path`: Path to the image file
- `mm_per_pixel_w`: Millimeters per pixel for width
- `mm_per_pixel_h`: Millimeters per pixel for height

**Returns:**
- `Some((width_mm, height_mm))` or `None` if image loading fails

#### `get_embedded_reference_dimensions() -> Option<(u32, u32)>`

Returns pixel dimensions of the embedded reference image.

**Returns:**
- `Some((width_px, height_px))` of embedded B4-s.jpg
- `None` if embedded image loading fails

### `sample_images`

#### `get_stamp_references() -> HashMap<String, StampReference>`

Returns complete map of stamp series configurations.

**Returns:**
- HashMap with keys: "L", "M", "N", "small"
- Each entry contains target dimensions and scaling factors

#### `get_scaling_for_series(series: &str) -> (f64, f64)`

Determines scaling factors based on stamp series identifier.

**Parameters:**
- `series`: Stamp filename or series identifier

**Returns:**
- Tuple of `(width_scale, height_scale)` factors

**Series Detection:**
- `"L*.jpg"` or `"O*.jpg"` → L-series scaling (1.25, 2.45)
- `"M*.jpg"` → M-series scaling (1.15, 1.45)
- `"N*.jpg"` → N-series scaling (1.10, 1.40)
- Others → Small series scaling (1.05, 1.35)

## Data Structures

### `StampReference`
```rust
pub struct StampReference {
    pub name: String,              // Human-readable series name
    pub expected_width_mm: f64,    // Target width in millimeters
    pub expected_height_mm: f64,   // Target height in millimeters
    pub scaling_width: f64,        // Width scaling factor
    pub scaling_height: f64,       // Height scaling factor
}
```

### `StampStub`
Core stamp data structure used throughout the application:
```rust
pub struct StampStub {
    pub id: i32,
    pub year: i32,
    pub month: Option<String>,
    pub day: Option<String>,
    pub country: String,
    pub series: String,
    pub denomination: String,
    pub currency: String,
    pub image_name: String,
    // ... additional fields
}
```

## Configuration

### Global Settings (config.toml)
```toml
# Reference dimensions for mm-per-pixel calculations
reference_width_mm = 200.0
reference_height_mm = 240.0

# Stamp box dimensions
stamp_box_height = 30.0

# Font spacing
row_font_spacing = 2.0
header_font_spacing = 3.0
```

### AlbumEasy Global Settings
- `ALBUM_STAMP_IMG_ASPECT_RATIO`: Controls dimension calculation mode
- `ALBUM_STAMP_IMG_NEW_SETTING`: Additional scaling parameters
- `STAMP_BOXES_SIZE_ADJUST`: Fine-tuning for box dimensions

## Error Handling

The API uses Rust's `Result` and `Option` types for error handling:

- **File I/O errors**: Return `None` or default values
- **Image loading failures**: Fallback to default dimensions
- **Database errors**: Logged with graceful degradation
- **Configuration errors**: Use compiled-in defaults

## Performance Considerations

### Optimizations
- **Embedded assets**: No runtime file I/O for reference images
- **Lazy loading**: Images loaded only when needed
- **Bounds checking**: Prevents layout calculation errors
- **Series caching**: Scaling factors computed once per series

### Memory Usage
- **Release build**: ~12MB resident memory
- **Per-stamp overhead**: Minimal additional memory
- **Image processing**: Temporary buffers for dimension calculations

## Integration Examples

### Custom Stamp Processing
```rust
use stamp_album_gen2::{album_globals, image_utils, sample_images};

// Load custom stamp data
let stamps = load_custom_stamps();

// Calculate dimensions for each stamp
for stamp in &stamps {
    let (width, height) = album_globals::calculate_stamp_dimensions(
        &stamp.image_name,
        &stamp.country
    );
    println!("Stamp {}: {:.1}mm x {:.1}mm", stamp.image_name, width, height);
}

// Generate album output
album_globals::generate_album_output_from_stamps(
    stamps,
    "Custom Album",
    None
);
```

### Batch Processing
```rust
// Process multiple albums
let countries = vec!["Canada", "USA", "UK"];

for country in countries {
    let db_path = format!("Albums/{}/{}.db", country, country);
    let stamps = load_stamps_from_db(&db_path);

    album_globals::generate_album_output_from_stamps(
        stamps,
        country,
        Some(|page| println!("Generated page {}", page))
    );
}
```

## Version History

### v0.5.0 (Current)
- ✅ Embedded reference image system
- ✅ Series-specific scaling factors
- ✅ Comprehensive API documentation

## StampWorld slug overrides

The application supports a `slug_overrides.toml` mapping to resolve mismatches between the user-provided `--site-name` and the canonical StampWorld slug. Place a file named `slug_overrides.toml` in the project root with:

```toml
[slug_overrides]
"British-ColumbiaIsland" = "British-Columbia!Vancouver-Island"
```

The loader searches upward from the current working directory and will use the provided mapping before applying heuristic fallbacks.

## CLI options (summary)

- --source <source> (default: stampworld)
- --site-name <site_name> (required for scraping)
- --album-name <album_name>
- --db-path <path> (output-only mode)
- --base-url <url> (override default StampWorld URL construction)
- --start-year <int>
- --end-year <int>
- --overwrite-album
- --overwrite-images
- --max-pages <int>
- --stop-after-no-hits <int>

## Config keys (summary)

See `config_template.toml` for a template. Key groups:

- Page and stamp sizing: `reference_width_mm`, `reference_height_mm`, `stamp_box_height`, `stamp_box_adjust`
- Layout spacing: `row_font_spacing`, `header_font_spacing`
- Header toggles: `include_design`, `include_engraving`, `include_watermark`, `include_perforation`, `include_catalog_numbers`, `include_series`, `include_country`, `include_date`
- `print_flag_cover` (bool)
- Optional `slug_overrides` table (documented above)

- ✅ Release build optimizations

### Future Enhancements
- 🔄 Additional stamp series support
- 🔄 Configurable scaling per country
- 🔄 Enhanced image processing options
- 🔄 Web API interface</content>
<parameter name="filePath">/media/ray/program/Rust/StampAlbumGen2/API_DOCUMENTATION.md
