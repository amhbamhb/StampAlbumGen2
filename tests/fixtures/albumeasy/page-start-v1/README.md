# PAGE_START reference fixture

**Issue:** #103. **Date:** 2026-11-09.
**Stage:** AlbumEasy first pass accepted by the maintainer; bounded native
implementation and comparison generated. Native visual/print approval pending.
No production routing or database code has changed.

| Input | AlbumEasy 4.5.0 PDF | Purpose |
|---|---|---|
| [basic.txt](basic.txt) | [basic.pdf](basic.pdf) | Two zero-argument page starts, each following an active row. |
| [album.txt](album.txt) | [album.pdf](album.pdf) | One page-start group, invoked twice with different arguments. |

Each PDF contains two US Letter pages, two real stamps per page, repeated
title/border and page numbering. The deliberate unused space proves that a
page starts explicitly, not because a row overflowed. Both reference files
end with an active row, also exercising final-row flush.

The group exercises quoted multiword parameters as complete text arguments,
an unquoted year embedded inside text, repeated substitution of the same
placeholder, and fresh substitutions on the second invocation. The fixtures
do not establish all nine placeholder positions, missing/excess arguments,
escaping, nested/malformed groups, groups containing arbitrary commands,
blank pages, PAGE_START_VAR or PAGE_START_INFO behavior. The native tests below
add strict missing/excess argument checks and all nine placeholder positions;
those checks are not additional AlbumEasy conformance claims.

## Physical sizing and asset provenance

Reuse the accepted source images and recorded Colnect physical sizes from the
[pagination manifest](../stamp-pagination-v1/source-records.json); do not
reopen or mutate the source database. Frames add 6 mm total in each dimension,
with 3 mm image padding per edge and preserved aspect ratio.

| DB ID | SG | Recorded physical size | Frame | Shared image SHA-256 |
|---|---|---|---|---|
| 150 | 1125a | 52 x 29 mm | 58 x 35 mm | `7aeaeb83f4b392d6c2c014ec2e969547070ba25cd62d2f796146f179e73ce66f` |
| 151 | 1137a | 26 x 29 mm | 32 x 35 mm | `394984daa03e373beeaa87de87cea4bc3265ab420692bacaed7490a28402f65d` |
| 152 | 1137b | 26 x 29 mm | 32 x 35 mm | `44625396b71541ffd23d414d07909cd71f56f78844d06c272287f3644ea80e24` |
| 156 | 1148a | 52 x 29 mm | 58 x 35 mm | `885aa98f62def5106302f52aa7778e4bab6642e8d6e4a4619b3ae7b75c8b3610` |

## Reference and execution report

Linux run window: 2026-11-09 18:22 through 18:24, local UTC-06:00.
Installed renderer reports AlbumEasy v4.5.0. Dependencies:

- Qt help SHA-256:
  `8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`.
- Linux wrapper SHA-256:
  `76122bc30566a6daf86312e2b5110f02b1531bdef0a47a96846f0a9684ce3336`.

