# United States stamp physical-sizing accuracy check

Date: 2026-11-04 local / 2026-11-05 UTC.
Outcome: analysis completed; no production sizing, album files, or database rows changed.

## Reuse this analysis in future layout work

This is the project's preserved physical-sizing evidence, shared by existing
AlbumEasy generation and the native PDF/layout work. Read it before changing
size inference, mounting-space calculations, image fitting, or row packing.
The [native PDF roadmap](NATIVE_PDF_IMPLEMENTATION_PLAN.md) tracks implementation;
this report records the agreed requirements, completed experiment, and limits.
The requirements below are agreed; the candidate estimation methods remain
experimental and have not been implemented in production.

Do not repeat this baseline merely because a new layout phase starts. Reuse
the recorded results and full per-record ledgers. Re-evaluate only for changed
inputs/configuration, a new estimator, or additional validation coverage, and
record the difference from this baseline rather than replacing it.
The database SHA-256 and exact method/settings below identify this run.
Reference labels are not confirmed physical measurements, and no early-era
accuracy claim is supported by this sample.

## Agreed physical-sizing requirements

- Physical dimensions mean outer perforation tip to outer perforation tip.
- Keep known physical sizes; move an item to the next row when it does not fit
  the remaining row. Normal row wrapping needs no warning.
- Mounting clearance, frame dimensions, illustration scaling, and text are
  separate layout concerns. Do not equate illustration size with physical size.
- Infer missing dimensions automatically using comparable known items and
  suitable references. Fine-tuning is after viewing or printing the output.
- The present check evaluates physical-size estimates only, not row packing,
  frame/text geometry, printed illustrations, or physical print measurements.

### Approved clearance compatibility contract (2026-11-04)

Preserve the existing total-addition settings; use explicit per-edge geometry
internally. Frame width is physical width + `extra_width_mm` +
`stamp_box_adjust`; frame height uses `extra_height_mm` in the same way.
A 22x25 mm physical stamp with 6 mm total additions becomes a 28x31 mm frame,
with 3 mm per edge when centred. Achieving 6 mm per edge would require
12 mm total additions, not reinterpreting existing settings.

The [native PDF roadmap](NATIVE_PDF_IMPLEMENTATION_PLAN.md) records the #77
known-size geometry/proof slice and its validation. It does not implement the
experimental unknown-size estimator or change legacy generation behavior.

## Inputs and coverage

Album: `/home/ray/AlbumGen2/Albums/United-States`.

| Inventory | Count |
|---|---:|
| Stamp records | 331 |
| Positive, finite Colnect width/height pairs | 248 |
| Records without a complete known size | 83 |
| Current image files associated with records | 315 |
| Known-size records with a current image | 248 |
| Blank image paths | 16 |
| Nonblank paths with missing/unreadable files | 0 |
| Stored pixel-dimension pairs | 26 |
| Stored pixel pairs with a corresponding current image | 17 |
| Stored pairs that disagree with that image's dimensions | 17 |
| Known-size identity groups after shared-image/Colnect-ID grouping | 246 |
| Known-size records with over 15% multiplicative image/physical aspect discrepancy | 11 |

Known-size format distribution: 188 Stamp, 50 Se-tenant, 6 Booklet,
4 Booklet Pane. The album contains only years 1959, 1960, 1962, 1970,
1971, and 1981. It cannot validate the early 1840-1900 sizing behavior or
generalize across all modern eras.

The database also has 628 `colnect_stamps` rows. They were counted but NOT used
as an extra training pool; avoiding unresolved cross-table identity/variant
matches keeps the present evaluation bounded.

## Method

Stored Colnect dimensions serve as provisional reference labels, not
independently verified physical measurements. For each of the 248 known-size
records, hide its physical dimensions from the estimator and compare the
prediction against those labels.

Five approaches were evaluated:

1. **DPI baseline:** mirror the primary generator's image-sizing prepass with
   stored DB pixels preferred over current file-header pixels, 96 DPI, and
   proportional reduction to a maximum height of 25 mm.
2. **Embedded-reference sensitivity:** same fallback calculation but use
   19/120 mm per horizontal pixel and 25/140 mm per vertical pixel instead of
   DPI; retain the 25 mm height cap. This is a hypothetical no-DPI alternative,
   not the current runtime setting.
3. **Series peers:** match country, series, format, emission, and image shape;
   use known sizes from comparable peers.
4. **Hierarchical peers:** prefer the same-series peers; otherwise match
   country, format, emission, image shape, and issue year within +/-10 years.
5. **Strict peers:** the hierarchical method, requiring at least three peers
   whose width and height are each within 2 mm of their median.

