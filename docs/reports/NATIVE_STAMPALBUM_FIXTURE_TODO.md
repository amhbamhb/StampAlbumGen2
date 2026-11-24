# Native `.stampalbum` Fixture Inventory & TODO Tracking Report

**Document ID:** `NATIVE_STAMPALBUM_FIXTURE_TODO.md`  
**Date:** 2026-11-19  
**Status:** Active Tracking  
**Cross-Repo Sync:** Synchronized with `tests/fixtures/albumeasy/` and website gallery at `https://stampalbumgenerator2.com/pages/examples.html` (`http://localhost:4205`).

---

## 1. Executive Summary

This report audits all 43 fixture families in `tests/fixtures/albumeasy/` to track the rollout of native `.stampalbum` source files alongside legacy AlbumEasy `.txt` sources and reference `.pdf` files.

### Global Statistics

| Metric | Count | Status |
|---|---|---|
| **Total Command Fixture Families** | **43** | All 43 integrated in website gallery |
| **Total Reference Output PDFs** | **222** | All 222 verified against AlbumEasy 4.5.0 |
| **Total AlbumEasy `.txt` Source Files** | **198** | Available for download |
| **Existing `.stampalbum` Source Files** | **97** | 22 fixtures have native coverage |
| **Fixtures with Native `.stampalbum` Files** | **22 / 43 (51%)** | Complete or partial native variations |
| **Fixtures Missing `.stampalbum` Files** | **21 / 43 (49%)** | **TODO: Native files to be created** |
| **Total Variations Missing `.stampalbum` Files** | **~127** | To be authored as Pass 1 (raw) and Pass 2 (native) |

---

## 2. Category Summary & Progress

| Category | Fixtures | Completed | Partial | Missing | Status |
|---|---|---|---|---|---|
| **1. Page Setup & Layout** | 6 | 3 | 1 | 2 | In Progress |
| **2. Page Borders & Decorative Framing** | 3 | 2 | 0 | 1 | In Progress |
| **3. Headers, Footers & Titles** | 10 | 1 | 4 | 5 | In Progress |
| **4. Stamp Frames, Shapes & Rows** | 5 | 0 | 0 | 5 | Pending |
| **5. Stamp Images & Placement** | 5 | 0 | 2 | 3 | In Progress |
| **6. Text, Headings & Captions** | 4 | 1 | 0 | 3 | In Progress |
| **7. Settings & Pagination** | 4 | 0 | 3 | 1 | In Progress |
| **8. Real Stamp Collections** | 6 | 5 | 1 | 0 | Mostly Complete |
| **Totals** | **43** | **12** | **11** | **20** | **23 Active / 20 TODO** |

---

## 3. Fixture-by-Fixture Detailed TODO List

### Category 1: Page Setup & Layout (6 Fixtures)

| Fixture Name | PDFs | Existing `.stampalbum` | Missing Variations (TODO) | Command / Notes |
|---|---|---|---|---|
| `page-size-v1` | 4 | 8 (`a4-portrait`, `a4-portrait-raw`, `a4-landscape`, `a4-landscape-raw`, `us-letter`, `us-letter-raw`, `fractional`, `fractional-raw`) | **None (Complete)** | `ALBUM_PAGES_SIZE` |
| `margins-v1` | 4 | 8 (`even-override`, `even-override-raw`, `symmetric`, `symmetric-raw`, `asymmetric`, `asymmetric-raw`, `fractional`, `fractional-raw`) | **None (Complete)** | `ALBUM_PAGES_MARGINS`, `ALBUM_PAGES_MARGINSE` |
| `spacing-v1` | 4 | 8 (`combined`, `combined-raw`, `horizontal`, `horizontal-raw`, `vertical`, `vertical-raw`, `zero`, `zero-raw`) | **None (Complete)** | `ALBUM_PAGES_SPACING` |
| `page-start-v1` | 9 | 5 (`basic`, `group-two-pages`, `group`, `offset-start`, `two-pages`) | `direct-starts`, `group-parameters`, `reusable-templates` | `PAGE_START`, `PAGE_START_GROUP_BEGIN/END` |
| `page-start-var-v1` | 5 | 2 (`var-direct`, `var-group`) | `var-zero`, `var-horizontal-only`, `var-vertical-only` | `PAGE_START_VAR` |
| `crop-marks-v1` | 9 | 0 | `crop-colour`, `crop-custom`, `crop-default`, `crop-disabled`, `crop-landscape`, `crop-one`, `crop-three`, `crop-two`, `crop-zero` | `ALBUM_PAGES_CROP_MARKS` |

