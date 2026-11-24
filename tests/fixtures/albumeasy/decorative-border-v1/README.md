# Decorative Page Border Specification & Fixtures (decorative-border-v1)

This fixture demonstrates and validates the AlbumEasy command `ALBUM_PAGES_DECORATIVE_BORDER`, controlling ornate page borders comprised of corner and margin ornaments, scaling factors, padding displacements, connecting multi-line frames, and end-point insets.

---

## Collector's Guide: Understanding Decorative Borders & Page Ornaments

In classical 19th and early 20th-century philately—such as the iconic Scott, Schaubek, and Lighthouse vintage albums—pages were frequently framed with elegant engraved page ornaments, flourishes, floral corners, and ornate column rules. These decorative elements lend stamp exhibits a sense of historical gravitas, period authenticity, and artistic beauty.

### Why Decorative Borders Matter to Collectors
- **Archival Exhibit Aesthetics**: Decorative borders transform a modern plain sheet into a period-appropriate vintage showcase, ideal for classic 19th-century issues, imperial stamps, or commemorative series.
- **Corner Ornaments (`IMAGE_CORNER`)**: Symmetrical decorative corner vignettes (swirls, acanthus leaves, classical scrollwork) anchor the page corners. The user provides a single PNG designed for the top-left corner; the software automatically rotates and mirrors it for the top-right, bottom-right, and bottom-left corners.
- **Margin & Edge Ornaments (`IMAGE_EDGE_*`)**: Mid-page accents placed centered along the top, bottom, left, and right margins. Top ornaments automatically mirror to the bottom, and left ornaments automatically mirror to the right.
- **Connecting Frame Lines (`BORDER_LINES`)**: Up to three concentric rule lines (outer, middle, inner) that span between ornaments.
- **Overlap Prevention (`BORDER_END_OFFSETS`)**: Keeps connecting frame lines from colliding with or obscuring the delicate artwork of corner ornaments.
- **Fine Adjustment Paddings (`IMAGE_CORNER_PAD`, `IMAGE_EDGE_PAD`, `BORDER_PAD`)**: Allows fine-tuning the placement of ornaments and rules outward or inward in millimetres.

---

## Border Definition File Grammar & Commands

A decorative border is specified by a separate definition file (e.g. `FancyBorder.txt` or `corner.def`), invoked in the album via:
```text
ALBUM_PAGES_DECORATIVE_BORDER ("FancyBorder.txt")
```

Inside the border definition file, the following directives are supported:

| Command | Arguments | Description |
|---|---|---|
| `IMAGE_SCALE` | `(scale_factor)` | Scaling factor applied to all corner and edge images. Many scanned ornaments are digitised at 300 or 600 dpi; scale factors such as `0.24` or `0.12` bring them into standard 72 dpi coordinate space. |
| `IMAGE_CORNER` | `("filename.png")` | Transparent PNG corner ornament. Base image is oriented for the **top-left** corner; the engine automatically mirrors/rotates it across all four page corners. |
| `IMAGE_CORNER_PAD` | `(h_pad v_pad)` | Horizontal and vertical displacement (mm) shifting corner ornaments outward or inward relative to the margin boundary. |
| `IMAGE_EDGE_TOP` | `("filename.png")` | Top margin ornament, placed at horizontal center of the top margin. |
| `IMAGE_EDGE_BOTTOM` | `("filename.png")` | Bottom margin ornament. If omitted, mirrors `IMAGE_EDGE_TOP`. |
| `IMAGE_EDGE_LEFT` | `("filename.png")` | Left margin ornament, placed at vertical center of the left margin. |
| `IMAGE_EDGE_RIGHT` | `("filename.png")` | Right margin ornament. If omitted, mirrors `IMAGE_EDGE_LEFT`. |
| `IMAGE_EDGE_PAD` | `(h_pad v_pad)` | Outward/inward displacement (mm) for horizontal and vertical edge ornaments. |
| `BORDER_LINES` | `(outer inner1 inner2 spacing)` | Up to three concentric connecting lines with thickness and separation in millimetres. Setting unused line slots to `0.0` disables them. |
| `BORDER_PAD` | `(h_pad v_pad)` | Shifts connecting lines horizontally or vertically from margin centers. |
| `BORDER_END_OFFSETS` | `(h_offset v_offset)` | Insets connecting line start and end points from corners to prevent overlapping corner ornaments. |

---

## File Lookup Precedence

When resolving the definition file specified by `ALBUM_PAGES_DECORATIVE_BORDER ("filename.def")`:
1. If the path is absolute and exists, it is loaded directly.
2. The engine checks a `borders/` subfolder located in the same directory as the album file (`<album_dir>/borders/<filename>`).
3. The engine checks a global `borders/` directory at application runtime.
4. The engine checks directly in the same folder as the album text file (`<album_dir>/<filename>`).

All image files referenced inside the definition file (`IMAGE_CORNER`, `IMAGE_EDGE_*`) must be PNG files with alpha transparency and are resolved relative to the directory containing the definition file.

---