Peer methods use current image headers for shape, not stored pixel dimensions.
They require nonempty format/emission and do not infer missing metadata.
Candidate image aspect ratios must be within a multiplicative factor of 1.15
of the target image aspect ratio. A known donor's image aspect must also be
within that factor of its own recorded physical aspect ratio. Select up to
five distinct identity groups, ordered by closest image shape, then closest
year, then ID. Predict the median width and median height.

These are exploratory, preset heuristics, not a trained or approved production
algorithm. No threshold tuning or correction of labels was performed.

### Identity holdout

Exclude every record connected to the target through either identical image
SHA-256 or the same normalized Colnect ID. Deduplicate training peers by those
identity groups. This prevents exact shared-image/ID copies from providing
the answer; other issue-related records can remain.

### Whole-series holdout

Additionally exclude all records in the target series. This is a harder test
of estimation when no known same-series reference is available.

### What the baseline means

The inspected global configuration in `/home/ray/AlbumGen2/config.toml` uses
96 DPI, `fit_box`, a 25 mm target height, and 19x25 mm reference dimensions.
The album-local configuration has no overrides for these sizing keys.
The current working repository's global configuration has the same settings.
This does not establish which settings produced every historical saved PDF.

The actual generator prioritizes known Colnect dimensions before using this
fallback. The baseline intentionally HID those known sizes. Its poor results
do not mean the application currently mis-sizes all 248 known-size records.
This is a numerical reproduction of the primary prepass, not an end-to-end
execution of all generator/layout branches or the native PDF proof.

Relevant implementation at the time of analysis:
[primary sizing prepass, calibration, and pixel-based series averages](../../src/album_globals/generator.rs),
[embedded reference](../../src/image_utils.rs).

## Identity-holdout results

"Within 1 mm" means BOTH predicted dimensions are within 1 mm of their
reference dimensions. This is an analysis threshold, not an approved mounting
tolerance. Rates below are conditional on making a prediction; no-prediction
cases are shown separately and are not successes.

| Method | Predicted / 248 | Both dimensions within 1 mm | Within 2 mm | Mean absolute width / height error | Worst error on either axis |
|---|---:|---:|---:|---:|---:|
| DPI baseline | 248 | 72/248 (29.0%) | 106/248 | 10.08 / 10.07 mm | 77.86 mm |
| Embedded reference, no DPI | 248 | 0/248 (0.0%) | 16/248 | 12.85 / 10.46 mm | 83.20 mm |
| Same-series peers only | 158 | 144/158 (91.1%) | 147/158 | 0.76 / 0.85 mm | 40.00 mm |
| Hierarchical peers | 229 | 196/229 (85.6%) | 201/229 | 1.27 / 1.49 mm | 40.00 mm |
| Strict hierarchical peers | 150 | 138/150 (92.0%) | 140/150 | 1.13 / 1.25 mm | 40.00 mm |

Across ALL 248 targets, including non-predictions, hierarchical peers achieve
196/248 (79.0%) within 1 mm; strict peers achieve 138/248 (55.6%).
Stricter peer agreement improves the conditional pass rate but reduces
coverage and does not eliminate large errors.

The DPI baseline underestimates at least one dimension by more than 1 mm in
152 cases. Hierarchical peers do so in 20 of their 229 predictions. Errors
from missing labels, mismatched images, and item variants remain in these
figures rather than being silently discarded.

### Hierarchical peer results by format

| Format | Known targets | Predictions | Within 1 mm | Mean absolute width / height error |
|---|---:|---:|---:|---:|
| Stamp | 188 | 184 | 161/184 (87.5%) | 1.16 / 1.27 mm |
| Se-tenant | 50 | 42 | 32/42 (76.2%) | 1.81 / 2.57 mm |
| Booklet | 6 | 3 | 3/3 | 0.00 / 0.00 mm |
| Booklet Pane | 4 | 0 | Not evaluated | Not available |

The three successful booklet predictions are far too few to establish
reliability. No booklet-pane prediction was available.

## Whole-series-holdout results

| Method | Predictions | Within 1 mm | Within 2 mm | Mean absolute width / height error |
|---|---:|---:|---:|---:|
| Hierarchical peers | 187/248 | 127/187 (67.9%) | 136/187 | 3.02 / 3.06 mm |
| Strict hierarchical peers | 110/248 | 88/110 (80.0%) | 94/110 | 2.31 / 2.25 mm |

Same-series-only peers necessarily make no predictions in this test.
The non-learning pixel baselines are unchanged. Whole-series performance
confirms that same-series references carry substantial useful information,
but that inference is less reliable for new or unsupported series.

## Important failure examples