---

### Category 2: Page Borders & Decorative Framing (3 Fixtures)

| Fixture Name | PDFs | Existing `.stampalbum` | Missing Variations (TODO) | Command / Notes |
|---|---|---|---|---|
| `border-v1` | 10 | 22 (`single`, `double`, `triple`, `legacy-double`, `disabled`, `omitted`, `inner-only`, `middle-only`, `skipped-middle`, `zero-gap`, `all-in-one-compare` + 11 raw pairs) | **None (Complete)** | `ALBUM_PAGES_BORDER` |
| `border-colour-v1` | 6 | 0 | `alias`, `default`, `global-last`, `light-grey`, `named`, `white` | `COLOR_ALBUM_BORDER`, `COLOUR_ALBUM_BORDER` |
| `decorative-border-v1` | 13 | 26 (`corner`, `edge-mixed`, `edge-only`, `image-only`, `omitted`, `padded`, `sparse`, `all-in-one-compare`, `classic-showcase`, `artdeco-showcase`, `corner-style-a-showcase`, `pillars-showcase`, `imperial-showcase` + 13 raw pairs) | **None (Complete)** | `ALBUM_PAGES_DECORATIVE_BORDER`, `COLOUR_ALBUM_DECORATIVE_BORDER` |

---

### Category 3: Headers, Footers & Titles (10 Fixtures)

| Fixture Name | PDFs | Existing `.stampalbum` | Missing Variations (TODO) | Command / Notes |
|---|---|---|---|---|
| `title-v1` | 5 | 0 | `title-8`, `title-25`, `title-multiline`, `title-negative-gap`, `title-omitted-gap` | `ALBUM_PAGES_TITLE` |
| `header-v1` | 7 | 0 | `header-centre`, `header-left`, `header-lr`, `header-omitted-pad`, `header-right`, `header-rl`, `header-title` | `ALBUM_PAGES_HEADER`, `ALBUM_PAGES_HEADER_PAD` |
| `header-date-v1` | 7 | 0 | `date-basic`, `date-centre`, `date-coexist`, `date-lr`, `date-omitted`, `date-right`, `date-rl` | `ALBUM_PAGES_HEADER_DATE` |
| `header-number-v1` | 7 | 6 (`basic`, `basic-raw`, `centre`, `centre-raw`, `lr`, `lr-raw`) | `header-number-coexist`, `header-number-left`, `header-number-omitted`, `header-number-rl` | `ALBUM_PAGES_HEADER_NUMBER` |
| `footer-v1` | 7 | 0 | `footer-centre`, `footer-left`, `footer-lr`, `footer-multiline`, `footer-omitted-pad`, `footer-right`, `footer-rl` | `ALBUM_PAGES_FOOTER`, `ALBUM_PAGES_FOOTER_PAD` |
| `footer-date-v1` | 7 | 0 | `date-basic`, `date-centre`, `date-coexist`, `date-lr`, `date-omitted`, `date-right`, `date-rl` | `ALBUM_PAGES_FOOTER_DATE` |
| `footer-number-v1` | 7 | 0 | `number-basic`, `number-coexist`, `number-lr`, `number-omitted`, `number-rl`, `number-signed`, `number-tokens` | `ALBUM_PAGES_FOOTER_NUMBER` |
| `footer-numbering-import-v1` | 11 | 5 (`album`, `album-raw`, `defaults-raw`, `normal`, `normal-raw`) | `section-boundaries`, `token-expansion` | `NATIVE_PAGE_FOOTER_NUMBER` |
| `margin-text-v1` | 13 | 6 (`album`, `basic`, `vertical-left`, `vertical-right`, `combined`, `padding`) | `alignment-variations`, `multi-page-margins` | `ALBUM_PAGES_MARGIN_TXT` |
| `album-author-v1` | 3 | 4 (`basic`, `basic-raw`, `album`, `album-raw`) | **None (100% Complete)** | `ALBUM_AUTHOR`, `ALBUM_TITLE` |

