# Page Spacing Specification & Fixtures (spacing-v1)

This fixture demonstrates and validates the AlbumEasy command `ALBUM_PAGES_SPACING`, defining horizontal and vertical spacing between rows, stamps, captions, and page elements, alongside native extensions for frame clearance and extra-wide caption footprints.

---

## Collector's Guide: Understanding Page Spacing & Frame Clearance

When creating custom stamp album pages, proper spacing is essential to ensure that your collection looks balanced, elegant, and easy to read.

### Why Spacing Matters to Collectors
- **Horizontal Spacing (`h_mm`)**: Sets the horizontal breathing room between adjacent stamp mounts on a row. Without adequate horizontal spacing, stamp mounts crowd each other or overlap when stamps are placed on the page.
- **Vertical Spacing (`v_mm`)**: Sets the vertical distance between rows of stamps, headings, and paragraph blocks. This ensures distinct stamp sets remain visually separated and cleanly organized.
- **Frame Clearance (No Bleeding Lines)**: A common defect in automated album layout is stamp caption text touching or overlapping the frame lines. In our native PDF engine, an explicit minimum text gap (2.5 mm) is strictly enforced between the bottom frame stroke and the top of the caption text.
- **Extra-Long Captions (`expand_footprint`)**: Real stamp descriptions often have long denominations, catalog numbers, or historical commentary (e.g. *"First definitive series with high-value denominations"*). Rather than forcing text into an unreadably small font, the native engine expands the stamp's layout footprint horizontally by up to 140%, maintaining legible typography and automatically adjusting neighboring stamp positions to prevent overlapping.

---

## Authentic Stamp Data & Image Provenance

To ensure real-world fidelity, this fixture incorporates authentic scanned stamp images and metadata from four distinct countries:

| Country | Stamp Image | Dimensions (Px) | Frame Size (mm) | Description & Provenance |
|---|---|---|---|---|
| **San Marino** | `images/san-marino.jpg` | 200 × 150 | 40.0 × 30.0 mm | Classic pictorial definitive from San Marino archives (`Albums/San Marino`) |
| **New Zealand** | `images/new-zealand.jpg` | 222 × 132 | 42.0 × 25.0 mm | Scenic landscape definitive from New Zealand archives (`Albums/New Zealand`) |
| **Malta** | `images/malta.jpg` | 139 × 220 | 25.0 × 39.5 mm | Tall vertical sovereign definitive from Malta archives (`Albums/Malta`) |
| **Sweden** | `images/sweden.jpg` | 140 × 165 | 26.0 × 31.0 mm | Historical postal definitive from Sweden archives (`Albums/Sweden`) |

---

## Fixture Variations & Benchmarks

Each variation is authored in both AlbumEasy TXT syntax (for baseline comparison) and native `STAMPALBUM 1` syntax (with full vector layout, high-DPI image embedding, and frame clearance policies):

| Variation | Baseline TXT | AlbumEasy Ref PDF | Native Album (`.stampalbum`) | Raw Direct (`-raw.stampalbum`) | Rendered Native PDF | Spacing Configuration |
|---|---|---|---|---|---|---|
| **Combined** | [combined.txt](combined.txt) | [combined.reference.pdf](combined.reference.pdf) | [combined.stampalbum](combined.stampalbum) | [combined-raw.stampalbum](combined-raw.stampalbum) | [combined-native.pdf](combined-native.pdf) / [raw](combined-raw-native.pdf) | `h = 6.25 mm`, `v = 8.75 mm` |
| **Horizontal** | [horizontal.txt](horizontal.txt) | [horizontal.reference.pdf](horizontal.reference.pdf) | [horizontal.stampalbum](horizontal.stampalbum) | [horizontal-raw.stampalbum](horizontal-raw.stampalbum) | [horizontal-native.pdf](horizontal-native.pdf) / [raw](horizontal-raw-native.pdf) | `h = 6.25 mm`, `v = 0.0 mm` |
| **Vertical** | [vertical.txt](vertical.txt) | [vertical.reference.pdf](vertical.reference.pdf) | [vertical.stampalbum](vertical.stampalbum) | [vertical-raw.stampalbum](vertical-raw.stampalbum) | [vertical-native.pdf](vertical-native.pdf) / [raw](vertical-raw-native.pdf) | `h = 0.0 mm`, `v = 8.75 mm` |
| **Zero Baseline**| [zero.txt](zero.txt) | [zero.reference.pdf](zero.reference.pdf) | [zero.stampalbum](zero.stampalbum) | [zero-raw.stampalbum](zero-raw.stampalbum) | [zero-native.pdf](zero-native.pdf) / [raw](zero-raw-native.pdf) | `h = 0.0 mm`, `v = 0.0 mm` |
| **All-in-One Showcase** | [all-in-one-compare.txt](all-in-one-compare.txt) | [all-in-one-compare.reference.pdf](all-in-one-compare.reference.pdf) | [all-in-one-compare.stampalbum](all-in-one-compare.stampalbum) | [all-in-one-compare-raw.stampalbum](all-in-one-compare-raw.stampalbum) | [all-in-one-compare-native.pdf](all-in-one-compare-native.pdf) / [raw](all-in-one-compare-raw-native.pdf) | 4-page sequence with clean short captions (1 mode per page, zero bottom overflow) |

---

## Technical Command Specifications

