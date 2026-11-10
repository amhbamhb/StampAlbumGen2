# PAGE_START_VAR reference

**Issue:** #104. **Date:** 2026-11-09.
**Stage:** AlbumEasy reference accepted by maintainer; bounded native mapping
implemented. Native screen/print acceptance remains pending.
No production routing changes.

[Input](album.txt) / [AlbumEasy reference PDF](album.pdf).
Four complete US Letter pages, each with two rows and four measured real
stamps. The group is declared once and receives its arguments on each page.

| Page | Page start | Group values |
|---|---|---|
| 1 | `PAGE_START` | Default H 4 / V 14; 1959 |
| 2 | `PAGE_START_VAR (12 6 ...)` | Override H 12 / V 6; 1960 |
| 3 | `PAGE_START_VAR (0 0 ...)` | Zero H/V override; 1961 |
| 4 | `PAGE_START` | Restored default H 4 / V 14; 1962 |

Two required spacing values precede group arguments: the first group argument
is the **third** PAGE_START_VAR argument. Quoted labels and repeated year
substitution use the accepted page-start group idiom from
[page-start-v1](../page-start-v1/README.md). Ordinary starts restore global
spacing instead of inheriting the last variable-page override.

## Evidence and bounds

All four pages are 612 x 792 pt. PDF image inventory contains 16 image entries,
four per page. Extracted text contains the correct page-specific group label,
year repeated twice and `Page N of 4`, with no unexpanded `$$` markers.

`pdftotext -bbox` measured these caption/group positions from the top:

| Page | Year text top (mm) | First-row SG caption top (mm) | Second minus first row caption (mm) |
|---|---:|---:|---:|
| 1 | 63.6204 | 107.4014 | 58.8778 |
| 2 | 55.6203 | 99.4014 | 50.8778 |
| 3 | 49.6203 | 93.4014 | 44.8778 |
| 4 | 63.6204 | 107.4014 | 58.8778 |

The spacing changes are page-local and restore correctly. The title retains
the global pre-title vertical gap, while following content and inter-row gaps
use the local override. V14 -> V6 moves following content up 8 mm, not 16 mm.

The ES rows do not visibly isolate horizontal-spacing behavior: their
placements appear unchanged. Do **not** claim that assigning H 12 / H 0 here
alone proves the full horizontal override contract. The isolated probe below
establishes the horizontal inset/reset, not every unmapped row mode.
Page 3 deliberately uses zero vertical spacing, bringing the second row close
to the preceding captions; this is reference behavior, not a recommended
collector default.

PAGE_START_INFO, columns and full group-body language remain outside this
slice. Invalid spacing arguments are rejected; native regression tests cover
all nine substitution values and automatic continuation inheritance.

## Horizontal reference probe

[Input](horizontal.txt) / [AlbumEasy PDF](horizontal.pdf).
Three pages use left-aligned PAGE_TEXT to isolate the horizontal inset:

| Page | H / V | LEFT x from left (mm) | LEFT top from top (mm) |
|---|---|---:|---:|
| 1 | 4 / 14 | 19.200001 | 54.839284 |
| 2 | 12 / 6 | 27.199999 | 46.839279 |
| 3 | ordinary reset 4 / 14 | 19.200001 | 54.839284 |

This proves +8 mm horizontal displacement and exact restoration. PAGE_TEXT
itself is still unmapped: this is **reference-only** evidence, not an input
accepted by the compatibility renderer. Do not silently drop it.

## Three native input paths

| Input | Output | Boundary |
|---|---|---|
| [Original TXT](album.txt) | [TXT-native PDF](album-txt-native.pdf) | Approved AlbumEasy source, interpreted natively. |
| [Raw command file](album-raw.stampalbum) | [Raw-native PDF](album-raw-native.pdf) | Byte-identical to TXT; no STAMPALBUM header. |
| [Native document](album.stampalbum) | [Native PDF](album-native.pdf) | Genuine STAMPALBUM 1; attached headings show group values, not free page text. |

Each output is four US Letter pages with 16 real-image placements. Physical
frames remain 58/32 x 35 mm. Native heading placement is deliberately not a
claim of identical free-page-text layout.

### Supported syntax and state

- `PAGE_START_VAR (horizontal vertical [group values...])`: two finite,
  nonnegative millimetre values, followed by up to nine group arguments.
  Zero values are intentional, not missing/default values.
- A VAR start may be the first/only TXT page and need not have a group.
  Group substitution begins **after** the two spacing values.
- Flush a previous page's active row with that page's V, before switching.
  Ordinary `PAGE_START` restores document H/V.
