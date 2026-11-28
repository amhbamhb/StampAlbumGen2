# Page Border Specification & Fixtures (border-v1)

This fixture demonstrates and validates the AlbumEasy command `ALBUM_PAGES_BORDER`, controlling decorative page border framing (single, double, and triple concentric lines, line widths, and separation gaps), alongside native vector engine rendering and layout coordinate insets.

---

## Collector's Guide: Understanding Album Page Borders & Framing

Page borders provide the framing structure for an album page. In classical and modern philately, the page border defines the visual boundary of the exhibit, separating the stamps and annotations from the binder margins.

### Why Page Borders Matter to Collectors
- **Classic Philatelic Framing (Double Border)**: The most traditional stamp album border consists of a bolder outer line (e.g. 0.5 mm) paired with a delicate inner hairline (e.g. 0.1 mm), separated by a neat white-space channel (e.g. 1.25 mm). This timeless look gives pages an authoritative, archival appearance.
- **Single-Line Borders**: A crisp single rule (e.g. 0.25 mm) suits clean, modern, or minimalist page designs without distracting from colorful stamp designs.
- **Triple-Line Presentation Borders**: Triple concentric rules (e.g. 0.1 mm outer, 0.5 mm middle accent, 0.2 mm inner) provide an elaborate frame ideal for high-value commemorative issues, souvenir sheets, or title pages.
- **Asymmetric Weights & Accent Lines**: By adjusting individual line widths (outer, middle, inner), collectors can craft custom decorative styles—such as bold inner accent frames or prominent outer borders.
- **Border Gap Separation (`gap_mm`)**: Controls the distance between concentric strokes. A zero gap merges lines into contiguous compound ribbons, while a 1.0–1.5 mm gap creates classic double and triple frames.
- **Borderless & Omitted Pages**: Omitting `ALBUM_PAGES_BORDER` or setting widths to `0` cleanly disables border strokes, allowing full-bleed content or modern borderless presentation.

---

## Authentic Stamp Data & Image Provenance

To ensure real-world fidelity, the fixtures incorporate authentic scanned stamp images and metadata:
- **All-in-One Showcase** (`all-in-one-compare`): Features 4 authentic postage stamps across 4 pages in US Letter format (`215.9 × 279.4 mm`):
  - **San Marino**: `images/san-marino.jpg` (200 × 150 px, 35.0 × 26.0 mm frame) from San Marino archives (`Albums/San Marino`)
  - **New Zealand**: `images/new-zealand.jpg` (222 × 132 px, 36.0 × 22.0 mm frame) from New Zealand archives (`Albums/New Zealand`)
  - **Malta**: `images/malta.jpg` (139 × 220 px, 22.0 × 34.0 mm frame) from Malta archives (`Albums/Malta`)
  - **Sweden**: `images/sweden.jpg` (140 × 165 px, 23.0 × 27.0 mm frame) from Sweden archives (`Albums/Sweden`)
- **Single-Page Variations**: Feature `us-150.png` (52 × 29 mm frame) from US 1847 definitive archives (`tests/fixtures/albumeasy/stamp-pagination-v1/us-150.png`).

---

## Fixture Variations & Benchmarks

Each variation is authored in AlbumEasy TXT syntax (for authentic AlbumEasy v4.5.0 compilation), native `STAMPALBUM 1` syntax (for native vector engine rendering), and raw AlbumEasy format:

