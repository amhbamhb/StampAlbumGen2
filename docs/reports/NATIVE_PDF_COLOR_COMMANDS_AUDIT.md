# Native PDF Color Options & Modernization Audit

This document tracks the systematic audit and upgrade of all command families featured on the official website ([stampalbumgenerator2.com/pages/examples.html](https://stampalbumgenerator2.com/pages/examples.html)).

For each item in order, we verify and implement:

1. **Native Command**: First-class command parser support in `NativeAlbum` (`src/native_pdf/album.rs`) and AlbumEasy raw importer (`src/native_pdf/albumeasy.rs`).
2. **Color Option**: Optional color parameter accepting all standard formats:
   - 6-digit hex (`#RRGGBB` and bare `RRGGBB`, case-insensitive)
   - 3-digit shorthand hex (`#RGB` and bare `RGB`)
   - Functional integer RGB (`rgb(r, g, b)`)
   - Functional percentage RGB (`rgb(r%, g%, b%)`)
   - SVG / CSS named colors (`forestgreen`, `midnightblue`, etc.)
   - Sensible defaults (e.g. `black`) when omitted
3. **`.stampalbum` Fixtures**: Full suite of native `.stampalbum` and raw `*-raw.stampalbum` files created.
4. **All-In-One Showcase**: Comprehensive `all-in-one-compare.stampalbum` containing **all** variations of that family (one distinct showcase per page).
5. **Native PDF Generation**: Actual `*-native.pdf` and `*-raw-native.pdf` generated and verified.
6. **Website Gallery Sync**: Synced to website repository with updated thumbnails and manifest.

---

## Master Inventory & Progress Table

|   #    | Fixture ID                       | Command Family Title            | Category                          | Native Command |   Color Option   | .stampalbum Fixtures | All-In-One Suite | Native PDFs | Status      |
| :----: | :------------------------------- | :------------------------------ | :-------------------------------- | :------------: | :--------------: | :------------------: | :--------------: | :---------: | :---------- |
| **1**  | `crop-marks-v1`                  | Printer Crop Marks              | Page Setup & Layout               |     ✅ Yes     |      ✅ Yes      |        ✅ Yes        |      ✅ Yes      |   ✅ Yes    | ✅ Complete |
| **2**  | `margins-v1`                     | Page Margins & Clearances       | Page Setup & Layout               |     ✅ Yes     | N/A (Clearances) |        ✅ Yes        |      ✅ Yes      |   ✅ Yes    | ✅ Complete |
| **3**  | `page-size-v1`                   | Page Dimensions                 | Page Setup & Layout               |     ✅ Yes     | N/A (Dimensions) |        ✅ Yes        |      ✅ Yes      |   ✅ Yes    | ✅ Complete |
| **4**  | `page-start-v1`                  | Page Breaks & Group Templates   | Page Setup & Layout               |     ✅ Yes     | N/A (Structural) |        ✅ Yes        |      ✅ Yes      |   ✅ Yes    | ✅ Complete |
| **5**  | `page-start-var-v1`              | Variable Page Spacing           | Page Setup & Layout               |     ✅ Yes     | N/A (Structural) |        ✅ Yes        |      ✅ Yes      |   ✅ Yes    | ✅ Complete |
| **6**  | `spacing-v1`                     | Row & Stamp Spacing             | Page Setup & Layout               |     ✅ Yes     | N/A (Clearances) |        ✅ Yes        |      ✅ Yes      |   ✅ Yes    | ✅ Complete |
| **7**  | `border-v1`                      | Single, Double & Triple Borders | Page Borders & Decorative Framing |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **8**  | `border-colour-v1`               | Border Colors & Tints           | Page Borders & Decorative Framing |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **9**  | `decorative-border-v1`           | Decorative Ornate Borders       | Page Borders & Decorative Framing |     ✅ Yes     |      ✅ Yes      |        ✅ Yes        |      ✅ Yes      |   ✅ Yes    | ✅ Complete |
| **10** | `title-v1`                       | Album & Page Titles             | Headers, Footers & Titles         |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **11** | `header-v1`                      | Running Page Headers            | Headers, Footers & Titles         |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **12** | `header-date-v1`                 | Header Date & Timestamps        | Headers, Footers & Titles         |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **13** | `header-number-v1`               | Header Page Numbering           | Headers, Footers & Titles         |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **14** | `footer-v1`                      | Running Page Footers            | Headers, Footers & Titles         |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **15** | `footer-date-v1`                 | Footer Date Annotations         | Headers, Footers & Titles         |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **16** | `footer-number-v1`               | Footer Page Numbering           | Headers, Footers & Titles         |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **17** | `footer-numbering-import-v1`     | Native Footer Numbering         | Headers, Footers & Titles         |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **18** | `margin-text-v1`                 | Vertical Margin Text            | Headers, Footers & Titles         |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **19** | `album-author-v1`                | Album Author & Metadata         | Headers, Footers & Titles         |    Pending     |       N/A        |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **20** | `native-section-numbering-v1`    | Section Page Numbering          | Headers, Footers & Titles         |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **21** | `stamp-frame-v1`                 | Stamp Mount Clearances          | Stamp Frames, Shapes & Rows       |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **22** | `stamp-shapes-v1`                | Non-Rectangular Stamp Shapes    | Stamp Frames, Shapes & Rows       |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **23** | `stamp-rows-v1`                  | Row Packing & Auto-Wrap         | Stamp Frames, Shapes & Rows       |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **24** | `row-alignment-v1`               | Centered Row Alignment          | Stamp Frames, Shapes & Rows       |    Pending     |       N/A        |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **25** | `stamp-border-styles-v1`         | Stamp Frame Line Styles         | Stamp Frames, Shapes & Rows       |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **26** | `stamp-inner-borders-v1`         | Inner Decorative Framing        | Stamp Frames, Shapes & Rows       |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **27** | `stamp-background-colours-v1`    | Mount Background Fills          | Stamp Frames, Shapes & Rows       |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **28** | `stamp-image-controls-v1`        | Stamp Artwork Controls          | Stamp Images & Placement          |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **29** | `stamp-image-overloads-v1`       | Image Syntax Overloads          | Stamp Images & Placement          |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **30** | `stamp-image-overloads-v2`       | Native Image Parity             | Stamp Images & Placement          |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **31** | `stamp-items-v1`                 | Caption Text Wrapping           | Stamp Images & Placement          |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **32** | `stamp-headings-v1`              | Stamp Headings (VA)             | Text, Headings & Captions         |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **33** | `stamp-footer-text-v1`           | Stamp Footer Captions           | Text, Headings & Captions         |     ✅ Yes     |      ✅ Yes      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **34** | `stamp-text-v1`                  | Adobe Font Metrics Engine       | Text, Headings & Captions         |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **35** | `stamp-settings-save-restore-v1` | Save & Restore Stamp Styles     | Settings & Pagination             |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **36** | `stamp-pagination-v1`            | Multi-Page Flow (24 Stamps)     | Settings & Pagination             |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **37** | `core-v1`                        | Core Album Document             | Settings & Pagination             |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **38** | `native-real-stamps-v1`          | US 1959–1960 Commemoratives     | Real Stamp Collections            |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **39** | `denmark-txt-native-v1`          | Denmark 1989 (Complete 16-Page) | Real Stamp Collections            |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **40** | `denmark-background-native-v2`   | Denmark 1989 (Backgrounds)      | Real Stamp Collections            |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **41** | `denmark-footer-native-v3`       | Denmark 1989 (Footers)          | Real Stamp Collections            |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **42** | `denmark-settings-native-v4`     | Denmark 1989 (Styles)           | Real Stamp Collections            |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **43** | `denmark-margin-native-v5`       | Denmark 1989 (Margin Text)      | Real Stamp Collections            |    Pending     |     Pending      |       Pending        |     Pending      |   Pending   | ⏳ Queued   |
| **44** | `color-formats-v1`               | Hexadecimal & RGB Color Formats | Page Borders & Decorative Framing |     ✅ Yes     |      ✅ Yes      |        ✅ Yes        |      ✅ Yes      |   ✅ Yes    | ✅ Complete |