- Native `NATIVE_PAGE_BREAK (horizontal vertical)` selects local H/V for the
  next page and its automatic continuations. `NATIVE_PAGE_BREAK ()` resets
  document defaults. Both pagination paths support leading/repeated/trailing
  breaks with intentional empty pages.
- Native first-page override is deferred by maintainer choice. A leading
  break creates an initial empty default-spaced page; it does not rewrite it.
- Title pre-gap uses global V. Local V changes its following reservation by
  local-minus-global V; this native document uses explicit title gap 19
  (= default V14 + extra5). Default document settings are never mutated.
- Footer numbering re-resolution preserves each page's local layout,
  including section numbering and intentional blank pages.
- No new TOML keys, production integration, source DB access, or full language
  parity. Existing unsupported-command diagnostics remain active.

## Provenance

Same unchanged assets and Colnect physical sizes as the
[pagination manifest](../stamp-pagination-v1/source-records.json):
DB 150 / SG 1125a and DB 156 / SG 1148a are 52 x 29 mm, framed 58 x 35 mm;
DB 151 / SG 1137a and DB 152 / SG 1137b are 26 x 29 mm, framed 32 x 35 mm.
Image padding is 3 mm per edge; aspect ratio is preserved.
No source database was reopened or modified.

Installed renderer: AlbumEasy v4.5.0. Qt help SHA-256:
`8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`.
Linux wrapper SHA-256:
`76122bc30566a6daf86312e2b5110f02b1531bdef0a47a96846f0a9684ce3336`.

| File | SHA-256 |
|---|---|
| album.txt | `5bb8b42bcea12e0637d3e6648e5d369b76e7f73d874207c361eb8a31ee581f2d` |
| album.pdf | `3bffdf6be8837755d2bbce7c9437c93712781ba017e6be43086fcc1e6fdc226f` |
| horizontal.txt | `681646269522c8d8536c3933bd3c2d36feb5f342c95ff5622d6a81157c842a99` |
| horizontal.pdf | `8fae2b2d885813ee53e17da7c4b97cbcd832e2c225e50af8260cf262cdd48920` |

## Execution report

