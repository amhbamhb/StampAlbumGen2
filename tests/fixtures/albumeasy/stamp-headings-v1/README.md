# Stamp headings: AlbumEasy-only reference pass

Tracking [#93](https://github.com/amhbamhb/StampAlbumGen2/issues/93),
branch `issue/93-stamp-headings`, based on pushed `41cba21a` (#92).
Reference-first scope: `STAMP_HEADING`, `STAMP_HEADING_VA`, their optional
horizontal offsets and signed `STAMP_HEADING_PADDING`. These references are
accepted and now have standalone mapping coverage in both compatibility input
(`.txt` / raw-command `.stampalbum`) and native `STAMPALBUM 1` derivatives.
No commit/push, PR, production routing or OKF regeneration is included.
Side-of-frame text and country Review UI previews remain ideas only.

## Complete-page references

| Input | AlbumEasy 4.5.0 PDF | Cases |
|---|---|---|
| [alignment.txt](alignment.txt) | [alignment.pdf](alignment.pdf) | Local versus vertically aligned headings; unequal physical frame heights; TOP/MIDDLE/BOTTOM rows; one/two heading lines |
| [padding-state.txt](padding-state.txt) | [padding-state.pdf](padding-state.pdf) | Omitted/zero/+3/-3 mm horizontal offsets; offsets do not persist; padding 0/+3/-1 mm; saved/restored padding and heading colour |

Standalone derivatives used by native tests:

| Route | Alignment input | Padding/state input | Notes |
|---|---|---|---|
| Raw-command compatibility | [alignment-raw.stampalbum](alignment-raw.stampalbum) | [padding-state-raw.stampalbum](padding-state-raw.stampalbum) | Exact command copies of approved TXT references |
| Native `STAMPALBUM 1` | [alignment.stampalbum](alignment.stampalbum) | [padding-state.stampalbum](padding-state.stampalbum) | Explicit native heading mode/alignment/padding controls |

Each PDF contains two exact US Letter pages (215.9 x 279.4 mm,
612 x 792 pt), 12 real RGB image placements and 12 measured stamp frames,
six images/frames per page. Each retains a 14 pt title, visible left margin
text, numbered footer, 15 mm margins and 0.2 mm page/stamp borders.
Horizontal/vertical spacing is 4/4 mm. Row labels are 8 pt; stamp captions
are two lines at 8 pt; headings are 9 pt. All four final full-page rasters
were inspected. No obvious clipping, missing images or caption overlap
remains. Human reference/native print acceptance is still pending.
Print Letter at 100%, without Fit/Shrink.

### Measured assets and unchanged physical sizing

Reuse the unchanged [source ledger](../stamp-pagination-v1/source-records.json)
and existing PNGs; no new DB access, image copying or resizing.

| Record | SG | Colnect ID | Physical dimensions | Nominal frame | Image |
|---:|---|---:|---|---|---|
| 287 | 1391 | 92832 | 40 x 26 mm | 46 x 32 mm | [us-287.png](../stamp-pagination-v1/us-287.png) |
| 288 | 1422a | 92896 | 50 x 40 mm | 56 x 46 mm | [us-288.png](../stamp-pagination-v1/us-288.png) |

The frame addition remains 6 mm total, 3 mm per edge. Explicit image
padding is 3 mm with aspect preservation; the source aspect discrepancy
recorded in the ledger is not corrected by distorting the image.
Both image SHA-256 values match the ledger:

- 287: `1aa5159ba152ca48b439d7eed328fe1db1cb08e92cb123e8bc8bbe5cdf90d9fb`
- 288: `ad7e489c32b7edf3bb0954f94bc2962a74b7a2351073328554b612910dbe8e8f`

Unequal 32/46 mm frame heights are intentional: headings tied to local
frame tops differ from headings aligned with one another. No physical
stamp dimension or mounting clearance was changed to fit the references.

## Pinned grammar and actual geometry findings

AlbumEasy 4.5.0 help QCH SHA-256:
`8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`.
Extracted heading help lines 1094-1140 and the installed
`examples/StampHeadings.txt` were inspected before preparing the cases.

```text
STAMP_HEADING (font fontsize "text" [horizontal-offset-mm])
STAMP_HEADING_VA (font fontsize "text" [horizontal-offset-mm])
STAMP_HEADING_PADDING (signed-padding-mm)
```

Headings follow the stamp-add command immediately. Newlines make multiline
headings. Padding is added to default spacing; zero restores that spacing.
Horizontal offset is per heading, not persistent state.

Actual MuPDF trace findings (points, source PDF coordinates before MuPDF's
vertical-axis transform):

- `alignment` has 11 heading lines per page; `padding-state` has 12.
  Both have 25 stroke operations per page: 24 stamp sides plus one page
  border, and six real images.
- `alignment` page 1, top row: the single-line heading baseline and the
  two-line heading's first baseline are both 672.5591 pt. The single-line
  frame top including its half-stroke extension is 663.8425 pt; the
  two-line frame top is 654.8425 pt. **ROW_ALIGN_TOP does not simply
  align bare frame tops when local headings have different line counts.**
- `alignment` page 2, top row: VA retains first baselines at 672.5591 pt
  but aligns both frame tops at 654.8425 pt, reserving the maximum heading
  footprint. This distinction needs a real row-layout test.
- Middle rows: local first baselines are 454.6457/474.4882 pt; VA first
  baselines are both 474.4882 pt. The 19.8425 pt (7 mm) difference is half
  the physical frame-height difference.
- Bottom rows: local first baselines are 236.7323/276.4173 pt; VA first
  baselines are both 276.4173 pt. The 39.6850 pt (14 mm) difference is the
  full frame-height difference.
- `padding-state` page 1: the left heading's first-line x changes from
  162.4176 to 170.9215 pt (+8.5039 pt = +3 mm); right from 385.8822 to
  377.3782 pt (-8.5040 pt = -3 mm). The next row returns to baseline x,
  confirming omitted offset resets to zero. Each multiline line stays
  centred about the same horizontally shifted heading centre.
- Padding/state baseline last-heading-line baseline is 663.5591 pt;
  top including half-stroke is 654.8425 pt. Difference: 8.7166 pt,
  or 9 pt to the outer centreline after adding the 0.2835 pt half-stroke.
  Do not infer the general font/default gap from this single 9 pt case.
- Temporary last baselines are both 465.4882 pt, with top coordinates
  448.2677 (+3 mm padding) and 459.6063 (-1 mm padding). Gaps are
  17.2205/5.8819 pt, respectively: baseline gap plus/minus the requested
  padding. Padding therefore participates in the heading/frame footprint;
  it is not merely a final text translation.
- Restored last baseline/top are 270.2520/261.5354 pt, restoring the
  8.7166 pt gap. Baseline/restored headings are darkred
  `[0.5451 0 0]`; temporary headings are darkgreen `[0 0.39216 0]`.
  Save/restore preserves padding and colour without modifying earlier items.

Bounds check after correction: all frame paths/caption baselines remain
above the lower page border at 42.803 pt. Lowest frame coordinate is
127.740 pt; lowest caption baseline 109.024 pt (111.858 pt on the
padding/state restored page). All 12 two-line caption baselines per page
are present. Do not mistake a successful AlbumEasy process for visual
fit: the first attempts below also exited 0 despite overflowing content.

## Exact execution report

From this fixture directory on Linux:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c alignment.txt
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c padding-state.txt
pdfinfo alignment.pdf
pdfinfo padding-state.pdf
pdfimages -list alignment.pdf
pdfimages -list padding-state.pdf
pdftoppm -f 1 -l 2 -scale-to 1200 -png alignment.pdf <session>/headings-alignment-final
pdftoppm -f 1 -l 2 -scale-to 1200 -png padding-state.pdf <session>/headings-padding-final
mutool draw -F trace -o <session>/headings-alignment-final.xml alignment.pdf
mutool draw -F trace -o <session>/headings-padding-final.xml padding-state.pdf
pdftotext -layout alignment.pdf <session>/headings-alignment-final.txt
pdftotext -layout padding-state.pdf <session>/headings-padding-final.txt
git diff --check
```

Both final renders and PDF inspection/extraction commands exited 0.
AlbumEasy reported successful creation with no input warnings/errors.
MuPDF reported `warning: ICC support is not available`; extraction still
succeeded. `<session>` is the active session `files` directory. Windows:
open/export both TXT files in AlbumEasy 4.5.0 with relative assets intact;
the Poppler/MuPDF arguments are portable. Windows runtime remains unverified.

Initial attempts: title 16 pt/pad 5 mm, spacing 4/8 mm, labels with 3 mm
pads rendered two pages/12 images each, but the third rows visibly crossed
the lower border and clipped captions. There was no AlbumEasy error message.
Before reference acceptance, corrected only fixture page furniture:
title 14 pt/pad 3 mm, spacing 4/4 mm, main label pad 1 mm and row label
pads 0 mm. Rerendered and inspected all four pages; no scaling or physical
frame/image-size changes were used. The superseded attempts remain only as
session logs/rasters/traces, not accepted repository references.

Filesystem impact: created this versioned directory, two TXT/reference PDF
pairs, README and binary attributes. Only its not-yet-accepted PDFs were
regenerated during fitting. Existing assets/fixtures/proofs were not
modified, moved or deleted. No helper script was created. Scratch
logs/traces/PNGs/extracted text and reference hashes remain session-only.
Plan/matrix documentation updated; future ideas preserved.

Database/ingest impact: none. No DB opened, schema checked, rows
inserted/updated/skipped/deleted, duplicate detection or country inbox
run applies. No app source, dependencies, runtime config, production
routing or native proofs were changed. Reference/documentation-only work
does not require a new Cargo test run.

## Approval boundary and next native pass

Stop for maintainer reference acceptance. This fixture is not native
support or complete heading conformance certification.

After acceptance, derive native heading mode/offset/signed-padding
semantics from actual geometry, including row footprint/pagination and
per-stamp snapshot/state. Existing native compatibility only accepts the
three-argument `STAMP_HEADING_VA` and top rows; it does not yet implement
this complete slice. MIDDLE/BOTTOM frame alignment is a coupled
prerequisite for the full alignment reference, not silently available.
Native support must explicitly map it or state an approved boundary;
do not silently comment out these commands in supported proofs.

Add raw/native-language inputs and all-three-input persistent proofs;
check baseline-to-frame geometry, individual multiline lines, mixed
heights, signed offsets/padding, saved colour/padding, unchanged image/
frame physical dimensions and actual row/frame alignment. Probe malformed
arguments, excessive/negative padding, offset overflow, different font
sizes/line leading, mixed heading modes and attachment ordering rather
than guessing. Retain #92's 120-test scope, immutable prior fixtures and
non-mutating Denmark v5 gate. No production integration, PR or OKF
regeneration; no unsolicited commit/push.
