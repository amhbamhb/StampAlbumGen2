# Native PDF Color Formats & Extended Palette Guide (`color-formats-v1`)

This fixture family defines, tests, and showcases comprehensive color support across the native PDF engine in StampAlbumGen2.

---

## 1. Stamp Collector Guide (User-Friendly Overview)

Stamp albums often benefit from subtle, authentic, or thematic colors to complement stamps from different eras and countries. For instance, Victorian British stamps look stunning against deep navy or burgundy borders, while classic French issues pair well with subtle slate or sepia frames.

The StampAlbumGen2 native PDF engine supports all popular digital color representations:

### Supported Color Formats

1. **Standard Named Colors**:
   Everyday color names such as `black`, `navy`, `forestgreen`, `darkred`, `midnightblue`, `crimson`, `darkgoldenrod`, `ivory`, `white`, and `lightgray`. You can write them in lowercase, uppercase, or mixed case:

   ```text
   COLOR_ALBUM_BORDER (forestgreen)
   COLOR_STAMP_BORDER (midnightblue)
   NATIVE_PAGE_TITLE (TB 15 "SWISS CANTONAL ISSUES" 8 forestgreen)
   ```

2. **6-Digit Hexadecimal Colors (`#RRGGBB`)**:
   Exact digital color codes commonly used on the web and graphic design software. The `#` prefix is supported both unquoted and quoted:

   ```text
   COLOR_ALBUM_BORDER (#1E4A7A)
   NATIVE_PAGE_TITLE (TB 15 "US COMMEMORATIVES" 8 #1E4A7A)
   NATIVE_STAMP_FRAME (0.6 #2B5B84 2)
   ```

3. **3-Digit Shorthand Hexadecimal Colors (`#RGB`)**:
   Short 3-character shortcuts where each character automatically doubles (for example, `#036` expands to `#003366`, and `#F80` expands to `#FF8800`):

   ```text
   COLOR_ALBUM_BORDER (#F80)
   NATIVE_PAGE_TITLE (TB 15 "SPECIAL EXHIBIT" 8 #F80)
   ```

4. **Bare Hexadecimal Colors (Without `#`)**:
   For clean scripts without special symbols, bare 6-digit or 3-digit hex strings are directly supported:

   ```text
   COLOR_ALBUM_BORDER (2E8B57)
   NATIVE_PAGE_TITLE (TB 15 "IRISH DEFINITIVES" 8 2E8B57)
   ```

5. **RGB Functional Syntax (`rgb(R, G, B)`)**:
   Specifies Red, Green, and Blue light values from `0` to `255`:

   ```text
   COLOR_ALBUM_BORDER ("rgb(178, 34, 34)")
   NATIVE_PAGE_TITLE (TB 15 "DANISH COMMEMORATIVES" 8 "rgb(178, 34, 34)")
   ```

6. **RGB Percentage Syntax (`rgb(R%, G%, B%)`)**:
   Specifies Red, Green, and Blue intensity from `0%` to `100%`:
   ```text
   COLOR_ALBUM_BORDER ("rgb(50%, 0%, 50%)")
   NATIVE_PAGE_TITLE (TB 15 "IMPERIAL JUBILEE" 8 "rgb(50%, 0%, 50%)")
   ```

### Thicker Deluxe Borders for Color Visibility

To make colors rich, prominent, and clearly visible on printed album pages and digital viewers, borders can be configured with bold outer rules:

```text
ALBUM_PAGES_BORDER (1.5 0.5 0.0 1.5)
```

This produces a stately 1.5 mm outer border and a crisp 0.5 mm inner accent rule in the chosen color.

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

| Scope                   | Command / Setting                                   | Accepted Syntax                                           | Default |
| :---------------------- | :-------------------------------------------------- | :-------------------------------------------------------- | :------ |
| **Page Border**         | `COLOR_ALBUM_BORDER`, `COLOUR_ALBUM_BORDER`         | `#RGB`, `#RRGGBB`, `RGB`, `RRGGBB`, `rgb(...)`, SVG names | `black` |
| **Page Title**          | `NATIVE_PAGE_TITLE` (optional 5th arg)              | All supported color forms                                 | `black` |
| **Page Header**         | `NATIVE_PAGE_HEADER` (optional 7th arg)             | All supported color forms                                 | `black` |
| **Decorative Border**   | `COLOR_ALBUM_DECORATIVE_BORDER`, `COLOUR_...`       | All supported forms                                       | `black` |
| **Page Header Number**  | `NATIVE_PAGE_HEADER_NUMBER` (optional 9th arg)      | All supported forms                                       | `black` |
| **Page Footer Number**  | `NATIVE_PAGE_FOOTER_NUMBER` (optional 9th arg)      | All supported forms                                       | `black` |
| **Margin Side Text**    | `NATIVE_PAGE_MARGIN_TEXT`                           | All supported forms                                       | `black` |
| **Stamp Mount Frames**  | `NATIVE_STAMP_FRAME`, `COLOR_STAMP_BORDER`          | All supported forms                                       | `black` |
| **Stamp Backgrounds**   | `NATIVE_STAMP_BACKGROUND`, `COLOR_STAMP_BACKGROUND` | All supported forms + `"none"`                            | `none`  |
| **Stamp Inner Borders** | `NATIVE_STAMP_INNER_BORDER`                         | All supported forms                                       | `black` |
| **Stamp Headings**      | `NATIVE_STAMP_HEADING`, `COLOR_STAMP_HEADING`       | All supported forms                                       | `black` |
| **Stamp Footer Text**   | `NATIVE_STAMP_FOOTER_TEXT`                          | All supported forms                                       | `black` |
| **Configuration File**  | `config.toml` (`color_album_border`, etc.)          | All supported forms in TOML strings                       | `black` |

