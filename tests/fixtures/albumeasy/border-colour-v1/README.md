# Border Colors & Tints Specification & Fixtures (border-colour-v1)

This fixture demonstrates and validates the AlbumEasy command `COLOUR_ALBUM_BORDER` (and `COLOR_ALBUM_BORDER`), native border color commands, and inline border color options in the native vector engine.

---

## Collector's Guide: Understanding Border Colors & Framing Tints

Color is one of the most powerful artistic tools available to a philatelist when designing custom album pages. While traditional pre-printed albums rely almost exclusively on plain black rules, custom page design allows collectors to harmonize the border with the cultural, historical, or visual character of their collection.

### Why Border Colors Matter to Collectors
- **Classic Archival Black (`black`)**: The standard default in philately. Clean, high contrast, and neutral—ensuring that the stamps themselves remain the primary visual focal point.
- **Deep Navy & Nautical Blues (`navy`, `darkblue`, `#1B365D`)**: Imparts an authoritative, dignified archival tone. Ideal for British Commonwealth, maritime issues, polar expeditions, and naval philately.
- **Imperial Crimson & Dark Reds (`darkred`, `#8B0000`)**: Adds regal warmth and prominence to royalty issues, coronation sets, holiday stamps, and high-value commemorative sheets.
- **Heritage Forest Greens (`forestgreen`, `darkgreen`, `#1C4D25`)**: Creates an organic, balanced feel well suited for flora and fauna issues, national parks, agricultural commemoratives, and environmental topicals.
- **Subtle Light Grey (`lightgrey`, `#D3D3D3`)**: Understated and modern. Provides subtle framing without overpowering delicate nineteenth-century pastel printings or faded classics.
- **Hidden White Borders (`white`)**: Invisible to the eye on white album stock, this allows collectors to maintain strict internal margin clearance and spacing grids without displaying a visible line.
- **Per-Page & Color Overrides**: In the modern native engine, collectors can specify different border colors for title pages, exhibit sections, or individual countries within a single unified album file.

---

## Authentic Stamp Data & Image Provenance

To ensure real-world fidelity, the fixtures incorporate authentic scanned stamp images and metadata:
- **San Marino**: `images/san-marino.jpg` (200 × 150 px, 35.0 × 26.0 mm frame) from San Marino archives (`Albums/San Marino`)
- **New Zealand**: `images/new-zealand.jpg` (222 × 132 px, 36.0 × 22.0 mm frame) from New Zealand archives (`Albums/New Zealand`)
- **Malta**: `images/malta.jpg` (139 × 220 px, 22.0 × 34.0 mm frame) from Malta archives (`Albums/Malta`)
- **Sweden**: `images/sweden.jpg` (140 × 165 px, 23.0 × 27.0 mm frame) from Sweden archives (`Albums/Sweden`)
- **United States**: `us-150.png` (52 × 29 mm frame) from US 1847 definitive archives (`tests/fixtures/albumeasy/stamp-pagination-v1/us-150.png`).

---

## Fixture Variations & Benchmarks

Each variation is authored in AlbumEasy TXT syntax, native `STAMPALBUM 1` syntax, and raw AlbumEasy format:

