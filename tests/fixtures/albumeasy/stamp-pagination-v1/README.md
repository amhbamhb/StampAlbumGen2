# Native paginated real-stamp proofs (US Letter)

Open each PDF and print **US Letter / Actual size 100%** (Fit/Shrink off).
Labels are 10 pt for manual ruler checks.

| Case | Native PDF | Adapted AlbumEasy TXT | Purpose |
|---|---|---|---|
| Wrapped | [real-stamps-wrapped.pdf](real-stamps-wrapped.pdf) | [TXT](real-stamps-wrapped.txt) | Real known-size stamps with wrap-to-frame text |
| Expanded | [real-stamps-expanded.pdf](real-stamps-expanded.pdf) | [TXT](real-stamps-expanded.txt) | Same records with expanded-footprint text |
| Overflow caption | [overflow-caption.pdf](overflow-caption.pdf) | [TXT](overflow-caption.txt) | One real stamp with long multiline caption continued across pages |

Green = complete text/frame footprint. Black = mounting frame. Blue = physical
stamp bounds (recorded Colnect width/height). Illustrations stay centered and
fit only inside physical bounds; no image/stretch-to-text coupling.

## Real source set

- Source DB: `/home/ray/AlbumGen2/Albums/United-States/United-States.db`
- Selection criteria:
  - positive `stamps.colnect_width` and `stamps.colnect_height`
  - years 1959–1971 (pre-1978 request)
  - local PNG exists
  - image-vs-recorded aspect mismatch ≤ 5%
- Exported count: 24 real images (`us-*.png`)
- Source checksum before/after selection is identical (read-only):
  `0e06f8b7f5ec92d8e220a6324018459e31679341360fbf203893e44f7c663e55`

Full per-record provenance (record id, SG number, Colnect ID/URL, dimensions,
pixel size, SHA256, source path) is in [source-records.json](source-records.json).

## Pagination behaviors validated

1. Pending items are repacked per page using that page’s resolved content width.
2. Order is preserved globally; no losses or duplicates.
3. A stamp frame appears once; only text lines continue to later pages.
4. Continuation keeps explicit line ordering, including blank lines.
5. Footers use final count (`Page N of TOTAL`) on every page.
6. Existing output is preserved if a later-page image is missing/corrupt.

[expected.json](expected.json) records per-page content bounds, each item’s
footprint/frame/physical/image bounds, and every text line position.

## Reproduce

```text
cargo test --test native_pdf_pagination --test native_pdf_pagination_proofs
cargo test --test native_pdf_pagination_proofs regenerate_real_letter_pagination_proofs -- --ignored --exact
mutool info tests/fixtures/albumeasy/stamp-pagination-v1/real-stamps-wrapped.pdf
mutool info tests/fixtures/albumeasy/stamp-pagination-v1/real-stamps-expanded.pdf
mutool info tests/fixtures/albumeasy/stamp-pagination-v1/overflow-caption.pdf
```

## Development execution record

Commands executed in this slice:

```text
cargo test --test native_pdf_pagination --test native_pdf_stamp_rows --test native_pdf_stamp_text
cargo test --test native_pdf_pagination --test native_pdf_stamp_items
cargo test --test native_pdf_pagination --test native_pdf_pagination_proofs
cargo test --test native_pdf_pagination_proofs regenerate_real_letter_pagination_proofs -- --ignored --exact
cargo test --test native_pdf_stamp_items --test native_pdf_stamp_rows --test native_pdf_stamp_text
cargo check --target x86_64-pc-windows-gnu
```

Outcome summary:
- Focused pagination/proof tests: pass
- Real proof fixture generator: pass
- Neighboring native suites: pass
- Windows GNU compile check: pass (existing warnings only)

Filesystem impact:
- Created this fixture folder with:
  - 24 copied real PNGs (`us-*.png`)
  - 3 PDF proofs, 3 TXT companions
  - [expected.json](expected.json), [source-records.json](source-records.json), this README
- No file moves/deletes in source album folders.
- No database schema/data writes.

Database impact:
- Target DB: `/home/ray/AlbumGen2/Albums/United-States/United-States.db`
- Inserts/updates/deletes: 0/0/0 (read-only query + copy only)
- Compatibility checks: positive Colnect size columns required; image existence
  and aspect sanity filter applied before fixture inclusion.

Error inventory:
- No runtime fixture-generation errors after implementation.
- Earlier compile failures fixed:
  - missing `native_pdf::pagination` module wiring
  - TOML even-margins key path in test setup
  - `PdfDocument` page index/count type conversions (`u32`)

Logic-risk notes:
- Real proofs use one country’s available known-size set (United States).
- Physical printer calibration remains manual; this artifact only guarantees
  geometry/math consistency from DB dimensions through rendered bounds.
