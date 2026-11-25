# Native PDF Color Formats & Extended Palette Guide (`color-formats-v1`)

This fixture family defines, tests, and showcases comprehensive color support across the native PDF engine in StampAlbumGen2.

---

## 1. Stamp Collector Guide (User-Friendly Overview)

Stamp albums often benefit from subtle, authentic, or thematic colors to complement stamps from different eras and countries. For instance, Victorian British stamps look stunning against deep navy or burgundy borders, while classic French issues pair well with subtle slate or sepia frames.

StampAlbumGen2 native PDF engine supports all popular digital color representations:

### Supported Color Formats

1. **Standard Named Colours**:
   Everyday color names such as `black`, `navy`, `forestgreen`, `darkred`, `midnightblue`, `crimson`, `darkgoldenrod`, `ivory`, `white`, and `lightgray`. You can write them in lowercase, uppercase, or mixed case.
   ```text
   COLOUR_ALBUM_BORDER (midnightblue)
   COLOUR_STAMP_BORDER (forestgreen)
   ```

2. **6-Digit Hexadecimal Colors (`#RRGGBB`)**:
   Exact digital color codes commonly used on the web and graphic design software. The `#` prefix is supported both unquoted and quoted.
   ```text
   COLOUR_ALBUM_BORDER (#1E4A7A)
   COLOUR_STAMP_BORDER ("#2B5B84")
   ```

3. **3-Digit Shorthand Hexadecimal Colors (`#RGB`)**:
   Short 3-character shortcuts where each character automatically doubles (for example, `#036` expands to `#003366`, and `#F80` expands to `#FF8800`).
   ```text
   COLOUR_ALBUM_BORDER (#036)
   COLOUR_STAMP_BORDER (#369)
   ```

4. **Bare Hexadecimal Colors (Without `#`)**:
   For clean scripts without special symbols, bare 6-digit or 3-digit hex strings are directly supported:
   ```text
   COLOUR_ALBUM_BORDER (1E4A7A)
   COLOUR_STAMP_BORDER (2E8B57)
   ```

5. **RGB Functional Syntax (`rgb(R, G, B)`)**:
   Specifies Red, Green, and Blue light values from `0` to `255`:
   ```text
   COLOUR_ALBUM_BORDER ("rgb(30, 74, 122)")
   COLOUR_STAMP_BORDER ("rgb(34, 139, 34)")
   ```

6. **RGB Percentage Syntax (`rgb(R%, G%, B%)`)**:
   Specifies Red, Green, and Blue intensity from `0%` to `100%`:
   ```text
   COLOUR_ALBUM_BORDER ("rgb(12%, 29%, 48%)")
   COLOUR_STAMP_BORDER ("rgb(13%, 55%, 13%)")
   ```

### Transparency & Print Safety

Stamp album printing requires opaque, solid inks. Semi-transparent colors or alpha channels are intentionally disallowed so your albums print with crisp lines and authentic colors without unexpected transparency blending on paper.

---

## 2. Technical Reference & Grammar

### Common Color Module Architecture

