# Native section-local page numbering review proofs

This fixture provides persistent PDFs for reviewing native `.stampalbum`
section numbering, generated from one identical five-page US Letter document:

| Section start | PDF |
|---:|---|
| 1 (default behavior) | [section-start-1.pdf](section-start-1.pdf) |
| 10 (custom start) | [section-start-10.pdf](section-start-10.pdf) |

Open the two PDFs side by side or switch between them. The stamp layout and
section counts are identical; only the first printed number for each section
changes. The footer keeps the document total distinct from the section ordinal
and section page count.

## What to inspect

- The first `NATIVE_PAGE_NUMBER_SECTION` follows the first stamp within the
  first row. Page 1 contains stamps from both sections, but remains assigned
  to the preceding section. The following physical page begins local numbering
  for Section B.
- The second explicit marker follows the first 18 stamp items. Page 3 contains
  stamps from both Sections B and C, but remains assigned to Section B; Section
  C numbering begins on page 4.
- On the default PDF, sections start at 1 and continue as 1, 2 where needed.
  On the custom PDF, they start at 10 and continue as 10, 11.
- `$PAGES$` and `$DOCUMENT_PAGES$` both show the actual five-page document
  length; `$SECTION_PAGE$/$SECTION_PAGES$` shows the page ordinal/count for
  the section assigned to that physical page.
- Page size is exact US Letter (215.9 x 279.4 mm); the border, title, header,
  stamp frames, captions and centered dark-blue footer are included for
  collector-facing review.

The fixture reuses the unchanged verified stamp images in
[`stamp-pagination-v1`](../stamp-pagination-v1/README.md). No database lookup
or source-image change was needed.

## Reproduce

From the repository root, render either or both samples:

```sh
cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/native-section-numbering-v1/section-demo.stampalbum \
  tests/fixtures/albumeasy/native-section-numbering-v1/section-start-1.pdf \
  tests/fixtures/albumeasy/native-section-numbering-v1/section-start-1.toml

cargo run --example render_stampalbum -- \
  tests/fixtures/albumeasy/native-section-numbering-v1/section-demo.stampalbum \
  tests/fixtures/albumeasy/native-section-numbering-v1/section-start-10.pdf \
  tests/fixtures/albumeasy/native-section-numbering-v1/section-start-10.toml
```

The source album intentionally uses only explicit native boundaries; it does
not infer years from captions. This is a review fixture for the standalone
native renderer, not an AlbumEasy TXT/raw reference or application integration
test. The separate section-numbering test suite verifies that imported TXT and
raw AlbumEasy routes keep their token behavior unchanged.

## Execution record

Both `cargo run --example render_stampalbum` commands above exited 0 and
created five-page PDFs. `pdfinfo` reports US Letter (612 x 792 pt) for both.
`pdftotext -layout` confirms these labels:

| PDF | Page 1 | Page 2 | Page 3 | Page 4 | Page 5 |
|---|---|---|---|---|---|
| Start 1 | `Page 1 of 5 \| Section 1/1 \| Document 5` | `Page 1 of 5 \| Section 1/2 \| Document 5` | `Page 2 of 5 \| Section 2/2 \| Document 5` | `Page 1 of 5 \| Section 1/2 \| Document 5` | `Page 2 of 5 \| Section 2/2 \| Document 5` |
| Start 10 | `Page 10 of 5 \| Section 1/1 \| Document 5` | `Page 10 of 5 \| Section 1/2 \| Document 5` | `Page 11 of 5 \| Section 2/2 \| Document 5` | `Page 10 of 5 \| Section 1/2 \| Document 5` | `Page 11 of 5 \| Section 2/2 \| Document 5` |

Both PDFs contain all 36 stamp placements. The generation output directory
received the two PDFs; no inputs were moved, changed or deleted. No database
was accessed or modified, and no data-ingest flow ran. There were no rendering
errors. The build emitted nine existing Review UI/dead-code warnings, unrelated
to this fixture.

Representative pages 1, 3 and 4 were rasterized and visually inspected. Stamp
images, frames, captions, page layout and footer labels are visible and
consistent between variants. Maintainer review remains open for visual
acceptance and Actual-size printing at 100% on US Letter; do not use
Fit/Shrink. The PDFs are proof samples, not accepted final output or a claim of
production integration.