---

### Category 4: Stamp Frames, Shapes & Rows (5 Fixtures)

| Fixture Name | PDFs | Existing `.stampalbum` | Missing Variations (TODO) | Command / Notes |
|---|---|---|---|---|
| `stamp-frame-v1` | 11 | 0 | `01-standard-small-image`, `02-standard-fit-image`, `03-fractional`, `04-portrait`, `05-landscape`, `06-booklet`, `07-se-tenant`, `08-wide`, `09-zero-clearance`, `10-calibration`, `all-cases` | `STAMP_ADD_IMG`, frame clearances |
| `stamp-shapes-v1` | 1 | 0 | `album` (triangular, diamond, circular mounts) | `STAMP_SHAPE` |
| `row-alignment-v1` | 7 | 0 | `all-cases`, `center`, `center-continuation`, `justified`, `left`, `mixed-sections`, `right` | `ROW_START_ES`, `ROW_ALIGN_*` |
| `stamp-rows-v1` | 5 | 0 | `all-cases`, `large-wrap`, `mixed-heights`, `standard-wrap`, `zero-clearance` | `ROW_START_FS` |
| `stamp-inner-borders-v1` | 6 | 2 (`solid`, `dashed`) | `dotted`, `double`, `offset`, `colored` | `STAMP_INNER_BORDER`, `STAMP_INNER_BORDER_STYLE` |

---

### Category 5: Stamp Images & Placement (5 Fixtures)

| Fixture Name | PDFs | Existing `.stampalbum` | Missing Variations (TODO) | Command / Notes |
|---|---|---|---|---|
| `stamp-image-controls-v1` | 4 | 2 (`controls-native`, `controls-raw`) | `aspect-ratio-locked`, `stretch-override` | `ALBUM_STAMP_IMG_ASPECT_RATIO`, `ALBUM_STAMP_IMG_NEW_SETTING` |
| `stamp-image-overloads-v1` | 1 | 0 | `album` | Legacy argument syntax |
| `stamp-image-overloads-v2` | 2 | 0 | `album`, `overload-variants` | Extended argument syntax |
| `stamp-background-colours-v1` | 4 | 2 (`background-tint`, `background-fill`) | `transparent`, `per-stamp-color` | `COLOR_STAMP_BACKGROUND` |
| `stamp-border-styles-v1` | 12 | 4 (`solid`, `dashed`, `dotted`, `thick`) | 8 remaining border stroke variations | `STAMP_BORDER_STYLE` |

---

### Category 6: Text, Headings & Captions (4 Fixtures)

| Fixture Name | PDFs | Existing `.stampalbum` | Missing Variations (TODO) | Command / Notes |
|---|---|---|---|---|
| `stamp-headings-v1` | 2 | 4 (`headings-va`, `headings-va-raw`, `headings-local`, `headings-local-raw`) | **None (100% Complete)** | `STAMP_HEADING`, `STAMP_HEADING_VA` |
| `stamp-footer-text-v1` | 4 | 0 | `alignment`, `native-alignment`, `native-padding-state`, `padding-state` | `STAMP_FOOTER_TXT_ALIGN`, `STAMP_FOOTER_TXT_PAD` |
| `stamp-text-v1` | 5 | 0 | `all-cases`, `expanded-mm-30`, `expanded-percent-120`, `expand_footprint`, `wrap_to_frame` | `PAGE_TEXT`, `PAGE_TEXT_CENTRE`, caption wrapping |
| `core-v1` | 1 | 0 | `reference` | Minimal baseline |