### 1. AlbumEasy Command: `ALBUM_PAGES_SPACING`
```text
ALBUM_PAGES_SPACING (horizontal vertical)
```
- **`horizontal`**: Non-negative decimal number specifying horizontal spacing in millimeters between adjacent stamps and inner margins.
- **`vertical`**: Non-negative decimal number specifying vertical separation in millimeters between successive rows, titles, and blocks.
- **Placement**: Must appear in the document header preceding the first `PAGE_START`.
- **Diagnostics**: Missing parameters or negative values emit structured errors (`missing-spacing.txt`, `missing-vertical.txt`).

### 2. Native Engine Policy: `NATIVE_STAMP_TEXT_POLICY`
```text
NATIVE_STAMP_TEXT_POLICY (expand_footprint percent 140 2.0 1.0 2.5)
```
- **Mode**: `expand_footprint` permits text bounding boxes to exceed stamp frame width up to a specified percentage threshold.
- **Limit Unit & Value**: `percent 140` expands footprint width up to 140% of stamp frame width.
- **Minimum Font Size**: `2.0` pt prevents text from scaling down into illegibility.
- **Font Step**: `1.0` pt step resolution for font shrinking before expanding footprint.
- **Text Gap Clearance (`text_gap_mm`)**: `2.5` mm guaranteed vertical gap between the lower edge of the stamp frame stroke and the top of the caption text. This completely eliminates text-line collision and bleeding.

### 3. Native Title Clearance: `NATIVE_PAGE_TITLE`
```text
NATIVE_PAGE_TITLE (TB 16 "Country Title" 12)
```
- The 4th argument specifies an explicit vertical bottom margin (12 mm) separating the title block from the outer page border and following content, preventing header text from bleeding into the top border.

### 4. AlbumEasy Caption Limitations vs Native Layout
In standard AlbumEasy (`STAMP_ADD_IMG`), captions are drawn as a single line centered directly under the stamp frame without horizontal boundary constraints. When stamp captions are extra-long, AlbumEasy does not automatically wrap text or expand stamp spacing, causing text to overwrite adjacent stamp frames and captions.

Furthermore, AlbumEasy does not automatically break pages when items exceed usable content height. Without explicit `PAGE_START` commands, rows will overflow directly across the bottom margin border line.

The **All-in-One Showcase** (`all-in-one-compare.txt` / `.reference.pdf` / `-native.pdf` / `-raw-native.pdf`) demonstrates:
1. **Clean Multi-Page Separation**: A 4-page sequence dedicating each page to one spacing configuration (Combined, Horizontal Only, Vertical Only, and Zero Gap Baseline).
2. **Zero Bottom Margin Overflow**: Each page features 2 rows (2 stamps per row) with ample breathing space, strictly preserving bottom page margin borders without any clipping or bleeding.
3. **Short Caption Benchmark**: Clean, short captions ("San Marino", "New Zealand", "Malta", "Sweden") fit within standard frames while highlighting row and column gap behaviors across all four variations.

---

## Reference Provenance (AlbumEasy 4.5.0 Hashes)

| Input File | Input SHA-256 | Reference PDF | Reference SHA-256 |
|---|---|---|---|
| `combined.txt` | `fe255ac6e2620fe1f906d0bda299fd0a5df297659f06c31cbe2511d5f27d08de` | `combined.reference.pdf` | `4e3500237cf377671c1cf766a97581ff4f224d382f2bd560b7652c4c62d72f41` |
| `horizontal.txt` | `b599356c9fcd5e888837dcf693a728d2201cd7e1086ffb24883dd7ea845184a5` | `horizontal.reference.pdf` | `4254d8dc3734320662fac0ae1a9a1413e7fb691d87e9c6d2e4a1b91223af86ad` |
| `vertical.txt` | `030d7d654a99fcfe2ac8ba7238998672c5530fd8908613359b8c93b78fe1013d` | `vertical.reference.pdf` | `08acd9c4e264861c9e3a7fb13446f4619fb59a8d0bd475d530f28052dc15d226` |
| `zero.txt` | `3a9636e9dbadd3733611313ac433144e4e9b6fb4a85d6da06f66b93bee656251` | `zero.reference.pdf` | `28b0a2ed97069e6df35676aaa3c34f8c46484804bfcdd7fa98299d810785b56b` |
| `all-in-one-compare.txt` | `d8ecffad6e8f363645682f581dfbaef5fd53fb17b0240ea42c46deadc25f2c11` | `all-in-one-compare.reference.pdf` | `138f9fc3dd0f7a598b71af2fd6aada965a596e2a088489cdce5ae250f68ebec0` |

Diagnostic inputs:
- `missing-spacing.txt`: `d92a6290c0c5e24459bae84630eded91adeea93dc1890b193b47fb3112b34866`
- `missing-vertical.txt`: `d590b85537303000f11fe1e34d4d9cbff7bc9782ae56660396d4a4898e1bec79`

---

## Automated Layout Verification

Run the page settings integration test suite:

```sh
cargo test --test native_pdf_page_settings
```

The test `spacing_stampalbum_and_raw_files_render_and_match_spacing` verifies:
1. Native parsing and document tree resolution for `.stampalbum` and `-raw.stampalbum` files.
2. Complete mathematical verification of frame clearance: `(frame_bottom - caption_top) >= 2.0 mm` across all stamps.
3. Row 1 horizontal spacing matches configured `horizontal` parameter.
4. Row 2 horizontal spacing matches configured `horizontal` parameter.
5. Vertical gap between Row 1 and Row 2 matches configured `vertical` parameter.