Linux, 2026-11-09, approximately 19:39 local UTC-06:00. From this directory:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
pdfinfo album.pdf
pdftotext -layout album.pdf album-pdf-text.txt
pdftoppm -f 1 -l 4 -scale-to 1200 -png album.pdf album-preview
pdfimages -list album.pdf
pdftotext -bbox album.pdf -
```

Final outcome: **success**, commands exited 0; renderer printed
`Successfully created`, no warning/error messages. An inline Python assertion
pass verified page text, numbering, image distribution and measured positions.
Assistant inspected pages 2/3 raster previews; maintainer subsequently accepted
the main reference and approved implementation plus the isolated H probe.
No native Cargo tests ran in that initial reference-only stage.

Filesystem impact: created this fixture directory, one TXT, one PDF, four PNG
previews, one text extract and this README. Existing proofs/assets unchanged;
no inputs moved/deleted/relocated. DB changes: none (no connections, migrations,
inserts, updates or duplicate checks). No errors observed.

Logic risks: consume the two spacing arguments before group substitution;
flush the preceding row using its own page's spacing; reset on ordinary
PAGE_START; preserve title/default versus per-page content spacing; establish
which row operations consume horizontal spacing before claiming parity.

### Native validation

The runnable suite is [native_pdf_page_start_var.rs](../../../native_pdf_page_start_var.rs).
It covers TXT/raw equivalence, first/only VAR starts, all-nine substitution
offsets, H bounds/V shifts/title invariance, unchanged physical frames, both
paginators, overflow inheritance, blank pages, errors preserving existing
output, document numbering and section numbering. Existing page-start,
AlbumEasy, album, page/title, pagination, heading and footer suites are retained.

Initial expanded test attempts exposed test-construction errors, not a
renderer regression:

| Error | Correction |
|---|---|
| `PAGE_TEXT_CENTRE arguments must be separated by whitespace` | A quoted multiword group value was placed inside an already quoted template. Use the existing supported whole-argument substitution idiom. Quote-context-aware interpolation remains outside this slice. |
| Expected `(Page 10 of 2) Tj` absent | `$PAGES$` is the document count (4), even under section scope. Correct the assertion; do not alter established numbering behavior. |
| `Missing ALBUM_PAGES_BORDER` | Supply the required border in the synthetic all-nine-values test. No production default added. |
| Final rerun temporarily failed with `E0308` during concurrent uppercase-match editing | Maintainer completed the edits; resumed validation uses `to_uppercase().as_str()`. That #104 validation did not yet add lowercase admission; the subsequent #105 shared-parser fix is documented in the [command checklist](../../../../docs/guides/NATIVE_PDF_COMMAND_CHECKLIST.md#command-keyword-case-105). |

#### Measured comparison and open follow-ups

Poppler bbox measurements verify the **change** in content position and row
separation at V6/V0 and the ordinary reset against V14 within **0.02 mm** in
all three native routes. TXT/raw positioned text is identical. Each PDF is
four Letter pages with 16 image entries, four per page.

Absolute parity is **not** claimed. An exploratory absolute year-position
threshold of 0.03 mm and row-gap threshold of 0.1 mm failed. Do not silently
relax these into a visual-parity claim:

| Follow-up | Evidence / next action |
|---|---|
| VAR-01: absolute text/row metrics | TXT/raw year top differs by up to 0.059302 mm; first caption by about 0.403 mm; inter-row caption separation by up to 1.124533 mm. Investigate existing font metrics/caption reservations in a separate reference-first slice; page-local delta/reset behavior passes. |
| VAR-02: native free page text | Native attached headings deliberately differ from the reference group text. Keep AUS-03 open rather than treating this rendition as identical free-text layout. |
| VAR-03: first-page native override | Deferred by explicit maintainer choice; do not use a leading break as a silent substitute without its intentional blank page. |
| VAR-04: acceptance | Native screen, actual-size US Letter print and Windows validation remain pending. |

Exact native validation commands, from repository root:

```sh
cargo test --test native_pdf_page_start_var --test native_pdf_page_start --test native_pdf_section_numbering --test native_pdf_albumeasy --test native_pdf_album --test native_pdf_page_title --test native_pdf_page_settings --test native_pdf_pagination --test native_pdf_stamp_headings --test native_pdf_footer_import
cargo test --lib native_pdf::
cargo test --test native_pdf_page_start_var generate_page_start_var_proofs -- --ignored --exact
cargo clippy --lib --test native_pdf_page_start_var --example render_stampalbum
cargo build --example render_stampalbum
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/page-start-var-v1/album.txt tests/fixtures/albumeasy/page-start-var-v1/album-txt-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/page-start-var-v1/album-raw.stampalbum tests/fixtures/albumeasy/page-start-var-v1/album-raw-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/page-start-var-v1/album.stampalbum tests/fixtures/albumeasy/page-start-var-v1/album-native.pdf
cargo fmt --all -- --check
git diff --check
```

The actual CLI smoke run used session-directory output PDFs to avoid replacing
the proof generator outputs. All three printed `Created native PDF` with four
pages and exited 0. Proof-generation test: 1 passed, exit 0.
Native library tests: 17 passed, exit 0. Integration regression baseline:
69 passed, 9 ignored, exit 0. Ignored tests retain their existing external-tool
or explicit proof-regeneration gates. Clippy exited 0 with pre-existing
warnings outside this change.

After the maintainer's concurrent string-match edits were complete, the full
ten-suite command above was rerun: **69 passed, 9 ignored, 0 failed, exit 0**.
The final row-spacing delta assertions are included in that run. Native units
were rerun (**17 passed, exit 0**), followed by clippy (**exit 0**, existing
warnings only), formatting and diff checks (**exit 0**). Final outcome:
**bounded mapping verified; visual/print/Windows sign-off still required**.

PDF inspection/raster commands, from this fixture directory:

```sh
pdfinfo album-txt-native.pdf
pdfimages -list album-txt-native.pdf
pdftotext -bbox album-txt-native.pdf -
pdftoppm -f 1 -l 4 -scale-to 1200 -png album-txt-native.pdf album-txt-native-preview
pdftoppm -f 1 -l 4 -scale-to 1200 -png album-native.pdf album-native-preview
```

The same pdfinfo/pdfimages/bbox checks were applied to the reference, raw-native
and genuine native PDFs. Eight native previews were created; TXT-native page 2
and genuine native page 3 were inspected. No raster-resolution parity claim.

No inputs moved/deleted/relocated; the approved AlbumEasy TXT/PDF bytes remain
unchanged. New local inputs/proofs/previews are confined to this fixture;
automated scratch PDFs use auto-cleaned temporary directories. Cargo updates
ignored build artifacts; execution logs live in the Copilot session directory.
DB impact: none, no connections/schema checks/rows/duplicates applicable.

Review at Actual size / 100% on US Letter, Fit/Shrink disabled.
AlbumEasy main reference: **accepted**. Native screen acceptance: **pending**.
Print and Windows: **unverified**; portable Rust/Path APIs are retained.