| Variation | Baseline TXT | AlbumEasy Ref PDF | Native Album (`.stampalbum`) | Raw Direct (`-raw.stampalbum`) | Rendered Native PDF | Border Configuration |
|---|---|---|---|---|---|---|
| **Single** | [single.txt](single.txt) | [single.reference.pdf](single.reference.pdf) | [single.stampalbum](single.stampalbum) | [single-raw.stampalbum](single-raw.stampalbum) | [single-native.pdf](single-native.pdf) / [raw](single-raw-native.pdf) | `(0.25 0 0 1.25)` — Single 0.25 mm stroke |
| **Double** | [double.txt](double.txt) | [double.reference.pdf](double.reference.pdf) | [double.stampalbum](double.stampalbum) | [double-raw.stampalbum](double-raw.stampalbum) | [double-native.pdf](double-native.pdf) / [raw](double-raw-native.pdf) | `(0.5 0.1 0 1.25)` — 0.5 mm outer, 0.1 mm inner |
| **Triple** | [triple.txt](triple.txt) | [triple.reference.pdf](triple.reference.pdf) | [triple.stampalbum](triple.stampalbum) | [triple-raw.stampalbum](triple-raw.stampalbum) | [triple-native.pdf](triple-native.pdf) / [raw](triple-raw-native.pdf) | `(0.1 0.5 0.2 1.25)` — 3 concentric lines |
| **Legacy Double** | [legacy-double.txt](legacy-double.txt) | [legacy-double.reference.pdf](legacy-double.reference.pdf) | [legacy-double.stampalbum](legacy-double.stampalbum) | [legacy-double-raw.stampalbum](legacy-double-raw.stampalbum) | [legacy-double-native.pdf](legacy-double-native.pdf) / [raw](legacy-double-raw-native.pdf) | `(0.5 0.1 1.25)` — 3-arg legacy syntax |
| **Disabled** | [disabled.txt](disabled.txt) | [disabled.reference.pdf](disabled.reference.pdf) | [disabled.stampalbum](disabled.stampalbum) | [disabled-raw.stampalbum](disabled-raw.stampalbum) | [disabled-native.pdf](disabled-native.pdf) / [raw](disabled-raw-native.pdf) | `(0 0 0 1.25)` — All line widths zero |
| **Omitted** | [omitted.txt](omitted.txt) | [omitted.reference.pdf](omitted.reference.pdf) | [omitted.stampalbum](omitted.stampalbum) | [omitted-raw.stampalbum](omitted-raw.stampalbum) | [omitted-native.pdf](omitted-native.pdf) / [raw](omitted-raw-native.pdf) | Command omitted (defaults to no border) |
| **Inner Only** | [inner-only.txt](inner-only.txt) | [inner-only.reference.pdf](inner-only.reference.pdf) | [inner-only.stampalbum](inner-only.stampalbum) | [inner-only-raw.stampalbum](inner-only-raw.stampalbum) | [inner-only-native.pdf](inner-only-native.pdf) / [raw](inner-only-raw-native.pdf) | `(0 0.3 0 1.25)` — Recessed single line |
| **Middle Only** | [middle-only.txt](middle-only.txt) | [middle-only.reference.pdf](middle-only.reference.pdf) | [middle-only.stampalbum](middle-only.stampalbum) | [middle-only-raw.stampalbum](middle-only-raw.stampalbum) | [middle-only-native.pdf](middle-only-native.pdf) / [raw](middle-only-raw-native.pdf) | `(0 0 0.4 1.25)` — Deeply recessed line |
| **Skipped Middle** | [skipped-middle.txt](skipped-middle.txt) | [skipped-middle.reference.pdf](skipped-middle.reference.pdf) | [skipped-middle.stampalbum](skipped-middle.stampalbum) | [skipped-middle-raw.stampalbum](skipped-middle-raw.stampalbum) | [skipped-middle-native.pdf](skipped-middle-native.pdf) / [raw](skipped-middle-raw-native.pdf) | `(0.5 0 0.2 1.25)` — Double gap between lines |
| **Zero Gap** | [zero-gap.txt](zero-gap.txt) | [zero-gap.reference.pdf](zero-gap.reference.pdf) | [zero-gap.stampalbum](zero-gap.stampalbum) | [zero-gap-raw.stampalbum](zero-gap-raw.stampalbum) | [zero-gap-native.pdf](zero-gap-native.pdf) / [raw](zero-gap-raw-native.pdf) | `(0.5 0.2 0.1 0)` — Contiguous multi-line |
| **All-in-One Showcase** | [all-in-one-compare.txt](all-in-one-compare.txt) | [all-in-one-compare.reference.pdf](all-in-one-compare.reference.pdf) | [all-in-one-compare.stampalbum](all-in-one-compare.stampalbum) | [all-in-one-compare-raw.stampalbum](all-in-one-compare-raw.stampalbum) | [all-in-one-compare-native.pdf](all-in-one-compare-native.pdf) / [raw](all-in-one-compare-raw-native.pdf) | 4-page US Letter sequence (double, triple, single, inner-only) |