### Error Handling & Validation Rules

- **Empty Strings**: Rejected with `"Color string must not be empty"`.
- **Invalid Hex Digits**: Rejected with `"Invalid hexadecimal color: contains non-hex characters"`.
- **Invalid Hex Length**: Only lengths of 3, 4, 6, or 8 hex digits are accepted. Non-matching lengths report clear errors without echoing private values.
- **Transparency / Alpha**: Values with alpha != 255 (or < 100%) return `"Transparency and alpha channels are not supported; colors must be fully opaque"`.
- **Non-Color Keywords**: `none` (except in background where handled explicitly), `transparent`, `currentcolor`, and `inherit` return structured validation errors.

---

## 3. Fixture Suite Variations & Gallery Artifacts

| Variation                | Description                                              | Primary Palette                                           | Rendered Artifacts                                                           |
| :----------------------- | :------------------------------------------------------- | :-------------------------------------------------------- | :--------------------------------------------------------------------------- |
| `hex-6digit`             | 6-digit hex format                                       | `#1E4A7A`, `#2B5B84`, `#8B0000`, `#006400`                | `hex-6digit-native.pdf`, `hex-6digit-raw-native.pdf`                         |
| `hex-3digit`             | 3-digit shorthand hex                                    | `#F80`, `#036`, `#C00`, `#080`                            | `hex-3digit-native.pdf`, `hex-3digit-raw-native.pdf`                         |
| `bare-hex`               | Bare hex without `#`                                     | `2E8B57`, `1E4A7A`, `800020`, `4B0082`                    | `bare-hex-native.pdf`, `bare-hex-raw-native.pdf`                             |
| `rgb-integer`            | `rgb(r, g, b)` (0–255)                                   | `rgb(178, 34, 34)`, `rgb(30, 74, 122)`                    | `rgb-integer-native.pdf`, `rgb-integer-raw-native.pdf`                       |
| `rgb-percent`            | `rgb(r%, g%, b%)` (0–100%)                               | `rgb(50%, 0%, 50%)`, `rgb(12%, 29%, 48%)`                 | `rgb-percent-native.pdf`, `rgb-percent-raw-native.pdf`                       |
| `svg-named`              | Rich SVG named colors                                    | `forestgreen`, `midnightblue`, `crimson`                  | `svg-named-native.pdf`, `svg-named-raw-native.pdf`                           |
| **`all-in-one-compare`** | **Comprehensive 6-page suite exhibiting all variations** | **Complete palette matrix (one distinct color per page)** | **`all-in-one-compare-native.pdf`**, **`all-in-one-compare-raw-native.pdf`** |

### All-In-One Showcase Page Breakdown

Every page in `all-in-one-compare` exhibits a distinct color format on the outer page border and stamp mount frames:

- **Page 1: 6-Digit Hexadecimal Colors (`#RRGGBB`)**:
  - Border: `#1E4A7A` (Prussian Navy Blue, 1.5 mm bold rule).
  - Mount Frame: `#2B5B84` (Slate Blue).
  - Accents: `#8B0000` (Dark Red), `#006400` (Dark Green).
  - Testing case-insensitivity and 24-bit direct color mapping.
- **Page 2: 3-Digit Shorthand Hex Colors (`#RGB`)**:
  - Border: `#F80` (Bright Orange, expands to `#FF8800`).
  - Mount Frame: `#036` (Deep Navy, expands to `#003366`).
  - Accents: `#C00` (Red), `#080` (Green).
  - Testing shorthand duplication of hex nibbles.
- **Page 3: Bare Hexadecimal Colors (Without Hash Sign)**:
  - Border: `2E8B57` (Sea Green bare hex).
  - Mount Frame: `800020` (Burgundy bare hex).
  - Accents: `1E4A7A` (Prussian Blue), `4B0082` (Indigo).
  - Testing direct bare hexadecimal string parsing.
- **Page 4: Functional Integer RGB Syntax (`rgb(R, G, B)`)**:
  - Border: `rgb(178, 34, 34)` (Firebrick Red).
  - Mount Frame: `rgb(30, 74, 122)` (Cobalt Blue).
  - Accents: `rgb(34, 139, 34)` (Forest Green), `rgb(218, 165, 32)` (Goldenrod).
  - Testing 0–255 integer color calibration.
- **Page 5: Functional Percentage RGB Syntax (`rgb(R%, G%, B%)`)**:
  - Border: `rgb(50%, 0%, 50%)` (Deep Imperial Purple).
  - Mount Frame: `rgb(12%, 29%, 48%)` (Navy Blue).
  - Accents: `rgb(80%, 15%, 25%)` (Carmine Wine), `rgb(85%, 65%, 13%)` (Amber Gold).
  - Testing proportional 0%–100% color mixing.
- **Page 6: SVG and CSS Standard Named Colors**:
  - Border: `forestgreen` (Deep Forest Green named color).
  - Mount Frame: `midnightblue` (Midnight Blue named color).
  - Accents: `crimson`, `darkmagenta`.
  - Testing English color keywords and comprehensive palette resolution.
