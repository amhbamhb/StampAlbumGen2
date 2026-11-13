# Stamp border styles: accepted reference and bounded native mapping

Tracking [#92](https://github.com/amhbamhb/StampAlbumGen2/issues/92),
branch `issue/92-stamp-border-styles`, based on pushed `f5ef92bc` (#91).
Maintainer approved native continuation on 2026-11-07 ("cool - proceed").
The two main AlbumEasy reference TXT/PDF pairs remain unchanged. Native
mapping and retained three-input regression are now implemented; no
commit/push, PR, production integration or OKF regeneration is included.

## Complete-page references

| AlbumEasy-only input | AlbumEasy 4.5.0 PDF | Cases |
|---|---|---|
| [album.txt](album.txt) | [album.pdf](album.pdf) | Outer ordinary styles, CORNER selectors, SIDES persistence/empty/reset, outer BLANK with inner L/R |
| [inner-state.txt](inner-state.txt) | [inner-state.pdf](inner-state.pdf) | Inner CORNER/SIDES, independent outer/inner masks/styles, temporary settings and save/restore |

Each reference has two exact US Letter pages (215.9 x 279.4 mm; 612 x 792 pt),
12 real RGB image placements, six per page. Each retains title, 15 mm
margins, 0.2 mm page border, readable captions, 4 mm horizontal/14 mm
vertical spacing, visible left margin text and numbered two-page footers.
Stamp outer-line thickness is explicitly 0.2 mm. All four full-page
1200-pixel rasters were inspected: no obvious clipping, missing images
or caption overlap. Reference continuation is approved; human native
visual/Actual-size print acceptance remains pending.
Print Letter at 100%, without Fit/Shrink.

Approved reference SHA-256 values (verified unchanged at native handoff):

| File | SHA-256 |
|---|---|
| album.txt | `ccbcf5f409281d58ce54aa52a5199034f6aaed5ee4c2b08d4df9253eab30ab39` |
| album.pdf | `68e2758c84624316f684582d60d0acc01fa45a8fe4e10eb191dab702ccd234fc` |
| inner-state.txt | `2bc505e8fb86a065c6c89f62b17159ae97f5e4b7db923ba946ced48b2b419611` |
| inner-state.pdf | `ed4522354213fb04c010f6d8470a562b1459376a99a13d132fae744f8c76a3f4` |

### Outer reference

- Page 1: SOLID/DOTTED, DASHED/DOT_DASH, all CORNER markers then TL/BR.
- Page 2: `SIDES B` with SOLID, then DASHED without resetting the mask;
  empty SIDES with SOLID, then CORNER TR/BL despite the empty mask;
  all-side reset then SOLID, followed by outer BLANK retaining inner L/R.

### Inner/state reference

- Page 1: all inner corners then TL/BR; inner T/B SOLID then DASHED;
  empty inner mask with SOLID then CORNER TR/BL despite the empty mask.
- Page 2 baseline: outer DOTTED L/R plus inner darkblue DASHED T/B.
  Temporary state: full outer SOLID plus green inner CORNER TL/BR, with
  an empty inner side mask. Restoration returns both independent masks,
  ordinary styles and inner colour. This also proves restore does not leave
  the temporary corner mode active.

## Assets / physical-size provenance

Four unchanged assets and recorded Colnect sizes are reused from
[source-records.json](../stamp-pagination-v1/source-records.json):

| Record | SG | Colnect ID | Physical size | Nominal outer frame |
|---:|---|---:|---|---|
| 150 | 1125a | 700029 | 52 x 29 mm | 58 x 35 mm |
| 151 | 1137a | 700273 | 26 x 29 mm | 32 x 35 mm |
| 152 | 1137b | 700274 | 26 x 29 mm | 32 x 35 mm |
| 156 | 1148a | 700272 | 52 x 29 mm | 58 x 35 mm |

Frames add 6 mm total (3 mm per edge); image padding is explicitly 3 mm with
aspect preservation. Inner padding 2 mm/line thickness 0.2 mm are distinct
from physical image sizing/mounting clearance. No DB lookup, asset copying
or resizing was necessary. This new versioned bundle does not modify
the accepted [inner-border reference](../stamp-inner-borders-v1/README.md).

## Pinned grammar and measured reference behavior

AlbumEasy 4.5.0 help QCH SHA-256:
`8b81707d861aacceb798b9f38efbbfdc3ea86fa49a57b7e343c3a25d5964d5be`.
Style/selector help is at extracted lines 1016-1075.

```text
STAMP_BORDER_STYLE (SOLID|DOTTED|DASHED|DOT_DASH|BLANK)
STAMP_BORDER_STYLE (CORNER [TL TR BL BR])
STAMP_BORDER_STYLE (SIDES [L R T B])
STAMP_INNER_BORDER_STYLE (SOLID|DOTTED|DASHED|DOT_DASH|BLANK)
STAMP_INNER_BORDER_STYLE (CORNER [TL TR BL BR])
STAMP_INNER_BORDER_STYLE (SIDES [L R T B])
```

Bracketed notation describes optional selectors, not literal input.
CORNER without selectors draws all four. SIDES without selectors draws
none. SIDES changes an independent mask; switching ordinary style does
not reset it. CORNER ignores the side mask. Explicit all-side selection
resets it. Outer and inner masks/styles are independent, and save/restore
captures both. These selector behaviors apply to rectangular boxes; other
shapes/rotation remain separate later obligations.

Actual MuPDF trace:

| Reference / page | Images | Stroke operations, including one page border |
|---|---:|---:|
| album / 1 | 6 | 23: sixteen ordinary outer sides, six corner paths, one page border |
| album / 2 | 6 | 11: six outer sides, two corner paths, two inner sides, one page border |
| inner-state / 1 | 6 | 37: twenty-four outer sides, four inner sides, eight inner corner paths, one page border |
| inner-state / 2 | 6 | 29: sixteen outer sides, eight inner sides, four inner corner paths, one page border |

- Each corner is one connected path: move, line to the corner, line away;
  not two independent stroked segments. Observed legs are 5.66929 pt
  (2 mm), solid even when ordinary dash styles previously applied.
- Ordinary dash arrays match #91: DOTTED `[1]`, DASHED `[3 2 3 2]`,
  DOT_DASH `[3 2 1 2]`, phase 0, in points.
- Normal 0.2 mm width is 0.56693 pt. Outer/inner corner anchors do not
  blindly reuse full-side endpoint adjustments. Example inner top-left
  path: `(139.55905, 581.1181)` to `(139.55905, 586.7874)` to
  `(145.22835, 586.7874)` pt. The earlier full-side inner x=139.41733 pt
  is not the same anchor. The next native pass must compare actual
  geometry/colour/dash and selector paths, not assume an inset rectangle.

The native pass below checks actual image-centred geometry. The apparent
anchor difference from #91 also reflected its different outer thickness:
the centreline inset is padding + (outer + inner thickness)/2 for both
styles. Unlike full sides, joined corner vertices have no half-thickness
endpoint extension. Unequal-width/hairline probes verify this rather than
assuming an extra inset. Full language support or whole-page pixel parity
is not claimed.

## Reference-pass execution report

From this fixture directory on Linux:

```sh
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c album.txt
timeout 60s xvfb-run -a -s '-screen 0 1280x1024x24' env QT_QPA_PLATFORM=xcb /home/ray/work/AlbumEasy/AlbumEasy.sh -c inner-state.txt
pdfinfo album.pdf
pdfinfo inner-state.pdf
pdfimages -list album.pdf
pdfimages -list inner-state.pdf
pdftoppm -f 1 -l 2 -scale-to 1200 -png album.pdf <session>/styles-album
pdftoppm -f 1 -l 2 -scale-to 1200 -png inner-state.pdf <session>/styles-inner-state
mutool draw -F trace -o <session>/styles-album-trace.xml album.pdf
mutool draw -F trace -o <session>/styles-inner-state-trace.xml inner-state.pdf
```

All render/inspection commands exited 0. AlbumEasy reported successful
PDF creation without input warnings/errors; page and RGB placement counts
were verified. MuPDF reported `warning: ICC support is not available` for
each trace command; trace extraction succeeded. `<session>` is the active
session's `files` directory. Windows equivalent: open/export both TXT files
in AlbumEasy 4.5.0 with relative assets intact; Poppler/MuPDF arguments are
portable. Windows runtime remains unverified.

Filesystem impact: created this directory, two TXT inputs, two reference
PDFs, README and fixture-local PDF binary attributes. No earlier sources,
fixtures, images or reference PDFs were modified, moved, deleted or
relocated. PNG/log/trace scratch remains session-only. No one-off helper
script created. Database/ingest impact: none; no DB opened, schema checks,
rows inserted/updated/skipped, duplicate detection or country inbox runs
apply. No application source or dependencies changed during the reference pass. Documentation-only
changes need whitespace validation, not a new Cargo test run.

## Native inputs and persistent proofs

| Case | Original TXT route | Raw-command route | Explicit native-language route |
|---|---|---|---|
| Outer styles | [album-txt-native.pdf](album-txt-native.pdf) | [album-raw.stampalbum](album-raw.stampalbum) / [album-raw-native.pdf](album-raw-native.pdf) | [album.stampalbum](album.stampalbum) / [album-native.pdf](album-native.pdf) |
| Inner/state | [inner-state-txt-native.pdf](inner-state-txt-native.pdf) | [inner-state-raw.stampalbum](inner-state-raw.stampalbum) / [inner-state-raw-native.pdf](inner-state-raw-native.pdf) | [inner-state.stampalbum](inner-state.stampalbum) / [inner-state-native.pdf](inner-state-native.pdf) |

All six PDFs have two exact Letter pages, six images per page. TXT/raw
sources are byte-identical and retain every reference command. Both
rendered page content streams and all four TXT/raw PNG pairs are identical.
Eight distinct TXT/native-language complete-page rasters were inspected.
No obvious clipping, missing images or caption overlap was found.

The explicit native documents retain the 6 mm total frame addition,
physical sizes, image aspect/scale, captions, styles, masks, colour,
save/restore, page title, visible margin text and numbered footer.
They are not complete-page positional replicas. Reference `PAGE_TEXT_CENTRE`
row labels are recorded as comments and represented by the stamp captions;
native headers summarise the case. `PAGE_START` is recorded as a comment:
native pagination is footprint-driven. The native vertical gap explicitly
reserves row-label space: 14 mm + 9 pt (3.175 mm) + 5 mm pad = 22.175 mm.
This keeps three rows per page with unchanged stamp/image dimensions.
TXT/raw retain all labels, explicit page starts and 14 mm vertical spacing.
No parser silently drops these commands: this is an explicit derivative
layout difference, not a claim that native row labels/page-start controls
have been mapped. No production settings or TOML keys were added.

### Shared state and path contract

Implementation is in [stamp_inner_border.rs](../../../../src/native_pdf/stamp_inner_border.rs),
[album.rs](../../../../src/native_pdf/album.rs),
[albumeasy.rs](../../../../src/native_pdf/albumeasy.rs) and
[renderer.rs](../../../../src/native_pdf/renderer.rs).

```text
NATIVE_STAMP_BORDER_STYLE (SOLID|DOTTED|DASHED|DOT_DASH|BLANK)
NATIVE_STAMP_BORDER_STYLE (CORNER [TL TR BL BR])
NATIVE_STAMP_BORDER_STYLE (SIDES [L R T B])
NATIVE_STAMP_INNER_BORDER_STYLE (same modes/selectors)
NATIVE_STAMP_INNER_BORDER (padding-mm thickness-mm base-style colour)
```

Outer and inner modes/masks are independent and captured per stamp.
Save/restore includes both. SIDES changes only the mask, not the current
ordinary style. No selectors means no sides, but bare CORNER means all
corners and ignores that mask. Switching modes does not reset sides.
Duplicates are idempotent, not rejected. All selectors must be unquoted;
unknown selectors/modes, wrong-family selectors and selectors after
ordinary modes produce explicit source-line errors before writing output.

The four-argument inner command remains valid, now also accepting bare
CORNER. Selected corners use the separate style command. Dimension/base-style/
colour changes preserve side masks. A SIDES command following a previously
typed DASHED inner border inherits DASHED rather than introducing SOLID.
Inner cancellation/hairlines and NO_FRAME/outer BLANK behavior from #91
remain covered by the unchanged baseline tests.

Each selected ordinary side is separately stroked. Corner markers are one
move and two joined lines; their fixed 2 mm legs and point-based dash arrays
are mapped AlbumEasy command semantics, not new configurable defaults.
Shared drawing saves/restores stroke colour, width and dash state. Existing
SOLID-only streams retain the earlier closed-rectangle outer primitive;
after selector/dash modes, full-side drawing uses measured separate sides.
No physical bounds or image scaling is changed by border settings.

### Supplemental reference evidence

| Probe | AlbumEasy reference | Native proof | Finding |
|---|---|---|---|
| [corner-geometry.txt](corner-geometry.txt) | [corner-geometry.pdf](corner-geometry.pdf) | [corner-geometry-txt-native.pdf](corner-geometry-txt-native.pdf) | Outer widths 0.1/0.4 mm, inner 0.35 mm and hairline: correct centreline inset and joined corner legs |
| [selector-duplicates.txt](selector-duplicates.txt) | [selector-duplicates.pdf](selector-duplicates.pdf) | [selector-duplicates-txt-native.pdf](selector-duplicates-txt-native.pdf) | Repeated TL/BR and B/T selectors each draw once; four stamp strokes plus one page border |

Both were rendered with the same AlbumEasy command shown above, changing
only the input filename. Both exited 0 without input warnings. MuPDF trace
extraction exited 0 with its existing ICC-support warning. The main accepted
references were not regenerated or edited.

## Native development execution report

Runnable coverage: [native_pdf_stamp_border_styles.rs](../../../native_pdf_stamp_border_styles.rs).
Its PDF parser tracks graphics state and affine image transforms. It
associates each real stroked path with its stamp image and compares points
relative to that image centre, not merely translating each individual line.
This checks border-to-image positioning, inset, every joined corner vertex,
selected side identity, lengths, width, colour and dashes. Absolute
row/title positions are intentionally excluded. Tolerances: coordinates/
image dimensions <0.02 pt, stroke width <0.01 pt, RGB <0.001.

Main stroke counts including one page border remain 23/11 for `album`
and 37/29 for `inner-state`. Each main input has 12 image placements.
Model tests cover all 16 side masks and all nonempty corner masks/bare
all-corners, mode persistence and BLANK. Native repeated-selector PDFs
equal their non-repeated counterparts. Separate regression covers inherited
inner styles and mask persistence across explicit dimension updates.

Commands (repository root):

```sh
cargo test --test external_album_backend --test native_pdf_album --test native_pdf_albumeasy --test native_pdf_explicit_settings --test native_pdf_footer_import --test native_pdf_footer_number --test native_pdf_margin_text --test native_pdf_page_footer --test native_pdf_pagination --test native_pdf_pagination_proofs --test native_pdf_row_alignment --test native_pdf_stamp_background --test native_pdf_stamp_image --test native_pdf_stamp_inner_border --test native_pdf_stamp_border_styles --test native_pdf_stamp_items --test native_pdf_stamp_rows --test native_pdf_stamp_settings --test native_pdf_stamp_text
cargo test --test native_pdf_stamp_border_styles --test native_pdf_stamp_inner_border --test native_pdf_albumeasy --test native_pdf_stamp_settings
cargo test --test native_pdf_stamp_border_styles regenerate_border_style_proofs -- --ignored
cargo check --all-targets
cargo clippy --lib --test native_pdf_stamp_border_styles --test native_pdf_stamp_inner_border --test native_pdf_stamp_settings
cargo fmt --check
git diff --check
cargo build --example render_stampalbum
./target/debug/examples/render_stampalbum tests/fixtures/albumeasy/stamp-border-styles-v1/<input> <session>/styles-cli-<input>.pdf
pdfinfo <session>/styles-cli-<input>.pdf
pdfimages -list <session>/styles-cli-<input>.pdf
```

The initial broad gate passed 119 tests/19 ignored/zero failed (exit 0).
Precommit rerun on 2026-11-08 used the exact broad command above:
120 passed/19 ignored/zero failed (exit 0). Format/whitespace and all four
approved reference hashes passed again. Tests used temporary output
directories; no fixture or ingest input was moved, deleted or rewritten,
and no production DB/schema/rows were changed. No new errors or logic risks
were found; the print/Windows/full-language gaps below remain open.
After the inherited-style correction/test, the final broad gate passed
120/19 ignored/zero failed (exit 0), and the final affected gate passed
22/4 ignored/zero failed (exit 0). Proof regeneration passed 1/zero failed
(exit 0), creating eight native PDFs. All-target check and focused Clippy
exited 0; existing Review UI/extraction warnings and the pre-existing renderer
argument-count warning remain unrelated and untouched. Formatting/whitespace
checks passed. All six actual standalone dispatch invocations exited 0,
each producing two Letter pages and 12 real image placements; alpha masks
were not counted as stamps. Their six temporary CLI PDFs were inspected
then deleted. Windows uses `target/debug/examples/render_stampalbum.exe`;
that runtime was unavailable and remains unverified.

Denmark v5 is rerendered by the retained inner-border regression: both
unchanged supported TXT/raw sources render 16 pages and 41 images, with
numbered footers and zero deferral comments. No style/margin command was
added to Denmark or any accepted prior fixture.

For each main case and route, `pdftoppm -f 1 -l 2 -scale-to 1200 -png
<case>-<route>-native.pdf <session>/styles-<case>-<route>-native` generated
session-only rasters; native-language inputs use `<case>-native.pdf`.
`cmp` on corresponding TXT/raw page PNGs exited 0 for all four pairs.

### Error inventory / corrections during development

- Initial coupled test run exited 101: the generic style error lost the
  native inner command name. Added command context; retained error test passes.
- First new suite exited 101: quoted selectors were accepted by the
  existing native text helper. Added a local unquoted-selector guard, without
  changing unrelated native text handling.
- That run also found eight native images on page 1 instead of six.
  Explicitly documented/reserved omitted row-label space in the native
  document gap; all routes now have two pages/six images each.
- Duplicate selectors were initially rejected. The AlbumEasy supplemental
  probe disproved that assumption; mapped idempotent selection and added
  actual PDF regression.
- Final code inspection found SIDES following an existing typed DASHED
  inner border could introduce SOLID. Inherited the existing mode and
  added a dedicated runtime-PDF regression.
- `cargo fmt --check` initially returned 1 for new formatting; `cargo fmt`
  corrected only changed Rust files. No failures remain in the final gate.

Filesystem impact: added four main derivative inputs, six main native PDFs,
two supplemental TXT/reference/native PDF triples and the new test module.
Changed shared border state/model/parser/writer and only invalidated
unsupported-command sentinels in two prior tests; updated plan/matrix/README.
No earlier fixture, input, asset or proof was moved/deleted/changed.
The unique derivative helper was created outside the repository and removed
after generating inputs. Scratch logs/traces/PNGs stay session-only.
No dependencies, runtime TOML, DB, schema or ingest flow changed. No DB
opened; per-country rows/duplicates/inbox statistics are not applicable.

### Verdict and continuation boundary

- Startup/render: PASS, standalone native execution without AlbumEasy.
- Geometry/state integration: PASS, exit 0, six main inputs plus technical
  probes and retained regressions.
- Human native print/visual acceptance: NEEDS SIGNOFF.
- Windows runtime: UNVERIFIED; Rust path handling remains portable.
- Full language/whole-page layout parity: UNVERIFIED, explicitly outside
  this rectangular image-frame slice.

Next work must remain reference-first. Nonrectangular/rotated/text-only
stamp borders, very small boxes with corner legs and broader stamp-add/
text construction are still open. Preserve these references, existing
fixtures and three-input/Denmark gates. No production integration, PR, OKF
regeneration, commit or push without the corresponding maintainer approval.

Logic risks: independent mode/mask state, selector defaults, corner path
joining/anchor/length, save/restore of masks and corner choices, graphics-state
leakage, invalid/duplicate selectors and compatibility boundaries for shapes.
No native code before acceptance; no production integration, PR or OKF.
Commit/push requires separate maintainer instruction.

Verdict: **NEEDS_SIGNOFF** for AlbumEasy reference acceptance.