---

## Technical Command Specifications

### 1. AlbumEasy & Native Commands: `ALBUM_PAGES_BORDER`, `NATIVE_PAGES_BORDER`, `NATIVE_PAGE_BORDER`
```text
ALBUM_PAGES_BORDER (outer middle inner gap [colour])
ALBUM_PAGES_BORDER (outer inner gap [colour])
NATIVE_PAGES_BORDER (outer middle inner gap [colour])
NATIVE_PAGE_BORDER (outer middle inner gap [colour])
```
- **Native Aliases**: `NATIVE_PAGES_BORDER` and `NATIVE_PAGE_BORDER` provide full syntax parity with `ALBUM_PAGES_BORDER`.
- **Optional Colour Parameter**: Accepts an optional trailing colour argument supporting all standard formats:
  - 6-digit hex (`#1B365D` or bare `1B365D`)
  - 3-digit shorthand hex (`#F80` or bare `F80`)
  - Functional integer/percentage RGB (`rgb(27, 54, 93)`)
  - SVG / CSS named colours (`navy`, `darkred`, `forestgreen`, `midnightblue`, etc.)
- **Dedicated Colour Commands**: `COLOUR_ALBUM_BORDER`, `COLOR_ALBUM_BORDER`, `NATIVE_PAGES_BORDER_COLOUR`, `NATIVE_PAGE_BORDER_COLOUR`, `NATIVE_BORDER_COLOUR` allow changing border color globally or per-page.
- **`outer`**: Stroke width (in millimeters) of the outermost border rectangle.
- **`middle`**: Stroke width (in millimeters) of the intermediate concentric rectangle (4-arg syntax only).
- **`inner`**: Stroke width (in millimeters) of the innermost rectangle. In 3-arg legacy syntax, this maps to the second stroke with `middle = 0.0`.
- **`gap`**: Distance (in millimeters) between successive concentric stroke boundaries.
- **Omission**: If `ALBUM_PAGES_BORDER` is omitted, no page border is drawn (`[0.0, 0.0, 0.0, 0.0]`).
- **Placement**: Document header preceding the first `PAGE_START`.

### 2. Native Engine Border Geometry & Insets
In the native PDF engine (`stamp_album_gen2::native_pdf`):
- Strokes are drawn along exact centerlines calculated as `inset + width / 2.0`.
- Content bounds inset calculation (`occupancy_mm`):
  Content reserves a gap only for enabled lines (`width > 0.0`), matching authentic AlbumEasy layout geometry:
  $$\text{occupancy} = w_{\text{outer}} + \sum_{i \in \{\text{inner}_1, \text{inner}_2\}, w_i > 0} (w_i + \text{gap})$$
- Zero-width strokes are completely skipped during vector path construction without emitting empty PDF drawing operators.

---

## Reference Provenance (AlbumEasy 4.5.0 Hashes)

All reference PDFs were generated using authentic AlbumEasy v4.5.0 on Windows:

