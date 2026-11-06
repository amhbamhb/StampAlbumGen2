# Real-stamp row alignment (US Letter)

Print **US Letter / Actual size 100% / no Fit or Shrink**. All labels are 10 pt.
The [combined print packet](all-cases.pdf) contains all eight pages in table order.
Blue outlines show recorded Colnect physical dimensions including perforation
tips; black mounting frames add 6 mm total (3 mm per edge). Green outlines reserve
complete stamp/text footprints. Images keep aspect ratio inside the physical
area, so a slightly mismatched scan need not fill both dimensions.

| Case | PDF | Adapted TXT | Native fragment |
|---|---|---|---|
| Left | [PDF](left.pdf) | [TXT](left.txt) | [TOML](left.toml) |
| Center (new native default) | [PDF](center.pdf) | [TXT](center.txt) | [TOML](center.toml) |
| Right | [PDF](right.pdf) | [TXT](right.txt) | [TOML](right.toml) |
| Justified, including final row | [PDF](justified.pdf) | [TXT](justified.txt) | [TOML](justified.toml) |
| Justified / left / right sections | [PDF](mixed-sections.pdf) | [TXT](mixed-sections.txt) | [TOML](mixed-sections.toml) |
| Centered caption continuation (3 pages) | [PDF](center-continuation.pdf) | [TXT](center-continuation.txt) | [TOML](center-continuation.toml) |

Center distributes equal gaps between items AND at the two usable-page edges.
Left/right retain the configured 3 mm internal gap. Justified puts first/last
footprints at the usable edges and distributes space only between items.
Center and justified center single-item rows. Nearly full centered rows keep
the minimum internal gap, splitting the remaining width at the outside edges.
Alignment never changes stamp dimensions, captions, image scale or font sizes.
It applies to all rows in a section, including the final row and continuations.

Mixed-section metadata is recorded in TOML comments and [expected.json](expected.json);
the current TOML setting specifies one default policy, not a document section
language. The generator supplies explicit sections through the native API.
Future last-row-left/last-row-center variants and their compatibility projection
are planned only. Existing AlbumEasy generation is unchanged.

## Provenance and measurement

Reuses records 150, 151, 152, 156 and 157 from
[source-records.json](../stamp-pagination-v1/source-records.json), with images
referenced relatively from the existing fixture. Known physical sizes are
52x29, 26x29, 26x29, 52x29 and 26x56 mm. Frames are 58x35, 32x35, 32x35,
58x35 and 32x62 mm. No unknown-size estimation.

[expected.json](expected.json) includes every page's content bounds, section
policies, item identities, text/frame/physical/image rectangles and every text
baseline. [verification.txt](verification.txt) records independent MuPDF checks.
Do not treat adapted TXT as native row/position/page parity or reference-rendered
AlbumEasy evidence. TXT preserves all text and images in separate blocks; native
policies are not emitted as invented AlbumEasy commands.

## Reproduce (Windows or Linux)

```text
cargo test --test native_pdf_row_alignment --test native_pdf_pagination_proofs
cargo test --test native_pdf_pagination_proofs regenerate_real_letter_alignment_proofs -- --ignored --exact
mutool merge -o tests/fixtures/albumeasy/row-alignment-v1/all-cases.pdf tests/fixtures/albumeasy/row-alignment-v1/left.pdf tests/fixtures/albumeasy/row-alignment-v1/center.pdf tests/fixtures/albumeasy/row-alignment-v1/right.pdf tests/fixtures/albumeasy/row-alignment-v1/justified.pdf tests/fixtures/albumeasy/row-alignment-v1/mixed-sections.pdf tests/fixtures/albumeasy/row-alignment-v1/center-continuation.pdf
cargo test --lib review_ui::config_editor::tests
cargo test --test 'native_pdf_*'
cargo clippy --lib --test native_pdf_row_alignment --test native_pdf_pagination_proofs
cargo check --lib --target x86_64-pc-windows-gnu
```

The quoted test glob is supported by Cargo; native Windows shell syntax can use
double quotes instead of single quotes. For traces, use
`mutool draw -F trace -o TRACE_PATH PDF_PATH`. Store trace/raster scratch outside
the repository. Regular tests use self-cleaning temporary outputs.

## Execution and impact

Started with missing alignment APIs (expected compile failure); four initial
alignment tests then passed. A generator placement error caused Rust E0434
(`can't capture dynamic environment in a fn item`); moved the tests to module
scope and reran successfully. No fixture-generation/runtime error remains.

Created six PDF/TXT/TOML sets, this README, binary attributes, expected geometry
and verification evidence plus the combined eight-page print packet. No album inputs moved/deleted, no new image copies,
no database access/schema changes/rows inserted, updated or skipped. The prior
source manifest remains authoritative; this run is not an ingestion run.
Unrelated manual autosaves are untouched. No OKF regeneration.

Focused alignment suite: 5 passed. Real-image PDF suite: 3 passed, 2 optional
generators ignored; the alignment generator passed when explicitly invoked.
Final full native regression: 131 passed, 0 failed, 22 optional generators ignored
across 22 suites.
Config-editor tests: 4 passed, including per-key alignment help/default agreement.
Clippy and Windows GNU library check pass with pre-existing warnings (including
unused application helpers, PDF parser needless borrows and missing winres tool).
Windows execution/resource embedding is not validated on this Linux host.

Logic risks/limits: native default intentionally changes from left to centered
even spacing; old fixture generators now explicitly select left. Minimum gaps
override equal-edge spacing on nearly full rows to avoid overlap. Physical
printer calibration, Windows runtime and AlbumEasy reference rendering remain
unverified. Production backend selection, export mapping and future native
last-row variants remain out of scope.