All color resolution in StampAlbumGen2 converges in a centralized location:
- Source: [`src/native_pdf/colour.rs`](file:///media/ray/2026-01-11/Rust/Rust/StampAlbumGen2/src/native_pdf/colour.rs)
- Type: `Color` (with backward-compatible alias `NamedColour = Color`)
- Internal representation: `[u8; 3]` (8-bit RGB components)
- Output representation: normalized float `[f64; 3]` for PDF rendering engines (`printpdf`).

### Command & Configuration Integration

The unified color parser handles all native PDF color commands and configuration keys:

| Scope | Command / Setting | Accepted Syntax |
| :--- | :--- | :--- |
| **Page Border** | `COLOUR_ALBUM_BORDER`, `COLOR_ALBUM_BORDER`, `PAGE_BORDER` | `#RGB`, `#RRGGBB`, `RGB`, `RRGGBB`, `rgb(...)`, SVG names |
| **Decorative Border** | `COLOUR_ALBUM_DECORATIVE_BORDER`, `COLOR_ALBUM_DECORATIVE_BORDER` | All supported forms |
| **Page Header/Footer** | `COLOUR_ALBUM_HEADER`, `COLOUR_ALBUM_FOOTER`, `NATIVE_PAGE_TITLE` | All supported forms |
| **Margins & Dates** | `COLOUR_ALBUM_MARGIN_TXT`, `ALBUM_PAGES_HEADER_DATE`, etc. | All supported forms |
| **Crop Marks** | `ALBUM_PAGES_CROP_MARKS` | All supported forms |
| **Stamp Boxes & Frames**| `COLOUR_STAMP_BORDER`, `STAMP_BORDER_STYLE`, `NATIVE_STAMP_FRAME` | All supported forms |
| **Stamp Backgrounds** | `COLOUR_STAMP_BACKGROUND`, `STAMP_BACKGROUND` | All supported forms + `"none"` |
| **Stamp Inner Borders**| `COLOUR_STAMP_INNER_BORDER`, `STAMP_INNER_BORDER` | All supported forms |
| **Stamp Headings** | `COLOUR_STAMP_HEADING`, `NATIVE_STAMP_HEADING` | All supported forms |
| **Stamp Text & Labels** | `COLOUR_STAMP_TEXT`, `NATIVE_STAMP_FOOTER_TEXT`, `STAMP_SIDE_LABELS` | All supported forms |
| **Configuration File** | `config.toml` (`[native_pdf] color_album_border`, `guide_colour`, etc.) | All supported forms in TOML strings |

### Error Handling & Validation Rules

- **Empty Strings**: Rejected with `"Colour string must not be empty"`.
- **Invalid Hex Digits**: Rejected with `"Invalid hexadecimal colour: contains non-hex characters"`.
- **Invalid Hex Length**: Only lengths of 3, 4, 6, or 8 hex digits are accepted. Non-matching lengths report clear errors without echoing private values.
- **Transparency / Alpha**: Values with alpha != 255 (or < 100%) return `"Transparency and alpha channels are not supported; colours must be fully opaque"`.
- **Non-Color Keywords**: `none` (except in background where handled explicitly), `transparent`, `currentcolor`, and `inherit` return structured validation errors.

---

## 3. Fixture Suite Variations & Gallery Artifacts

| Variation | Description | Primary Palette | Rendered Artifacts |
| :--- | :--- | :--- | :--- |
| `hex-6digit` | 6-digit hex format | `#1E4A7A`, `#2B5B84`, `#8B0000`, `#006400` | `hex-6digit-native.pdf`, `hex-6digit-raw-native.pdf` |
| `hex-3digit` | 3-digit shorthand hex | `#036`, `#369`, `#900`, `#080` | `hex-3digit-native.pdf`, `hex-3digit-raw-native.pdf` |
| `bare-hex` | Bare hex without `#` | `1E4A7A`, `2E8B57`, `800020`, `4B0082` | `bare-hex-native.pdf`, `bare-hex-raw-native.pdf` |
| `rgb-integer` | `rgb(r, g, b)` (0–255) | `rgb(30, 74, 122)`, `rgb(34, 139, 34)` | `rgb-integer-native.pdf`, `rgb-integer-raw-native.pdf` |
| `rgb-percent` | `rgb(r%, g%, b%)` (0–100%) | `rgb(12%, 29%, 48%)`, `rgb(13%, 55%, 13%)` | `rgb-percent-native.pdf`, `rgb-percent-raw-native.pdf` |
| `svg-named` | Rich SVG named colours | `midnightblue`, `forestgreen`, `crimson` | `svg-named-native.pdf`, `svg-named-raw-native.pdf` |
| **`all-in-one-compare`** | **Comprehensive 6-page suite exhibiting all variations** | **Complete palette matrix** | **`all-in-one-compare-native.pdf`**, **`all-in-one-compare-raw-native.pdf`** |
