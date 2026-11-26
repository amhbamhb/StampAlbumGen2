# Page Dimensions and Orientations (`page-size-v1`)

This fixture family tests and showcases page dimensions and orientations (`ALBUM_PAGES_SIZE` / `NATIVE_PAGES_SIZE` / `NATIVE_PAGE_SIZE`) in StampAlbumGen2 across both the native PDF engine and raw AlbumEasy format.

---

## 1. Stamp Collector Guide (User-Friendly Overview)

### Why Page Dimensions Matter for Stamp Collectors

Every stamp collector uses binders, page protectors, and exhibition frames with specific mechanical constraints:

- **US Letter (8.5 × 11.0 in / 215.9 × 279.4 mm)**: The North American standard size, ideal for standard 3-ring philatelic binders (e.g. Scott, Lighthouse, Harris, or generic office binders) and standard home/office inkjet or laser printers.
- **A4 Portrait (210.0 × 297.0 mm)**: The international ISO 216 standard format widely used throughout Europe, Asia, and worldwide philatelic federations (e.g. FIP exhibition frames, Leuchtturm / Lighthouse album systems).
- **A4 Landscape (297.0 × 210.0 mm)**: Horizontal orientation, essential for wide souvenir sheets, postal covers, panoramic cachets, booklet panes, and philatelic exhibit items that cannot fit upright within a standard portrait page.
- **Fractional / Custom Dimensions**: Many classic European and specialty binders (e.g. Lindner 18-ring, Schaubek, or Davo) require customized millimeter dimensions (such as 210.25 × 297.75 mm or custom square formats).

### Setting Orientation (Portrait vs. Landscape)

In StampAlbumGen2 and AlbumEasy, page orientation is controlled purely by the numerical order of the width and height values:

- When **`width < height`**: The page is rendered in **Portrait** orientation.
- When **`width > height`**: The page is rendered in **Landscape** orientation.

No cumbersome orientation mode switches or flags are required.

### Dynamic Page Sizing in Multi-Page Albums

Within a single album document, pages can dynamically adjust their dimensions or switch between portrait and landscape (for example, displaying regular stamps on portrait pages and transitioning to landscape for a wide souvenir sheet or cover on subsequent pages).

---

## 2. Technical Reference & Command Grammar

### Grammar & Argument Variations

```text
ALBUM_PAGES_SIZE ( width height )
NATIVE_PAGES_SIZE ( width height )
NATIVE_PAGE_SIZE ( width height )
```

| Parameter | Type       | Default             | Description                                                                             |
| :-------- | :--------- | :------------------ | :-------------------------------------------------------------------------------------- |
| `width`   | Float (mm) | `215.9` (US Letter) | Physical page width in millimetres. Must be finite, positive, and <= PDF point limits.  |
| `height`  | Float (mm) | `279.4` (US Letter) | Physical page height in millimetres. Must be finite, positive, and <= PDF point limits. |

### PDF MediaBox Calculation

Given dimensions `(width_mm, height_mm)`:

- Width in PDF points: `width_pt = width_mm * (72.0 / 25.4)`
- Height in PDF points: `height_pt = height_mm * (72.0 / 25.4)`
- MediaBox: `[0.0, 0.0, width_pt, height_pt]`

All interior coordinates, margins, borders, and stamp item grids are calculated relative to these physical bounds.

---

## 3. Fixture Variations Inventory

| Variation            | Command Signature                  | Physical MediaBox (mm)   | Orientation | Accent Color        |
| :------------------- | :--------------------------------- | :----------------------- | :---------- | :------------------ |
| `a4-portrait`        | `ALBUM_PAGES_SIZE (210.0 297.0)`   | `210.0 × 297.0`          | Portrait    | Navy `#1E4A7A`      |
| `a4-landscape`       | `ALBUM_PAGES_SIZE (297.0 210.0)`   | `297.0 × 210.0`          | Landscape   | ForestGreen         |
| `us-letter`          | `ALBUM_PAGES_SIZE (215.9 279.4)`   | `215.9 × 279.4`          | Portrait    | DarkRed             |
| `fractional`         | `ALBUM_PAGES_SIZE (210.25 297.75)` | `210.25 × 297.75`        | Portrait    | Purple              |
| `all-in-one-compare` | Multi-page (4 pages)               | All 4 sizes sequentially | Mixed       | Rainbow Progression |

### Reference & Output Artifacts

| Fixture              | Native Album                                                   | Native Direct Raw                                                      | Reference PDF (AlbumEasy 4.5.0)                          | Native PDF                                                     | Native Raw PDF                                                         |
| :------------------- | :------------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------- | :------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `a4-portrait`        | [a4-portrait.stampalbum](a4-portrait.stampalbum)               | [a4-portrait-raw.stampalbum](a4-portrait-raw.stampalbum)               | [a4-portrait.reference.pdf](a4-portrait.reference.pdf)   | [a4-portrait-native.pdf](a4-portrait-native.pdf)               | [a4-portrait-raw-native.pdf](a4-portrait-raw-native.pdf)               |
| `a4-landscape`       | [a4-landscape.stampalbum](a4-landscape.stampalbum)             | [a4-landscape-raw.stampalbum](a4-landscape-raw.stampalbum)             | [a4-landscape.reference.pdf](a4-landscape.reference.pdf) | [a4-landscape-native.pdf](a4-landscape-native.pdf)             | [a4-landscape-raw-native.pdf](a4-landscape-raw-native.pdf)             |
| `us-letter`          | [us-letter.stampalbum](us-letter.stampalbum)                   | [us-letter-raw.stampalbum](us-letter-raw.stampalbum)                   | [us-letter.reference.pdf](us-letter.reference.pdf)       | [us-letter-native.pdf](us-letter-native.pdf)                   | [us-letter-raw-native.pdf](us-letter-raw-native.pdf)                   |
| `fractional`         | [fractional.stampalbum](fractional.stampalbum)                 | [fractional-raw.stampalbum](fractional-raw.stampalbum)                 | [fractional.reference.pdf](fractional.reference.pdf)     | [fractional-native.pdf](fractional-native.pdf)                 | [fractional-raw-native.pdf](fractional-raw-native.pdf)                 |
| `all-in-one-compare` | [all-in-one-compare.stampalbum](all-in-one-compare.stampalbum) | [all-in-one-compare-raw.stampalbum](all-in-one-compare-raw.stampalbum) | _N/A (Multi-page showcase)_                              | [all-in-one-compare-native.pdf](all-in-one-compare-native.pdf) | [all-in-one-compare-raw-native.pdf](all-in-one-compare-raw-native.pdf) |

---

## 4. Automated Verification

Run the dedicated page dimensions regression suite:

```sh
cargo test --test native_pdf_page_settings
```
