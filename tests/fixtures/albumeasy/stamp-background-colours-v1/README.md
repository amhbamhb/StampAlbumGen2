# Stamp background colours: AlbumEasy 4.5.0 first pass

Tracking: [#86](https://github.com/amhbamhb/StampAlbumGen2/issues/86),
branch `issue/86-stamp-background-colours`, based on checkpoint `47b35606`.

The [source TXT](album.txt) contains AlbumEasy commands only. The maintainer
accepted the AlbumEasy reference page before native implementation began:
"Accept reference page; proceed with native implementation". The reference
TXT/PDF is preserved; actual-size print review remains pending.

## Reference behavior and scope

The pinned AlbumEasy 4.5.0 command help documents one colour-name argument,
both `COLOR_` and `COLOUR_` spellings, and persistence for succeeding items.
Stamp backgrounds apply to bordered rectangular boxes, except `CORNER`,
and not to other shapes. This first page exercises solid image stamps,
omitted background default, explicit black and persistence, the British
spelling/lightblue, `NO_FRAME`, `BLANK`, image hide/show and reset to white.
The frameless/blank cases are reference observations, not assumed semantics.
Shapes, rotated page boxes and corner-border behavior remain separate work.
Save/restore is deliberately excluded until its own mapped-state item.

Reference identity: `AlbumEasy45.0.qch`, SHA-256
`8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`.
The extracted command help's colour section, lines 1211-1284, is the source
for the syntax/restrictions above; see the
[command matrix](../../../../docs/guides/ALBUMEASY_4_5_0_STAMP_COMMAND_MATRIX.md)
for pinned-help provenance.

## Real stamps and page geometry

Reuse the verified records/assets in the
[pagination source manifest](../stamp-pagination-v1/source-records.json).
No new database query, source image modification or measurement inference is
needed for these already recorded specimens.

| Record | SG | Colnect ID | Physical size | Frame |
|---:|---|---:|---|---|
| 150 | 1125a | 700029 | 52 x 29 mm | 58 x 35 mm |
| 151 | 1137a | 700273 | 26 x 29 mm | 32 x 35 mm |
| 152 | 1137b | 700274 | 26 x 29 mm | 32 x 35 mm |
| 156 | 1148a | 700272 | 52 x 29 mm | 58 x 35 mm |

US Letter: 215.9 x 279.4 mm, 15 mm margins, 0.2 mm page border, 4 mm
horizontal and 14 mm vertical spacing. Total frame additions are 6 mm each
dimension, split into 3 mm per edge. Image padding is explicitly 3 mm,
aspect ratio preserved, with readable three-line captions. The page contains
eight placements, one intentionally hidden; frame/background visibility
must be checked in the reference output rather than inferred from counts.

## First-pass reproduction

From this directory on the current Linux reference host:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' \
  env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
```

On Windows, open `album.txt` in AlbumEasy 4.5.0 and export the PDF into this
directory; preserve the relative image paths to the sibling fixture.
Only AlbumEasy is used for this first pass.

## Review and next-pass gate

AlbumEasy 4.5.0 successfully generated [album.pdf](album.pdf), exit 0:
one exact US Letter page (612 x 792 points), seven visible image placements,
4,152,311 bytes. The 1400-pixel raster was inspected: full page border/title,
three clearly spaced rows, measured frames, readable captions and genuine
stamp images are present.

Observed reference behavior:

- Omitted background appears white; explicit black persists to the next stamp.
- `COLOUR_STAMP_BACKGROUND (lightblue)` changes the bordered rectangle fill.
- `NO_FRAME` removes both the border and coloured background for that item.
- **`STAMP_BORDER_STYLE (BLANK)` hides the border but retains the lightblue
  background.** Do not equate this setting with `NO_FRAME` in native mapping.
- Image hide leaves the black filled rectangle visible; show and explicit
  white restore the final item's normal presentation.
- Captions stay below the frame, outside its background fill.

These are reference observations, not full compatibility certification.
Reference screen acceptance is recorded above; actual-size print review and
native-pass acceptance remain pending.
Print on US Letter at **Actual size / 100%**, without Fit or Shrink.
Inspect the full border/title, row spacing, measured frame clearances,
image sizes, colour transitions, hidden image and frameless/blank behavior.

## Native pass

| Input | Direct native output |
|---|---|
| [Original AlbumEasy TXT](album.txt) | [TXT native PDF](album-txt-native.pdf) |
| [Identical raw AlbumEasy `.stampalbum`](album-raw.stampalbum) | [Raw native PDF](album-raw-native.pdf) |
| [Native-language `.stampalbum`](album.stampalbum) | [Native document PDF](album-native.pdf) |

All three paths render one exact Letter page with seven visible images,
six background fills and six stamp outlines plus the page border. Fills are
painted before images; captions remain outside them. Tests verify the exact
black/lightblue/white colour sequence, persistence, suppression on `NO_FRAME`,
retention on `BLANK` and hidden-image backgrounds. TXT/raw page content streams
and 1400-pixel raster outputs are identical.

Native commands added:

- `NATIVE_STAMP_BACKGROUND (colour)` sets subsequent frame backgrounds.
  `(none)` disables subsequent backgrounds, without affecting their outline.
- `NATIVE_STAMP_OUTLINE (show)` / `(hide)` independently controls subsequent
  outlines, retaining geometry, captions, backgrounds and image visibility.

The values are explicit in the document, not hardcoded rendering arguments.
Absent style commands preserve existing no-fill/visible-outline native
behavior. The native geometry already reserves the 3 mm mounting clearance
per edge; **zero additional image padding** matches the reference's 3 mm
padding inside the frame. Tests compare every visible native image's width/
height against the TXT native route within 0.02 points, preventing accidental
double-padding or image-size regressions.

Screen comparisons show matching background cases and image dimensions.
The TXT native rendition's content starts higher than AlbumEasy's; row
positions and title/text spacing are not exact parity. The native-language
rendition puts its subtitle in the top margin, omits repeated in-page section
headings and places rows earlier. Captions still identify every case.
These differences remain explicit; native visual/print acceptance is pending.

The separate [Denmark v2 proof](../denmark-background-native-v2/README.md)
restores the three background commands while retaining five other
deferrals. Its historical v1 inputs/PDFs/assets remain unchanged.
No production integration, OKF generation or PR is part of this item.

## Execution impact

Exact renderer command is recorded above; it completed successfully without
warnings or failures. Created this fixture directory, `album.txt`,
`album.pdf`, and README. The raster preview is in the session workspace, not
the fixture. Existing reference/native files were not regenerated or changed.
No input was moved, deleted or relocated; original stamp images are reused
without modification. No SQLite database was opened or modified: schema
checks, row insert/update/skip counts and duplicate detection are not
applicable. No ingest test or inbox flow was run.

### Native execution report

From the repository root:

```sh
cargo build --example render_stampalbum
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/stamp-background-colours-v1/album.txt tests/fixtures/albumeasy/stamp-background-colours-v1/album-txt-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/stamp-background-colours-v1/album-raw.stampalbum tests/fixtures/albumeasy/stamp-background-colours-v1/album-raw-native.pdf
target/debug/examples/render_stampalbum tests/fixtures/albumeasy/stamp-background-colours-v1/album.stampalbum tests/fixtures/albumeasy/stamp-background-colours-v1/album-native.pdf
cargo test --no-fail-fast --test native_pdf_stamp_background --test native_pdf_albumeasy --test native_pdf_album --test native_pdf_stamp_items --test native_pdf_pagination_proofs --test native_pdf_stamp_image --test native_pdf_stamp_rows --test native_pdf_stamp_text --test native_pdf_pagination --test native_pdf_row_alignment --test native_pdf_explicit_settings --test external_album_backend
cargo check --all-targets
cargo clippy --example render_stampalbum --test native_pdf_stamp_background
```

All exited 0. Focused regression suite: **80 passed, zero failed, 12 ignored**.
Background tests were rerun after adding colour/physical-size assertions:
three passed, one ignored. `cargo check --all-targets` includes all existing
examples/tests, preventing missing struct-field regressions. Clippy retains
pre-existing library warnings; no new background-test warnings.

On Windows use `target/debug/examples/render_stampalbum.exe`; these paths
use portable APIs, but Windows runtime was not available for validation.
No HTTP service, browser or database is needed for the standalone PDF flow.

Added two `.stampalbum` inputs and three native PDFs; native regeneration
replaces only this new fixture's unaccepted native outputs. No existing
accepted reference was replaced. No original stamp inputs were moved,
deleted or modified. Database impact: none (no target DB or schema/row/
duplicate checks apply).

Development errors corrected: an initially misplaced match arm prevented
Rust parsing; corrected before tests. The initial fill-count assertion
assumed `re f` without the backend's intervening `rg` colour operator;
corrected to assert actual PDF operators, exact colours and layering.
A native preview initially applied 3 mm image padding twice (geometry plus
image policy); the fixture now uses zero additional padding and dimension
regressions verify the correction. Save/restore and nonrectangular background
restrictions remain separate. Overall: implementation/automated checks pass;
native human visual/print signoff remains pending.