| Variation | Baseline TXT | AlbumEasy Ref PDF | Native Album (`.stampalbum`) | Raw Direct (`-raw.stampalbum`) | Rendered Native PDF | Border Color Specification |
|---|---|---|---|---|---|---|
| **Default** | [default.txt](default.txt) | [default.pdf](default.pdf) | [default.stampalbum](default.stampalbum) | [default-raw.stampalbum](default-raw.stampalbum) | [default-native.pdf](default-native.pdf) / [raw](default-raw-native.pdf) | Default black border rule |
| **Named** | [named.txt](named.txt) | [named.pdf](named.pdf) | [named.stampalbum](named.stampalbum) | [named-raw.stampalbum](named-raw.stampalbum) | [named-native.pdf](named-native.pdf) / [raw](named-raw-native.pdf) | Case-insensitive named color `DaRkBlUe` |
| **Alias** | [alias.txt](alias.txt) | [alias.pdf](alias.pdf) | [alias.stampalbum](alias.stampalbum) | [alias-raw.stampalbum](alias-raw.stampalbum) | [alias-native.pdf](alias-native.pdf) / [raw](alias-raw-native.pdf) | American spelling alias `COLOR_ALBUM_BORDER (green)` |
| **Light Grey** | [light-grey.txt](light-grey.txt) | [light-grey.pdf](light-grey.pdf) | [light-grey.stampalbum](light-grey.stampalbum) | [light-grey-raw.stampalbum](light-grey-raw.stampalbum) | [light-grey-native.pdf](light-grey-native.pdf) / [raw](light-grey-raw-native.pdf) | Named SVG gray tint `lightgrey` |
| **White** | [white.txt](white.txt) | [white.pdf](white.pdf) | [white.stampalbum](white.stampalbum) | [white-raw.stampalbum](white-raw.stampalbum) | [white-native.pdf](white-native.pdf) / [raw](white-raw-native.pdf) | Invisible structural rule `white` |
| **Global Last** | [global-last.txt](global-last.txt) | [global-last.pdf](global-last.pdf) | [global-last.stampalbum](global-last.stampalbum) | [global-last-raw.stampalbum](global-last-raw.stampalbum) | [global-last-native.pdf](global-last-native.pdf) / [raw](global-last-raw-native.pdf) | Multi-page sequence demonstrating initial red and navy override |
| **Hex** | [hex.txt](hex.txt) | — | [hex.stampalbum](hex.stampalbum) | [hex-raw.stampalbum](hex-raw.stampalbum) | [hex-native.pdf](hex-native.pdf) / [raw](hex-raw-native.pdf) | Modern 6-digit hex string `"#1E4A7A"` |
| **Short Hex** | [short-hex.txt](short-hex.txt) | — | [short-hex.stampalbum](short-hex.stampalbum) | [short-hex-raw.stampalbum](short-hex-raw.stampalbum) | [short-hex-native.pdf](short-hex-native.pdf) / [raw](short-hex-raw-native.pdf) | Modern 3-digit hex string `"#F80"` |
| **Unquoted Hex**| [unquoted-hex.txt](unquoted-hex.txt) | — | [unquoted-hex.stampalbum](unquoted-hex.stampalbum) | [unquoted-hex-raw.stampalbum](unquoted-hex-raw.stampalbum) | [unquoted-hex-native.pdf](unquoted-hex-native.pdf) / [raw](unquoted-hex-raw-native.pdf) | Unquoted bare hex token `#1E4A7A` |
| **All-in-One Showcase** | [all-in-one-compare.txt](all-in-one-compare.txt) | — | [all-in-one-compare.stampalbum](all-in-one-compare.stampalbum) | [all-in-one-compare-raw.stampalbum](all-in-one-compare-raw.stampalbum) | [all-in-one-compare-native.pdf](all-in-one-compare-native.pdf) / [raw](all-in-one-compare-raw-native.pdf) | 4-page US Letter sequence (Navy, Crimson, Forest Green, Purple) |

---

## Technical Command Specifications

### 1. AlbumEasy & Native Border Colour Commands
```text
COLOUR_ALBUM_BORDER (colour_name)
COLOR_ALBUM_BORDER (colour_name)
NATIVE_PAGES_BORDER_COLOUR (colour_name)
NATIVE_PAGE_BORDER_COLOUR (colour_name)
NATIVE_BORDER_COLOUR (colour_name)
```
- **Spelling Support**: Both British (`COLOUR`) and American (`COLOR`) spellings are fully recognized.
- **Native Aliases**: `NATIVE_PAGES_BORDER_COLOUR`, `NATIVE_PAGE_BORDER_COLOUR`, and `NATIVE_BORDER_COLOUR` provide standard naming parity.
- **Color Formats Accepted**:
  - SVG / CSS named colors (`black`, `navy`, `darkred`, `forestgreen`, `lightgrey`, etc., case-insensitive)
  - 6-digit hex (`#1B365D` or bare `1B365D`)
  - 3-digit shorthand hex (`#F80` or bare `F80`)
  - Functional integer/percentage RGB (`rgb(27, 54, 93)`)

### 2. Inline Border Color Parameter
In `ALBUM_PAGES_BORDER` and `NATIVE_PAGES_BORDER`:
```text
NATIVE_PAGES_BORDER (outer middle inner gap colour)
NATIVE_PAGE_BORDER (outer inner gap colour)
```
Color can be passed directly as an optional trailing parameter, eliminating the need for a separate color command.

---

## Automated Layout Verification

Run the dedicated border color test suite:

```sh
cargo test --test native_pdf_border_colour
```

The test suite validates:
1. Exact cryptographic identity of frozen AlbumEasy 4.5.0 reference PDFs and text content.
2. Exact parsed RGB float values for SVG named colors against reference tolerances.
3. Validation and rejection of malformed or invalid color names without private value leakage.
4. Rendering of all individual native and raw `.stampalbum` files (`*-native.pdf` and `*-raw-native.pdf`).
5. Multi-page rendering and US Letter dimensions for `all-in-one-compare-native.pdf`.