## Fixture Variations & Benchmarks

| Variation | Definition File | Baseline TXT | AlbumEasy Ref PDF | Native Album (`.stampalbum`) | Raw Direct (`-raw.stampalbum`) | Rendered Native PDF | Key Features |
|---|---|---|---|---|---|---|---|
| **Corner** | [corner.def](corner.def) | [corner.txt](corner.txt) | [corner.reference.pdf](corner.reference.pdf) | [corner.stampalbum](corner.stampalbum) | [corner-raw.stampalbum](corner-raw.stampalbum) | [corner-native.pdf](corner-native.pdf) / [raw](corner-raw-native.pdf) | 4 mirrored corner flourishes + 3 concentric frame rules (`0.3 0.2 0.1 1.0`) with end insets (`12 10`) |
| **Edge-Mixed** | [edge-mixed.def](edge-mixed.def) | [edge-mixed.txt](edge-mixed.txt) | [edge-mixed.reference.pdf](edge-mixed.reference.pdf) | [edge-mixed.stampalbum](edge-mixed.stampalbum) | [edge-mixed-raw.stampalbum](edge-mixed-raw.stampalbum) | [edge-mixed-native.pdf](edge-mixed-native.pdf) / [raw](edge-mixed-raw-native.pdf) | Complete framing suite: 4 corners + top/bottom/left/right edge ornaments + connecting frame rules |
| **Edge-Only** | [edge-only.def](edge-only.def) | [edge-only.txt](edge-only.txt) | [edge-only.reference.pdf](edge-only.reference.pdf) | [edge-only.stampalbum](edge-only.stampalbum) | [edge-only-raw.stampalbum](edge-only-raw.stampalbum) | [edge-only-native.pdf](edge-only-native.pdf) / [raw](edge-only-raw-native.pdf) | Edge vignettes only along 4 margins; no corner images; 2-page alternating margins (`ALBUM_PAGES_MARGINSE`) |
| **Image-Only** | [image-only.def](image-only.def) | [image-only.txt](image-only.txt) | [image-only.reference.pdf](image-only.reference.pdf) | [image-only.stampalbum](image-only.stampalbum) | [image-only-raw.stampalbum](image-only-raw.stampalbum) | [image-only-native.pdf](image-only-native.pdf) / [raw](image-only-raw-native.pdf) | Pure ornamental corners scaled 1.25x; all connecting rules disabled (`BORDER_LINES(0 0 0 0)`) |
| **Omitted** | [omitted.def](omitted.def) | [omitted.txt](omitted.txt) | [omitted.reference.pdf](omitted.reference.pdf) | [omitted.stampalbum](omitted.stampalbum) | [omitted-raw.stampalbum](omitted-raw.stampalbum) | [omitted-native.pdf](omitted-native.pdf) / [raw](omitted-raw-native.pdf) | Minimal definition file containing only `IMAGE_CORNER`; tests fallback to default policy values |
| **Padded** | [padded.def](padded.def) | [padded.txt](padded.txt) | [padded.reference.pdf](padded.reference.pdf) | [padded.stampalbum](padded.stampalbum) | [padded-raw.stampalbum](padded-raw.stampalbum) | [padded-native.pdf](padded-native.pdf) / [raw](padded-raw-native.pdf) | Scaled down ornaments (`0.5x`) with positive padding offsets (`2 3`), coloured borders (`red`), and alternating margins |
| **Sparse** | [sparse.def](sparse.def) | [sparse.txt](sparse.txt) | [sparse.reference.pdf](sparse.reference.pdf) | [sparse.stampalbum](sparse.stampalbum) | [sparse-raw.stampalbum](sparse-raw.stampalbum) | [sparse-native.pdf](sparse-native.pdf) / [raw](sparse-raw-native.pdf) | Negative padding offsets (`-1 -2`), landscape orientation (`150.25 × 100.5 mm`), and recessed lines (`0 0.4 0.2 1.5`) |
| **All-in-One Showcase** | [edge-mixed.def](edge-mixed.def) | [all-in-one-compare.txt](all-in-one-compare.txt) | [all-in-one-compare-raw-native.pdf](all-in-one-compare-raw-native.pdf) | [all-in-one-compare.stampalbum](all-in-one-compare.stampalbum) | [all-in-one-compare-raw.stampalbum](all-in-one-compare-raw.stampalbum) | [all-in-one-compare-native.pdf](all-in-one-compare-native.pdf) / [raw](all-in-one-compare-raw-native.pdf) | 2-page US Letter presentation framing authentic worldwide postage stamps (San Marino, New Zealand, Malta, Sweden, US 1847) |

---

## Detailed Variation Specifications

### 1. Corner Flourishes (`corner`)
- **Corner Ornament**: `ornament.png` (30 × 18 pt transparent PNG) placed at page corners.
- **Connecting Rules**: Triple line with outer 0.3 mm, middle 0.2 mm, inner 0.1 mm, and 1.0 mm separation.
- **End Insets**: Horizontal 12.0 mm, vertical 10.0 mm from margin corners to prevent rule lines overlapping the corner flourish.

