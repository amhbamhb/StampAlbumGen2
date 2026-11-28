# Variable Page Spacing (`page-start-var-v1`)

This fixture family tests and showcases per-page horizontal and vertical spacing overrides (`PAGE_START_VAR`, `NATIVE_PAGE_START_VAR`, and parameterized `NATIVE_PAGE_BREAK`) in StampAlbumGen2 across both the native vector PDF engine and raw AlbumEasy format.

---

## 1. Stamp Collector Guide (User-Friendly Overview)

### Why Per-Page Variable Spacing Matters

A stamp collection is rarely uniform. While standard definitive issues look best with uniform gaps across every page, real-world stamp albums regularly encounter layout challenges:

- **Souvenir Sheets & Miniature Sheets**: Large format sheets or souvenir sheets often require tighter vertical row margins so that multiple blocks or accompanying singles can comfortably fit on a single page without overflowing the bottom border.
- **Varying Stamp Density**: Some commemorative sets feature elongated horizontal commemoratives, while others contain compact vertical definitives. Variable page spacing allows you to widen horizontal spacing for small stamps (giving them breathing room) or tighten vertical row spacing for tall items.
- **Zero Spacing Edge-to-Edge Displays**: Specialized exhibits, gutter pairs, and strip mountings often require stamps or rows to sit directly adjacent to one another with zero gap (`0 0`).
- **No Global Compromises**: Rather than altering your album's global `ALBUM_PAGES_SPACING` settings (which affects every single page in your entire collection), `PAGE_START_VAR` applies custom horizontal and vertical spacing strictly to that single page.

### Automatic Spacing Restoration

A critical feature for philatelic album authors is **automatic scope isolation**:
- When you use `PAGE_START_VAR (12 6)` or `NATIVE_PAGE_BREAK (12 6)`, the new spacing applies exclusively to that page.
- On the very next page, an ordinary `PAGE_START` (or `NATIVE_PAGE_BREAK ()`) automatically restores your global album spacing (`4 14` in our showcase).
- The override does **not leak** into subsequent pages or require manual re-declaration of your default album settings.

---

## 2. Technical Reference & Command Grammar

### Grammar & Argument Variations

```text
PAGE_START_VAR ( horizontal_spacing vertical_spacing [group_arg1 ... group_arg9] )
NATIVE_PAGE_START_VAR ( horizontal_spacing vertical_spacing )
NATIVE_PAGE_BREAK ( horizontal_spacing vertical_spacing )
PAGE_START ( [group_arg1 ... group_arg9] )
NATIVE_PAGE_BREAK ()
```

| Command / Argument      | Type              | Default   | Description                                                                                     |
| :---------------------- | :---------------- | :-------- | :---------------------------------------------------------------------------------------------- |
| `PAGE_START_VAR`        | Floats (mm)       | Required  | AlbumEasy command setting local horizontal stamp spacing and vertical row spacing for one page. |
| `NATIVE_PAGE_START_VAR` | Floats (mm)       | Required  | Native command alias setting per-page horizontal and vertical row spacing.                      |
| `NATIVE_PAGE_BREAK (h v)`| Floats (mm)       | Optional  | Native page break accepting two optional spacing arguments to set local page spacing.           |
| `PAGE_START`            | Strings / Numbers | Inherited | Regular page start; restores global `ALBUM_PAGES_SPACING` and maps optional group arguments.    |
| `NATIVE_PAGE_BREAK ()`  | None              | Inherited | Native zero-argument break; automatically restores global document spacing.                     |

### Spacing Argument Ordering & Group Substitution

In AlbumEasy and StampAlbumGen2, `PAGE_START_VAR` requires two numeric arguments before any optional group template variables:
```text
PAGE_START_VAR ( 12 6 "PAGE TITLE" 1960 )
                 │  │       │       │
                 │  │       │       └── $$2 placeholder in template
                 │  │       └────────── $$1 placeholder in template
                 │  └────────────────── Vertical row spacing override (6 mm)
                 └───────────────────── Horizontal stamp spacing override (12 mm)
```

- Spacing arguments must be non-negative finite numbers (in millimetres).
- Group template parameters (`$$1` through `$$9`) immediately follow the two spacing arguments.

---

## 3. Fixture Variations Inventory

| Fixture Name             | Description / Variations Tested                                                          | Native Format                                                  | Raw AlbumEasy Format                                                   | Native PDF                                                     |
| :----------------------- | :--------------------------------------------------------------------------------------- | :------------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------- |
| `override`               | 2 pages: Baseline default (H4/V14) followed by wide/compact override (H12/V6)            | [override.stampalbum](override.stampalbum)                     | [override-raw.stampalbum](override-raw.stampalbum)                     | [override-native.pdf](override-native.pdf)                     |
| `zero`                   | 2 pages: Baseline default (H4/V14) followed by edge-to-edge zero spacing override (H0/V0) | [zero.stampalbum](zero.stampalbum)                             | [zero-raw.stampalbum](zero-raw.stampalbum)                             | [zero-native.pdf](zero-native.pdf)                             |
| `horizontal`             | 3 pages: Left-aligned horizontal probe verifying default, override, and restored insets  | [horizontal.stampalbum](horizontal.stampalbum)                 | [horizontal-raw.stampalbum](horizontal-raw.stampalbum)                 | [horizontal-native.pdf](horizontal-native.pdf)                 |
| `album`                  | 4 pages: Complete standard sequence (Default -> Override -> Zero -> Restored Default)    | [album.stampalbum](album.stampalbum)                           | [album-raw.stampalbum](album-raw.stampalbum)                           | [album-native.pdf](album-native.pdf)                           |
| **`all-in-one-compare`** | **Comprehensive 7-page showcase covering ALL variations in a single document**           | [all-in-one-compare.stampalbum](all-in-one-compare.stampalbum) | [all-in-one-compare-raw.stampalbum](all-in-one-compare-raw.stampalbum) | [all-in-one-compare-native.pdf](all-in-one-compare-native.pdf) |

---

## 4. Physical Sizing & Stamp Provenance

Stamps utilized across these fixtures are authentic historical stamps from the US 1954–1968 Liberty Issue records in `tests/fixtures/albumeasy/stamp-pagination-v1/source-records.json`:

| DB ID | SG Catalog | Physical Size (W × H) |  Frame Bounds  | Image File                          |
| :---: | :--------: | :-------------------: | :------------: | :---------------------------------- |
|  150  |   1125a    |    52.0 × 29.0 mm     | 58.0 × 35.0 mm | `../stamp-pagination-v1/us-150.png` |
|  151  |   1137a    |    26.0 × 29.0 mm     | 32.0 × 35.0 mm | `../stamp-pagination-v1/us-151.png` |
|  152  |   1137b    |    26.0 × 29.0 mm     | 32.0 × 35.0 mm | `../stamp-pagination-v1/us-152.png` |
|  156  |   1148a    |    52.0 × 29.0 mm     | 58.0 × 35.0 mm | `../stamp-pagination-v1/us-156.png` |

All pages render at standard **US Letter** dimensions (`215.9 × 279.4 mm` / `612 × 792 pt`).