| Input File | Input SHA-256 | Reference PDF | Reference SHA-256 |
|---|---|---|---|
| `single.txt` | `9acea4800f7feb7dc1f761058d781cb669b199c19c6af6a7d0645f7192ba41e1` | `single.reference.pdf` | `092bb4c6027c36fbcd0470b6561dcbcf9906b0bfc1e59d9cd0fcd6b5a495464f` |
| `double.txt` | `c0d06e5868ca0a2cc89845f0868a6091eef26dd965f79053baad50c347d24b15` | `double.reference.pdf` | `e68974628461e0e6391e4182672f6c2c0055859addbc04f33f83e50652ff1feb` |
| `triple.txt` | `0eb9c2ed1984d20202e8251111656d62602e8846b6760372ec6ae47a240ec60b` | `triple.reference.pdf` | `5e48c788fbdbaf43a077c419071aa5b8d8f73024e36b4ed15b2cdca693580b16` |
| `legacy-double.txt` | `3b4faec883fcdf19d2958e387c5ce4cc6508e6dc49146871182e731edfa25f1e` | `legacy-double.reference.pdf` | `e68974628461e0e6391e4182672f6c2c0055859addbc04f33f83e50652ff1feb` |
| `disabled.txt` | `ac9ed403189541e64ba9efd361dc324f88357d274932ec02483874a9eece2b7f` | `disabled.reference.pdf` | `667b6a3ec6c9d100fcc020dca76204b310236d233bf7f9ecec792cb3eeaeca16` |
| `omitted.txt` | `9406e3910831890b5efe151b64799fa2a73e2cece4565e2cfd40d766cf79455e` | `omitted.reference.pdf` | `f1f088a53946c12c17c50c196ada87f7975ab8655bd89db27307eff2e5520853` |
| `inner-only.txt` | `cf2b13deb2df0a94a877f684d20c8478d4ff507a5f267c47db713c001a3b60df` | `inner-only.reference.pdf` | `bf80f05411e1a4a143fa6730c96349b8e1a48acc9fa279712bd5dd3b8159c046` |
| `middle-only.txt` | `c8c2ef1d94a55f4c9173db61dd3e6ad0fc5569425300ab77e32f0faeadc4a004` | `middle-only.reference.pdf` | `c2044b440448ca88763d6340ba8f5044ee65b0155f6ebd1d48016d1615206cae` |
| `skipped-middle.txt` | `4cfccc72a8a48b98f6d61d78da32b72233294be92c19d7399ebf99bdb99def11` | `skipped-middle.reference.pdf` | `fc3e3415e1887bcb002928fc8f740b1107c241132d56719dfc5144c063bac50c` |
| `zero-gap.txt` | `ab8e0c59143c5345b1509c59b8e9a306e1cd7d981890fe30d0a8096ed2dac925` | `zero-gap.reference.pdf` | `bd3bd1170db7d025188618c32aea56eb9a8a7f52cc162ec72dcc79e5e675f475` |
| `all-in-one-compare.txt` | `f25a639c8571bc2c654b0541ed431322597b8b3feb1b70d3f6e9a3627b9a8dbb` | `all-in-one-compare.reference.pdf` | `ce35a3d3d252857bb6296639634ce8d3810dda5bce8b2c29fe421aa507f65c5f` |

Diagnostic input:
- `missing-gap.txt`: `98f6b77fdac83a4edb8c1a36a002d9322eb881131b58bf531d177d71935a85ba`

---

## Automated Layout Verification

Run the dedicated border test suite:

```sh
cargo test --test native_pdf_page_border
```

The test suite validates:
1. Exact cryptographic identity of frozen AlbumEasy 4.5.0 reference PDFs and text content.
2. Border stroke widths and rectangle coordinates matching reference geometry within 0.01 mm.
3. Content occupancy offsets and margin-to-glyph clearance.
4. Error handling for negative or malformed border arguments.
5. High-level rendering of all 10 single-page variations (`*-raw-native.pdf` and `*-native.pdf`).
6. High-level rendering of the 4-page showcase (`all-in-one-compare-native.pdf` and `all-in-one-compare-raw-native.pdf`) with US Letter dimensions.