| Record / SG | Stored physical size | Hierarchical estimate | Finding |
|---|---:|---:|---|
| 228 / 1924a | 50x80 mm | 25x40 mm | Image visibly contains a block of four, but stored format is `Stamp`; name explicitly says "Block of 4". Shape alone cannot distinguish a 2x2 block from a single stamp with the same aspect ratio. |
| 314 / 1414e | 25x40 mm | See per-record CSV | Image is a portrait stamp PLUS explanatory images and text on a landscape canvas. File aspect is not stamp aspect. |
| 366 / 1338Fi | 50x22 mm | 22x50 mm | Image visibly shows a vertical pair, while stored dimensions are landscape. Could involve rotated dimensions, mismatched item metadata, or another convention; no automatic correction is justified by this check alone. |
| 148 / 1874a | 50x80 mm | 25x40 mm | Image contains an error/variant illustration with an annotation and large blank area; the stored dimensions have not been independently confirmed. |
| 303 / 1414d | 75x40 mm | 40x25 mm | Another annotated/composite reference image; known metadata and full image dimensions must not be assumed to describe the same physical rectangle. |

These observations support keeping catalog identity, parent/variant
relationships, item multiplicity (single/pair/block/pane/booklet), and
whether an image is a usable full-item scan in the reference design.
`Stamp` versus `Se-tenant` alone is not sufficient.

## Applying the exploratory peer policy to genuinely unknown records

Of the 83 records without a complete known size, the hierarchical candidate
can produce 31 estimates; 52 have no eligible evidence under its rules.
These estimates have NO known physical truth in this check and must not be
called correct or counted in the accuracy results.

Forty unknown records have neither format nor emission; two more lack format.
This is an evidence/metadata coverage problem, not something image pixels
alone can solve. The user's desired future reference collection may help,
provided its identity, scale, and item type are recorded.

## Recommendations before implementation

1. Preserve known physical dimensions and mounting clearance independently of
   illustration-fit rules. A universal 25 mm height cap is unsuitable for
   estimating full-size mounting spaces for large items.
2. Prefer known references from the same issue/series, matched by physical item
   type and orientation; use broader country/era/format/emission matches only
   as explicitly lower-confidence fallbacks.
3. Include verified references with catalog identity, physical dimensions,
   item multiplicity, provenance, source/crop identity, and suitable image
   samples. Do not learn absolute mm-per-pixel from unrelated web-image sizes.
4. Detect or exclude annotated/composite images from geometric inference.
   Do not silently rotate or overwrite known dimensions when shapes disagree.
5. Account for stored pixel measurements that no longer describe the selected
   image. Do not silently replace the database values during this analysis.
6. Record the source and uncertainty of automatic estimates for subsequent
   output/print review; do not require manual entry or warn on normal row wrap.
7. Test a genuine early-era sample and more booklet/pane/block examples before
   claiming general sizing accuracy. Review which reference dimensions and
   item identities are physically confirmed.

No candidate is approved as a production replacement by this report.

## Execution and integrity record

Exact analysis command:

```text
python3 '/media/ray/320G-2020-06-Program/Rust/Various Scripts/us-stamp-sizing-20261105T031956Z.py'
```

Analysis started 2026-11-05T03:21:16Z; ended 2026-11-05T03:21:21Z.
Exit status: success.

Database opened using SQLite URI `mode=ro` and `PRAGMA query_only=ON`.
Relevant schema columns were present for ID, series/year, country, dimensions,
image path, format/emission, and Colnect identity. Reference dimensions had
to parse as positive finite numbers.

| Target | Inserted | Updated | Deleted | Operational duplicate handling |
|---|---:|---:|---:|---|
| United-States.db | 0 | 0 | 0 | No ingest. Held-out identity groups excluded; training peers deduplicated. |

Album filesystem: 517 files inventoried before/after; file sizes and
nanosecond modification times unchanged. No moved/deleted/created album
files or directories. Database SHA-256 before and after:

```text
0e06f8b7f5ec92d8e220a6324018459e31679341360fbf203893e44f7c663e55
```

Originally created outside the album/repository, now preserved here for future
work (no album databases or images are copied into the repository):

- This report.
- [Machine-readable analysis](../testing/us-sizing-20261105T031956Z.json).
- [Per-record prediction/error ledger](../testing/us-sizing-20261105T031956Z.csv):
  2,480 rows = 248 targets x 5 methods x 2 holdout modes.
- One temporary analysis script in the designated external script directory,
  removed after completing verification.

No runtime failures, image read failures, or missing nonblank image paths.
No application tests/build were needed or run: production code was not
modified. An independent CSV check verified unique target/model/holdout
rows, peer format/emission matching, whole-series exclusions, all coverage
counts, and every reported within-1-mm count. `git diff --check` passed;
pre-existing proof and manual-autosave worktree changes were not modified.

Logic risks are the stored-pixel/image mismatch, composite images,
same-shape single/block ambiguity, sparse metadata, provisional reference
labels, and narrow year coverage described above.