Exact rendering commands, from this fixture directory:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c basic.txt
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
```

Final rendering: **success**, both commands exited 0 and printed
`Successfully created` for the expected PDF, with no warnings/errors.
The group file was corrected once before this final run; see error inventory.
These are Linux reference commands, not portable application launch code.
On Windows, open each TXT with the installed AlbumEasy 4.5.0 application and
generate its PDF. Windows execution is unverified.

Exact inspection/preview commands, from this directory:

```sh
pdfinfo basic.pdf
pdfinfo album.pdf
pdftotext -layout basic.pdf basic-pdf-text.txt
pdftotext -layout album.pdf album-pdf-text.txt
pdfimages -list basic.pdf
pdfimages -list album.pdf
pdftoppm -f 1 -l 2 -scale-to 1200 -png basic.pdf basic-preview
pdftoppm -f 1 -l 2 -scale-to 1200 -png album.pdf album-preview
```

All completed successfully (exit 0). An inline Python assertion check also
passed: two extracted pages per PDF; page-specific SG captions and numbered
footers; four image entries, distributed 2/2; expected group text and no
unexpanded `$$` markers. `pdfinfo` reports 612 x 792 pt (US Letter) for both.
Group page previews were visually inspected for heading/row placement and
caption clearance. This is assistant inspection, not collector approval.
No native runtime tests were run at this reference-only stage.

| Output | Pages | Image entries | Page text / numbering |
|---|---:|---:|---|
| basic.pdf | 2 | 4 (2/page) | First/second markers and Page 1/2 of 2 correct |
| album.pdf | 2 | 4 (2/page) | United States, 1959/1960 repeated correctly, FIRST/SECOND GROUP PAGE |

Frozen input/output SHA-256 values for this candidate reference:

| File | SHA-256 |
|---|---|
| basic.txt | `17cdfe45ca71bbd7ba3ed57fae3115a6bfcb31e0e53f465d4c4c08ce13a2ead9` |
| basic.pdf | `cdd90d9902b4f51d9d7f7b6868efeb20133b05fd29f970553714defe698fc745` |
| album.txt | `2b7f176044d7552ec8d20fbc9bb3ce13714767b02aabd7b6cecbaeefb2b1640f` |
| album.pdf | `93f6920315e242e9d0e48c2b3f87c6032db720a259ced0f7681b79f2153056bd` |

PDF hashes include renderer timestamps and need not match after regeneration.
Compare page content/geometry, not only PDF bytes.

### Error inventory and logic risks

Initial group candidate used a quoted placeholder within an already quoted
text argument while passing quoted multiword values. AlbumEasy reported:

```text
Error in line 18: PAGE_TEXT_CENTRE command - Quoted text expected.
Error in line 18: The above error originated on line 14 as part of the PAGE_START_GROUP.
```

The renderer still exited **0**, so exit status alone is not a valid rendering
success gate. Inspect messages and verify the newly generated PDF.

Correction: put `$$1` and `$$3` directly in the text-argument position,
retaining quoted invocation values; leave unquoted `$$2` inside the quoted
year string. The subsequent render succeeded with the expected text.
This is evidence that quote/substitution behavior must be preserved; do not
implement it by blindly removing quotes or silently replacing missing values.
The exact full substitution/escaping contract remains a native implementation
question, not settled by these two pages.

Other risks for the next pass: flush active rows before explicit breaks;
preserve the final row; preserve correct page totals and per-page headers;
do not expand groups recursively without a validated policy; do not confuse
row breaks, numbering-section boundaries and explicit page breaks.

### Filesystem and database impact

- Created this fixture directory, its two TXT inputs, two PDFs, four PNG
  previews, two text extracts and this README.
- No inputs moved, deleted or relocated. No shared stamp assets changed.
- No database opened; rows inserted/updated/deleted: 0. No country ingest,
  schema migration or duplicate-detection run applies.
- The failed initial group candidate was corrected in place before reference
  acceptance. There was no accepted group PDF to overwrite.

## Acceptance and next pass

Review [basic.pdf](basic.pdf) and [album.pdf](album.pdf), preferably outside
the VS Code PDF viewer if its resource URL fails. Print at Actual size / 100%
on US Letter with Fit/Shrink disabled for physical review.

Maintainer AlbumEasy screen acceptance: **accepted** in the #103 session.
Actual-size print acceptance: **pending**. Native visual acceptance: **pending**.

The maintainer approved proceeding, requested intentional blank pages now
(rather than deferring them), and then approved investigating/correcting the
initial title-spacing mismatch. The implementation and evidence follow.

## Native second pass

| Case | Original TXT through native | Byte-identical raw commands through native | Native-language rendition |
|---|---|---|---|
| Explicit starts | [basic-txt-native.pdf](basic-txt-native.pdf) | [basic-raw-native.pdf](basic-raw-native.pdf) | [basic-native.pdf](basic-native.pdf) |
| Group arguments | [album-txt-native.pdf](album-txt-native.pdf) | [album-raw-native.pdf](album-raw-native.pdf) | [album-native.pdf](album-native.pdf) |
| Native intentional empty pages | Not part of this reference | Not part of this reference | [blank-pages-native.pdf](blank-pages-native.pdf) |

Inputs: [basic-raw.stampalbum](basic-raw.stampalbum) and
[album-raw.stampalbum](album-raw.stampalbum) are exact TXT byte copies.
[basic.stampalbum](basic.stampalbum), [album.stampalbum](album.stampalbum) and
[blank-pages.stampalbum](blank-pages.stampalbum) are genuine `STAMPALBUM 1`
documents, not compatibility files with a changed extension.

All six comparison PDFs have two Letter pages and four illustrations (2/page).
The empty-page demonstration has five Letter pages; illustrations appear
only on pages 2 and 4 (two each). Pages 1, 3 and 5 retain ordinary page
decorations and numbered footers but no stamps. They are intentionally empty
**content** pages, not an instruction to suppress borders/titles/footers.
Each PDF has text extracts and 1200-pixel page previews beside it; the empty
page demonstration has a page-one preview.

### Supported grammar and intentional bounds

- TXT accepts one `PAGE_START_GROUP_BEGIN` / `PAGE_START_GROUP_END` block
  before all pages. Supported body commands execute after each `PAGE_START`;
  placeholders `$$1` through `$$9` expand once, preserving argument quotes.
  Inserted text is not recursively substituted. Missing referenced values,
  excess arguments, duplicate/nested/unclosed groups, recursive page starts,
  unsupported body commands and invalid placeholders fail explicitly.
- Body commands remain subject to the existing compatibility interpreter's
  grammar/state restrictions. This is not a full AlbumEasy macro processor.
  PAGE_START_VAR, PAGE_START_INFO and their group/count behavior remain TODO.
- Native `NATIVE_PAGE_BREAK ()` flushes the active row and starts a fresh page.
  Leading, repeated and trailing breaks intentionally preserve empty pages.
  An explicit empty document with breaks works; required declarations remain
  mandatory. Row breaks and numbering-section boundaries are distinct.
- Both native pagination paths retain automatic overflow. Long captions finish
  their continuation pages before the next explicit page. Frames are neither
  repeated nor resized. Empty pages use current page settings and participate
  in numbering; with section numbering they remain in the preceding section
  until the next section's stamp is encountered.
- The native-language proofs use attached stamp headings for selected group
  values. Free-standing native page text is still unmapped: these are
  deliberate native presentations, **not pixel-equivalent reconstructions**
  of the reference. For example, the second native group heading does not
  repeat the year twice. Reference and TXT/raw paths retain the original text.

### Initial-spacing investigation and correction

The first native TXT comparison put the first group heading at **41.07 mm**
from the top instead of AlbumEasy's **55.07 mm**. A separate session-only
reference probe showed:

| Reference probe | First group heading top |
|---|---:|
| Title extra gap 5 mm, vertical spacing 14 mm | 55.0746 mm |
| Same title, vertical spacing 0 mm | 27.0746 mm |
| No title, vertical spacing 14 mm | 16.4301 mm |

The 28 mm title-case delta establishes vertical spacing **before and after**
the title. There is no unconditional extra page-start gap when the title is
absent. The fix adds vertical spacing to the TXT title-following reservation,
not to every PAGE_START and not to the shared/native default geometry.
Native title following space is already explicit; the two native proofs use
**19 mm = 14 mm ordinary spacing + 5 mm title extra**.

Final TXT and raw group-heading tops are **55.0676 mm on both pages**, within
the **0.02 mm** comparison threshold of the frozen reference. Cargo tests
independently verify the 28 mm title-case delta and zero no-title delta within
0.01 mm. This does not claim that all other row/caption metrics are identical.
Earlier native compatibility proof binaries in other versioned families were
not overwritten; rerunning those TXT inputs now uses corrected title spacing.

The isolated probe first failed to resolve absolute image paths, then deeply
relative paths from the session location:

```text
Error in line 21: The file: /media/ray/2026-01-11/Rust/Rust/StampAlbumGen2/tests/fixtures/albumeasy/stamp-pagination-v1/us-150.png was not found.
Image file paths should be specified relative to the directory containing the album data file.
Error in line 21: The file: ../../../../../../media/ray/2026-01-11/Rust/Rust/StampAlbumGen2/tests/fixtures/albumeasy/stamp-pagination-v1/us-150.png was not found.
```

Both failed reference processes returned 0; the absent PDF caused inspection
to fail (exit 1). Copying the four shared PNGs beside the probe and using simple
local filenames resolved this. Those copies and the diagnostic input/PDF were
temporary, not fixture dependencies; shared originals were not changed.

### Validation commands and outcomes

All commands below run from the repository root. Standard Cargo tests use
existing `tempfile` directories, real PNGs and real PDF writes/parsing; no
network, database, Docker, browser or authentication service is involved.
Windows has not been executed; path handling uses `Path`/`PathBuf` and existing
portable APIs. Production CLI/UI routing is unchanged.

Pre-change baseline:

```sh
cargo test --test native_pdf_album --test native_pdf_albumeasy --test native_pdf_section_numbering --test native_pdf_pagination --test native_pdf_stamp_headings
```

Result: **36 passed, 0 failed, 4 ignored**, exit 0.

Native unit checks:

```sh
cargo test --test native_pdf_page_start --lib native_pdf
```

Result: **17 native library tests passed**, exit 0. The name filter selected
zero tests in the integration binary; integration tests were run unfiltered
by the following command, not treated as covered by this filtered run.
The final source was subsequently rechecked with `cargo test --lib native_pdf`:
17 passed, 0 failed, exit 0.

Final impacted regression gate:

```sh
cargo test --test native_pdf_page_start --test native_pdf_album --test native_pdf_albumeasy --test native_pdf_section_numbering --test native_pdf_pagination --test native_pdf_stamp_headings --test native_pdf_stamp_image --test native_pdf_stamp_rows
```

Result: **54 passed, 0 failed, 6 ignored**, exit 0. This includes eight normal
PAGE_START tests: original/raw documents, both pagination paths, intentional
empty pages, automatic overflow, 180-line caption continuation, malformed
groups/preserved outputs, title spacing, and empty documents/invalid breaks.
Frame dimensions are checked within 1e-8 mm and all footprints stay in bounds.

Regenerate persistent native proofs (after reference acceptance only):

```sh
cargo test --test native_pdf_page_start generate_native_page_start_proofs -- --ignored
cargo clippy --lib --test native_pdf_page_start --example render_stampalbum
```

Proof generation: **1 passed**, exit 0. Clippy: exit 0, with 15 warnings in
pre-existing untouched locations (unused items/fields/imports, redundant field
names, needless borrows and an existing renderer argument-count warning).
No new warning in the changed implementation/test files was reported.

Standalone dispatch was also exercised for TXT, raw `.stampalbum` and native
`STAMPALBUM 1` inputs using `cargo run --example render_stampalbum --` and
the built example binary. Each successfully wrote a two-page PDF. A portable
reproduction, overwriting only the generated native comparisons, is:

```sh
cargo run --example render_stampalbum -- tests/fixtures/albumeasy/page-start-v1/album.txt tests/fixtures/albumeasy/page-start-v1/album-txt-native.pdf
cargo run --example render_stampalbum -- tests/fixtures/albumeasy/page-start-v1/album-raw.stampalbum tests/fixtures/albumeasy/page-start-v1/album-raw-native.pdf
cargo run --example render_stampalbum -- tests/fixtures/albumeasy/page-start-v1/album.stampalbum tests/fixtures/albumeasy/page-start-v1/album-native.pdf
```

For each native comparison PDF, `pdftotext -layout`, `pdfimages -list` and
`pdftoppm -f 1 -l 2 -scale-to 1200 -png` were rerun with matching output
prefixes. The empty-page proof used `-f 1 -l 1 -singlefile` for its preview.
An inline Python assertion pass verified the page/image distributions and
`pdftotext -bbox` heading positions against the threshold above.

Development failures resolved before the final gate:

- Initial targeted test: 2 passed / 3 failed. Native proof declarations omitted
  `NATIVE_STAMP_HEADING_PADDING`: `Line 16: Missing explicit NATIVE_STAMP_HEADING_PADDING`
  (line 17 in the leading-break variant). Added explicit padding to both inputs.
- Second targeted test: 4 passed / 1 failed. The “no page break” control merged
  four stamps into one heading row:
  `Native heading row starting at stamp 1: native_pdf.rows aligned row exceeds usable width`.
  Corrected the control to use a row break, preserving two rows on one page.
- Each failing Cargo invocation exited 101; subsequent unfiltered gates passed.
  No broad catch, skipped failure or stale PDF was used to claim success.

Second-pass filesystem impact: added two raw byte copies, three native input
documents, seven native PDFs, derived text extracts/previews and the focused
Rust test. Edited four standalone native implementation modules, checklist,
plan and this README. Build artifacts/logs changed normally. No accepted
reference input/PDF, shared image, imported album, database or production
routing was modified. No user inputs were deleted, moved or relocated.

Validation verdict:

```text
environment: Linux; local Rust/Cargo and installed AlbumEasy 4.5.0
infra-tier: N/A (standalone filesystem/PDF pipeline; no DB/service dependency)
browser-tier: N/A (no browser UI changes; Poppler raster/text inspection)
startup: PASS — standalone render_stampalbum example built and ran
integration: PASS — exit_code: 0; 54 passed, 0 failed, 6 ignored; plus 17 library tests
e2e: PASS — exit_code: 0; all three CLI input routes wrote verified two-page PDFs
proofs: PASS — exit_code: 0; seven native PDF artifacts; accepted references unchanged
overall: NEEDS_SIGNOFF — bounded automated checks pass; native visual/print and Windows pending
```

**Remaining acceptance:** maintainer native visual review, actual-size print
and Windows execution. Full group-body language, native free page text and
source-page/reflow provenance remain roadmap work. No production integration,
commit, push or knowledge-bundle regeneration is implied.