### 2. Edge & Corner Suite (`edge-mixed`)
- **Corner Ornament**: `ornament.png` offset by horizontal 1.0 mm, vertical 2.0 mm padding.
- **Edge Ornaments**: `horizontal.png` along top and bottom; `vertical.png` along left and right; offset by `IMAGE_EDGE_PAD(3 -2)`.
- **Connecting Rules**: Double line with outer 0.3 mm, middle 0.1 mm, inner 0.0 mm, 1.0 mm separation.

### 3. Edge-Only Ornaments (`edge-only`)
- **Corner Ornament**: None specified.
- **Edge Ornaments**: Top, bottom, left, right margin ornaments centered along their respective margins.
- **Pagination**: Demonstrates 2-page alternating margins (`ALBUM_PAGES_MARGINSE`), verifying that edge ornaments shift correctly on even versus odd pages.

### 4. Pure Image Corners (`image-only`)
- **Corner Ornament**: `ornament.png` scaled 1.25x with padding `(1 2)`.
- **Connecting Rules**: Completely disabled (`BORDER_LINES(0 0 0 0)`), providing a soft, open-framed presentation.

### 5. Minimal Defaults (`omitted`)
- **Definition Directives**: Only `IMAGE_CORNER("ornament.png")` is specified.
- **Default Policy**: Scale defaults to `1.0`, paddings to `0.0`, and connecting lines to `0.0` (disabled).

### 6. Padded & Coloured Border (`padded`)
- **Scale**: `0.5x` reduction.
- **Paddings**: Positive corner padding `(2 3)`, rule padding `(1 2)`, end offsets `(12 10)`.
- **Colour & Page Border**: Combined with `ALBUM_PAGES_BORDER(0.2 0 0 0)` and `COLOUR_ALBUM_BORDER(red)` to verify multi-layer boundary nesting.

### 7. Sparse Landscape Frame (`sparse`)
- **Orientation**: Landscape format (`150.25 × 100.5 mm`).
- **Scale & Inset**: Scaled `0.75x` with negative inward padding `(-1 -2)`.
- **Lines**: Asymmetric outer zero with inner accent lines (`0 0.4 0.2 1.5`).

---

## Authentic AlbumEasy Border Showcases & Custom Exhibition Borders

In addition to synthetic benchmark fixtures, five deluxe US Letter showcases demonstrate authentic AlbumEasy sample ornaments and our custom exhibition design:

| Showcase | Border Definition | AlbumEasy Source | Native Vector (`.stampalbum`) | Raw Direct (`-raw.stampalbum`) | Rendered PDF | Philatelic Showcase Highlights |
|---|---|---|---|---|---|---|
| **Classic Victorian** | `Classic.txt` | [classic-showcase.txt](classic-showcase.txt) | [classic-showcase.stampalbum](classic-showcase.stampalbum) | [classic-showcase-raw.stampalbum](classic-showcase-raw.stampalbum) | [classic-showcase-native.pdf](classic-showcase-native.pdf) | Stately acanthus scroll corner ornaments paired with double vector framing rules (`0.3 0.1 0.0 1.0`). |
| **Art Deco** | `ArtDeco.txt` | [artdeco-showcase.txt](artdeco-showcase.txt) | [artdeco-showcase.stampalbum](artdeco-showcase.stampalbum) | [artdeco-showcase-raw.stampalbum](artdeco-showcase-raw.stampalbum) | [artdeco-showcase-native.pdf](artdeco-showcase-native.pdf) | Geometric stepped corner ornaments reminiscent of 1920s–1930s philatelic albums. |
| **Corner Style A** | `CornerStyleA.txt` | [corner-style-a-showcase.txt](corner-style-a-showcase.txt) | [corner-style-a-showcase.stampalbum](corner-style-a-showcase.stampalbum) | [corner-style-a-showcase-raw.stampalbum](corner-style-a-showcase-raw.stampalbum) | [corner-style-a-showcase-native.pdf](corner-style-a-showcase-native.pdf) | Flowing baroque scrolled corner vignettes framing classic US 1847 specimens. |
| **Architectural Pillars** | `CornerAndVerticalStyleC.txt` | [pillars-showcase.txt](pillars-showcase.txt) | [pillars-showcase.stampalbum](pillars-showcase.stampalbum) | [pillars-showcase-raw.stampalbum](pillars-showcase-raw.stampalbum) | [pillars-showcase-native.pdf](pillars-showcase-native.pdf) | Fluted classical column ornaments centered along left and right margins, anchored by corner pedestals. |
| **Imperial Exhibition** | `ImperialExhibition.txt` | [imperial-showcase.txt](imperial-showcase.txt) | [imperial-showcase.stampalbum](imperial-showcase.stampalbum) | [imperial-showcase-raw.stampalbum](imperial-showcase-raw.stampalbum) | [imperial-showcase-native.pdf](imperial-showcase-native.pdf) | Deluxe custom exhibition border featuring palmettes, classical pillar side rules, navy framing rules, and period typography. |
