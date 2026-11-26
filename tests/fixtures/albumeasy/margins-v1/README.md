# Page Margins and Even-Page Margin Override (`margins-v1`)

This fixture family tests and showcases page margins (`ALBUM_PAGES_MARGINS` / `NATIVE_PAGES_MARGINS`) and even-page margin overrides (`ALBUM_PAGES_MARGINSE` / `NATIVE_PAGES_MARGINSE`) in StampAlbumGen2 across both the native PDF engine and raw AlbumEasy format.

---

## 1. Stamp Collector Guide (User-Friendly Overview)

### Why Page Margins Matter in Stamp Albums

When mounting stamps and creating custom stamp album pages, proper margins ensure that:

- **Binding clearance**: Ring binders, spring-back binders, and screw-post binders require extra margin on the binding side (typically 20mm to 30mm) so page contents and stamps don't get hidden under the binder rings or spine.
- **Visual balance & aesthetics**: Even spacing around album borders gives pages a clean, professional exhibition appearance.
- **Handling safety**: Sufficient outer margins keep stamp mounts and archival plastic away from fingers turning the pages.

### Book-Binding & Facing Pages (`ALBUM_PAGES_MARGINSE`)

When printing double-sided pages or assembling facing-page albums:

- **Odd pages (Right-hand / Recto)**: Require extra margin on the **left** edge (binding side).
- **Even pages (Left-hand / Verso)**: Require extra margin on the **right** edge (binding side).

With `ALBUM_PAGES_MARGINSE`, you can define odd and even page margins independently:

```text
ALBUM_PAGES_MARGINS (25.0 15.0 15.0 15.0)   # Odd pages: 25mm left binding margin
ALBUM_PAGES_MARGINSE (15.0 25.0 15.0 15.0)  # Even pages: 25mm right binding margin
```

The renderer automatically alternates margins across page breaks, providing seamless book-binding layouts.

---

## 2. Technical Reference & Command Grammar

### Grammar & Argument Variations

```text
ALBUM_PAGES_MARGINS ( left right top bottom )
NATIVE_PAGES_MARGINS ( left right top bottom )
NATIVE_PAGE_MARGINS ( left right top bottom )

ALBUM_PAGES_MARGINSE ( left right top bottom )
NATIVE_PAGES_MARGINSE ( left right top bottom )
NATIVE_PAGE_MARGINSE ( left right top bottom )
```

| Parameter | Type       | Description                                                                                |
| :-------- | :--------- | :----------------------------------------------------------------------------------------- |
| `left`    | Float (mm) | Margin distance from physical left edge of page to content/border boundary. Must be > 0.   |
| `right`   | Float (mm) | Margin distance from physical right edge of page to content/border boundary. Must be > 0.  |
| `top`     | Float (mm) | Margin distance from physical top edge of page to content/border boundary. Must be > 0.    |
| `bottom`  | Float (mm) | Margin distance from physical bottom edge of page to content/border boundary. Must be > 0. |

### Geometry & Content Area Calculation

Given physical page dimensions `(page_width, page_height)`:

- Content area left: `left`
- Content area right: `page_width - right`
- Content area bottom: `bottom`
- Content area top: `page_height - top`
- Printable width: `page_width - left - right`
- Printable height: `page_height - top - bottom`

Album borders, page titles, stamp grids, and annotations are formatted within this active content rectangle.

### Page Alternation Rules

- When only `ALBUM_PAGES_MARGINS` is set, its values apply to **all pages**.
- When `ALBUM_PAGES_MARGINSE` is also set:
  - **Odd physical pages** (1, 3, 5, 7...): Use `ALBUM_PAGES_MARGINS`.
  - **Even physical pages** (2, 4, 6, 8...): Use `ALBUM_PAGES_MARGINSE`.

---

## 3. Fixture Variations Inventory

| Variation            | Command Signature                                                                           | Key Feature Demonstrated                                            |
| :------------------- | :------------------------------------------------------------------------------------------ | :------------------------------------------------------------------ |
| `symmetric`          | `ALBUM_PAGES_MARGINS (15.0 15.0 15.0 15.0)`                                                 | Symmetric 15mm margins on all edges with Navy `#1E4A7A` border      |
| `asymmetric`         | `ALBUM_PAGES_MARGINS (25.0 15.0 15.0 15.0)`                                                 | 25mm left binding clearance with ForestGreen border                 |
| `fractional`         | `ALBUM_PAGES_MARGINS (12.25 17.75 20.25 23.75)`                                             | Precise fractional millimeter margins with DarkRed border           |
| `even-override`      | `ALBUM_PAGES_MARGINS (25.0 15.0 15.0 15.0)`<br>`ALBUM_PAGES_MARGINSE (15.0 25.0 15.0 15.0)` | Alternating odd/even facing-page binding margins with Purple border |
| `all-in-one-compare` | Multi-page (4 pages)                                                                        | All 4 variations combined sequentially in a single document         |

### Reference & Output Artifacts

| Fixture              | Native Album                                                   | Native Direct Raw                                                      | Reference PDF (AlbumEasy 4.5.0)                            | Native PDF                                                     | Native Raw PDF                                                         |
| :------------------- | :------------------------------------------------------------- | :--------------------------------------------------------------------- | :--------------------------------------------------------- | :------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `symmetric`          | [symmetric.stampalbum](symmetric.stampalbum)                   | [symmetric-raw.stampalbum](symmetric-raw.stampalbum)                   | [symmetric.reference.pdf](symmetric.reference.pdf)         | [symmetric-native.pdf](symmetric-native.pdf)                   | [symmetric-raw-native.pdf](symmetric-raw-native.pdf)                   |
| `asymmetric`         | [asymmetric.stampalbum](asymmetric.stampalbum)                 | [asymmetric-raw.stampalbum](asymmetric-raw.stampalbum)                 | [asymmetric.reference.pdf](asymmetric.reference.pdf)       | [asymmetric-native.pdf](asymmetric-native.pdf)                 | [asymmetric-raw-native.pdf](asymmetric-raw-native.pdf)                 |
| `fractional`         | [fractional.stampalbum](fractional.stampalbum)                 | [fractional-raw.stampalbum](fractional-raw.stampalbum)                 | [fractional.reference.pdf](fractional.reference.pdf)       | [fractional-native.pdf](fractional-native.pdf)                 | [fractional-raw-native.pdf](fractional-raw-native.pdf)                 |
| `even-override`      | [even-override.stampalbum](even-override.stampalbum)           | [even-override-raw.stampalbum](even-override-raw.stampalbum)           | [even-override.reference.pdf](even-override.reference.pdf) | [even-override-native.pdf](even-override-native.pdf)           | [even-override-raw-native.pdf](even-override-raw-native.pdf)           |
| `all-in-one-compare` | [all-in-one-compare.stampalbum](all-in-one-compare.stampalbum) | [all-in-one-compare-raw.stampalbum](all-in-one-compare-raw.stampalbum) | _N/A (Multi-variation showcase)_                           | [all-in-one-compare-native.pdf](all-in-one-compare-native.pdf) | [all-in-one-compare-raw-native.pdf](all-in-one-compare-raw-native.pdf) |

---

## 4. Automated Verification

Run the dedicated margins regression tests:

```sh
cargo test --test native_pdf_marginse
cargo test --test native_pdf_page_settings
```