---

### Category 7: Settings & Pagination (4 Fixtures)

| Fixture Name | PDFs | Existing `.stampalbum` | Missing Variations (TODO) | Command / Notes |
|---|---|---|---|---|
| `stamp-settings-save-restore-v1` | 4 | 2 (`save-restore`, `save-restore-raw`) | `nested-saves`, `stack-overflow-check` | `ALBUM_STAMP_SETTINGS_SAVE/RESTORE` |
| `stamp-items-v1` | 3 | 0 | `all-cases`, `expanded-images`, `wrapped-images` | Item flow & line wrapping |
| `stamp-pagination-v1` | 3 | 0 | `overflow-caption`, `real-stamps-expanded`, `real-stamps-wrapped` | Multi-page overflow |
| `native-section-numbering-v1` | 2 | 1 (`section-numbering`) | `section-start-1`, `section-start-10` | Section page resets |

---

### Category 8: Real Stamp Collections (6 Fixtures)

| Fixture Name | PDFs | Existing `.stampalbum` | Missing Variations (TODO) | Command / Notes |
|---|---|---|---|---|
| `denmark-txt-native-v1` | 3 | 1 (`denmark-1989-16p`) | `Denmark-1989-1989-reference` | Full 16-page commemorative album |
| `denmark-background-native-v2` | 2 | 1 (`denmark-1989-bg`) | **None (Complete for native)** | Background tint integration |
| `denmark-footer-native-v3` | 2 | 1 (`denmark-1989-footers`) | **None (Complete for native)** | Running footers & dynamic numbering |
| `denmark-settings-native-v4` | 2 | 1 (`denmark-1989-styles`) | **None (Complete for native)** | Multi-style commemorative sets |
| `denmark-margin-native-v5` | 2 | 1 (`denmark-1989-margin`) | **None (Complete for native)** | Alternating vertical margin text |
| `native-real-stamps-v1` | 2 | 1 (`us-commemoratives`) | **None (Complete for native)** | High-res real stamp placement |

---

## 4. Priority Execution Strategy

To eliminate empty `.stampalbum` files systematically, follow this execution order:

1. **Phase A — Complete High-Impact Page Setup Fixtures (100% COMPLETE):**
   - [x] `page-size-v1` (4 variations: A4 portrait, A4 landscape, US Letter, fractional)
   - [x] `margins-v1` (4 variations: asymmetric, fractional, symmetric, even-override)
   - [x] `spacing-v1` (4 variations: horizontal, vertical, combined, zero)
2. **Phase B — Complete Page Borders:**
   - [ ] `border-v1` (10 variations: single, double, triple, inner-only, etc.)
   - [ ] `border-colour-v1` (6 variations: hex, named colors, grey tints)
3. **Phase C — Complete Header & Footer Families:**
   - [ ] `title-v1` (5 variations)
   - [ ] `header-v1` (7 variations)
   - [ ] `header-date-v1` (7 variations)
   - [ ] `header-number-v1` (4 remaining variations: left, omitted, rl, coexist)
   - [ ] `footer-v1` (7 variations)
   - [ ] `footer-date-v1` (7 variations)
   - [ ] `footer-number-v1` (7 variations)
4. **Phase D — Stamp Frames, Images, and Rows:**
   - [ ] `stamp-frame-v1`, `stamp-rows-v1`, `row-alignment-v1`
   - [ ] `stamp-image-controls-v1`, `stamp-background-colours-v1`
5. **Phase E — Text, Captions & Advanced Pagination:**
   - [ ] `stamp-text-v1`, `stamp-footer-text-v1`, `stamp-pagination-v1`
