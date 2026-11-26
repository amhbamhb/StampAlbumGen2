# Printer Crop Marks (`crop-marks-v1`)

This fixture family tests and showcases printer crop marks (cutting guides) in StampAlbumGen2 across both the native PDF engine and raw AlbumEasy format.

---

## 1. Stamp Collector Guide (User-Friendly Overview)

### What Are Printer Crop Marks?
Printer crop marks are small crosshair lines printed at the four corners of an album page. When printing album pages on oversized or standard commercial cardstock (such as printing an 8.5 × 11 in US Letter page or 210 × 297 mm A4 page on larger paper), crop marks show exactly where to cut or trim the paper with a guillotine paper cutter so your album pages are precisely the right size.

### Why Use Custom Colors on Crop Marks?
By default, printer crop marks are printed in standard hairline black. However, stamp collectors and exhibit creators often prefer:
- **High-contrast colors** (such as navy `#1E4A7A`, dark red `darkred`, or forest green `forestgreen`) so cutting guidelines stand out clearly against off-white, cream, or textured archival paper.
- **Color coordination** with page titles, album borders, and country flag accents.
- **Distinguishing proof/draft cuts** from finished collector exhibition pages.

### How to Use Crop Marks in Your Album

You can use either standard AlbumEasy syntax or the native StampAlbum command syntax. All arguments are completely optional:

1. **Default Crop Marks (Zero Arguments)**:
   ```text
   ALBUM_PAGES_CROP_MARKS ()
   ```
   Uses standard dimensions: `0.1 mm` line width, `1.0 mm` horizontal padding, `1.0 mm` vertical padding, `5.0 mm` arm length, printed in solid `black`.

2. **Custom Color with Default Dimensions (One Color Argument)**:
   ```text
   ALBUM_PAGES_CROP_MARKS ("#1E4A7A")
   ALBUM_PAGES_CROP_MARKS (forestgreen)
   ```
   Maintains all standard dimensions while rendering the corner crosses in your chosen color.

3. **Disabling Crop Marks on Later Pages**:
   ```text
   NATIVE_PAGE_CROP_MARKS (none)
   ```
   Turns off crop marks on subsequent pages, leaving a clean page perimeter.

4. **Custom Dimensions with Optional Color (1 to 5 Arguments)**:
   - **1 number**: Custom line width in mm (e.g. `ALBUM_PAGES_CROP_MARKS (0.4)`).
   - **2 numbers**: Custom line width and horizontal padding in mm (e.g. `ALBUM_PAGES_CROP_MARKS (0.2 7.0)`).
   - **3 numbers**: Line width, horizontal padding, and vertical padding in mm (e.g. `ALBUM_PAGES_CROP_MARKS (0.2 7.0 8.0)`).
   - **4 numbers**: Line width, horizontal padding, vertical padding, and arm length in mm (e.g. `ALBUM_PAGES_CROP_MARKS (0.25 2.0 4.0 5.0)`).
   - **With Color**: Append any valid color name or hex code to any of the above:
     ```text
     ALBUM_PAGES_CROP_MARKS (0.25 8.0 9.0 3.0 #1E4A7A)
     ALBUM_PAGES_CROP_MARKS (0.3 8.0 9.0 2.5 "forestgreen")
     ```

---

## 2. Technical Reference & Command Grammar

### Grammar & Argument Variations

```text
ALBUM_PAGES_CROP_MARKS [ ( [ line_width [ h_pad [ v_pad [ arms ] ] ] ] [ color ] ) ]
NATIVE_PAGES_CROP_MARKS [ ( [ line_width [ h_pad [ v_pad [ arms ] ] ] ] [ color ] ) ]
NATIVE_PAGE_CROP_MARKS [ ( [ line_width [ h_pad [ v_pad [ arms ] ] ] ] [ color ] ) ]
```

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `line_width` | Float (mm) | `0.1` | Stroke thickness of the crosshair lines. Must be finite and non-negative. |
| `h_pad` | Float (mm) | `1.0` | Distance from the left/right physical page boundary to the crosshair center. |
| `v_pad` | Float (mm) | `1.0` | Distance from the bottom/top physical page boundary to the crosshair center. |
| `arms` | Float (mm) | `5.0` | Length of each of the 4 crosshair arms extending from the center point. |
| `color` | Color / String | `black` | Stroke color: `#RRGGBB`, `#RGB`, bare hex, `rgb(...)`, or SVG named color. |

### Corner Coordinate Geometry

Crop mark crosshairs are positioned at the 4 corners of the physical page:
- Bottom-Left: `(h_pad - line_width / 2, v_pad - line_width / 2)`
- Top-Left: `(h_pad - line_width / 2, page_height - v_pad + line_width / 2)`
- Top-Right: `(page_width - h_pad + line_width / 2, page_height - v_pad + line_width / 2)`
- Bottom-Right: `(page_width - h_pad + line_width / 2, v_pad - line_width / 2)`

Each crosshair consists of two orthogonal line segments (horizontal and vertical) drawn with butt line caps (`linecap = 0`). Outer arms extending outside the MediaBox boundary are permitted without altering page dimensions or clipping interior content.

### Multi-Page & Per-Page Override

In multi-page albums, crop marks can be configured globally at document setup or dynamically modified on individual page breaks:
- In native `.stampalbum`: `NATIVE_PAGE_BREAK ()` followed by `NATIVE_PAGE_CROP_MARKS (...)`.
- In raw AlbumEasy: `PAGE_START` followed by `ALBUM_PAGES_CROP_MARKS (...)`.

---

## 3. Fixture Variations Inventory

| Variation | Command Signature | Key Feature Demonstrated |
| :--- | :--- | :--- |
| `crop-default` | `ALBUM_PAGES_CROP_MARKS ()` | Zero arguments; standard 0.1mm width, 1.0mm pad, 5.0mm arms in black |
| `crop-custom` | `ALBUM_PAGES_CROP_MARKS (0.25 2.0 4.0 5.0)` | 4 explicit numeric values with asymmetrical vertical offset |
| `crop-colour` | `ALBUM_PAGES_CROP_MARKS (0.25 8.0 9.0 3.0 #1E4A7A)` | 4 numeric values plus `#1E4A7A` Navy archival color |
| `crop-disabled` | `ALBUM_PAGES_CROP_MARKS (none)` | Explicitly disabled crop marks; clean page periphery |
| `crop-landscape` | `ALBUM_PAGES_CROP_MARKS (0.3 8.0 9.0 2.5 "forestgreen")` | Landscape US Letter (279.4 × 215.9 mm) with SVG named color |
| `crop-one` | `ALBUM_PAGES_CROP_MARKS (0.4 "darkred")` | 1 numeric argument (0.4mm heavy stroke) with dark red color |
| `crop-two` | `ALBUM_PAGES_CROP_MARKS (0.2 7.0 "purple")` | 2 numeric arguments (0.2mm width, 7.0mm h-pad) in purple |
| `crop-three` | `ALBUM_PAGES_CROP_MARKS (0.2 7.0 8.0 "darkorange")` | 3 numeric arguments (0.2mm width, 7.0mm h-pad, 8.0mm v-pad) in dark orange |
| `crop-zero` | `ALBUM_PAGES_CROP_MARKS (0 0 0 0)` | 4 zero values; hairline non-rendering verification baseline |
| `all-in-one-compare` | Multi-page (9 pages) | All 9 variations in sequence demonstrating per-page transitions |
