# Native stamp-text width policies

Open [all-cases.pdf](all-cases.pdf). Print **US Letter, Actual size / 100%**,
with Fit/Shrink disabled. All text is 10 pt.

| Page | Policy / PDF | TXT comparison | Native policy fragment | Items per row |
|---|---|---|---|---|
| 1 | [wrap_to_frame](wrap_to_frame.pdf) | [TXT](wrap_to_frame.txt) | [TOML](wrap_to_frame.toml) | 5, 1 |
| 2 | [expand_footprint, 200% cap](expand_footprint.pdf) | [TXT](expand_footprint.txt) | [TOML](expand_footprint.toml) | 3, 3 |
| 3 | [Expanded, 120% cap](expanded-percent-120.pdf) | [TXT](expanded-percent-120.txt) | [TOML](expanded-percent-120.toml) | 2 |
| 4 | [Expanded, 30 mm cap](expanded-mm-30.pdf) | [TXT](expanded-mm-30.txt) | [TOML](expanded-mm-30.toml) | 2 |

The first two pages have the same six 22x25 mm physical stamps and 28x31 mm mounting
frames. Green outlines show total reserved footprints; black outlines show
frames; blue outlines show physical bounds. Horizontal padding is 2 mm per
side, vertical padding 1 mm per edge, and frame/text gaps 3 mm. Horizontal
and vertical row spacing are both 3 mm, from the template.

Wrapping constrains each line to frame width and reserves additional height.
Expanded mode preserves explicit lines, allows width up to the configured cap,
then wraps longer lines, reserving their complete width **and** height.
Neither changes physical/mounting dimensions or font size.
Frames are horizontally centered within top-aligned complete footprints.
Footprint padding is separate from mounting clearance and page margins.

Pages 3 and 4 use two copies of a four-line caption with long second/third
lines. For their 28 mm frames, 120% gives a 33.6 mm text cap (at most
37.6 mm footprint with padding); the 30 mm cap gives at most 34 mm footprint.
Limits are maximum widths, not forced widths: word wrapping can leave both
examples narrower than their caps. Regression coverage uses a 31.75 mm
Courier string to prove it fits the percentage cap but wraps at 30 mm.
Explicit newlines/blank lines are preserved; wrapping normalizes repeated spaces.
There is no fixed line-count limit (a 30-line caption is tested), but total
height still must fit this single-page proof. Oversized content errors rather
than truncating, shrinking fonts or silently starting a new page.

```toml
[native_pdf.stamp_text]
enabled = true
width_policy = "expand_footprint"
expanded_width_limit = "percent" # Or "mm".
expanded_width_value = 120.0     # Or 30.0 for a 30 mm cap.
horizontal_padding_mm = 2.0
vertical_padding_mm = 1.0
text_gap_mm = 3.0
```

Percentage uses mounting-frame width, not physical stamp/image width.
Millimetre caps below an item's frame are rejected. Both cap keys are required
and validated if the table exists, including when disabled or frame-wrapping.

The complete explanation for each key is in
[config_template.toml](../../../../config_template.toml), mirrored in
[config_full_example.toml](../../../../config_full_example.toml). Comments
immediately before keys become field help in the existing configuration UI.
The TOML fragments here must be merged into a full native configuration;
they are not standalone proof configs.

**Current scope:** isolated native layout/proof, opt-in and disabled in the
templates. No production backend or legacy generation behavior is switched.
The policy does not yet include stamp images, inside-frame text, automatic
pagination or production AlbumEasy export.

## Compatibility, not silent omission

Native policy settings are not AlbumEasy commands. The TXT files explicitly
identify adaptation and preserve every actual text block using page-text
commands, followed by unchanged frame rectangles. This is deliberately **not**
native position, row, page-break or font/wrapping parity. They have not been
reference-rendered. Native settings stay in TOML, never only in TXT comments.
Future production export must report feature/page/source compatibility
findings rather than silently dropping native behavior.

[expected.json](expected.json) records every item/frame/physical bound and
every text line's measured width and position.
[verification.txt](verification.txt) records independent PDF trace results.

## Reproduce (Windows or Linux)

From the repository root:

```text
cargo test --test native_pdf_stamp_text
cargo test --test native_pdf_stamp_text regenerate_letter_text_fixtures -- --ignored --exact
mutool merge -o tests/fixtures/albumeasy/stamp-text-v1/all-cases.pdf tests/fixtures/albumeasy/stamp-text-v1/wrap_to_frame.pdf tests/fixtures/albumeasy/stamp-text-v1/expand_footprint.pdf tests/fixtures/albumeasy/stamp-text-v1/expanded-percent-120.pdf tests/fixtures/albumeasy/stamp-text-v1/expanded-mm-30.pdf
```

Independent trace: `mutool draw -F trace -o TRACE_PATH` followed by the combined
PDF path. Choose a trace path outside the repository and recompute checks after
regeneration. Normal tests do not overwrite these saved fixtures.

## Development execution report

Commands:

```text
cargo test --test native_pdf_stamp_text
cargo test --lib native_stamp_text_template_help_reaches_each_editor_field
cargo test --test native_pdf_stamp_text regenerate_letter_text_fixtures -- --ignored --exact
cargo test --test 'native_pdf_*'
cargo test --lib config_editor::tests
cargo clippy --test native_pdf_stamp_text
cargo check --lib --target x86_64-pc-windows-gnu
rustfmt --edition 2021 --check src/native_pdf/stamp_text.rs src/native_pdf/text.rs src/native_pdf/config.rs src/native_pdf/renderer.rs tests/native_pdf_stamp_text.rs
git diff --check
```

Base slice: 7 text tests passed; the follow-up adds capped-expansion regressions.
The opt-in fixture test passes explicitly.
Cap follow-up: full native regression 113 passed, 19 ignored, 0 failed.
After adding the distinct 31.75 mm wrap-boundary regression, final focused
suite: 10 text tests passed, 1 opt-in generator ignored, 0 failed.
Final commit-gate rerun: 114 native tests passed, 19 ignored, 0 failed;
all 3 config-editor tests passed. Format and whitespace checks passed.
Config-editor regression: 3 passed, 0 failed.
Clippy/format and Windows GNU library cross-check pass. Existing unrelated
unused/dead-code/PDF-import warnings remain; Windows reports
`winres compile failed: No such file or directory (os error 2)`.
Windows runtime/resource embedding is not tested on this Linux host.

Independent MuPDF checks verify Letter media size, every reserved/frame/physical
rectangle, complete stamp text and every line's left/baseline position within
0.004 mm. Actual inter-item/row gaps match within 0.006 mm, accounting for
three PDF operands rounded to 0.01 pt. Both pages were rasterized at 120 DPI
and visually inspected; the two new capped long-text pages were also inspected
at 120 DPI. All four pages pass independent bounds/text/gap checks and configured
cap-plus-padding checks. MuPDF emits `warning: ICC support is not available`.

Initial expected red errors: unresolved `native_pdf::stamp_text` and missing
`NativePdfConfig.stamp_text`. A config test initially failed
`assertion failed: config.stamp_text.is_some()` because its string replacement
assumed no comments before `enabled`; the test now modifies parsed TOML.
An independent proof check initially assumed a 4 mm vertical gap; the actual
template specifies 3 mm. The fixture ledger/TXT now explicitly record 3 mm
and the final rendered gaps pass.

Cap follow-up initial red: unresolved `ExpandedWidthLimit` and constructor
argument-count errors before implementation. The overflow fixture now uses
10,000 characters, ensuring it exceeds page height after wrapping rather than
assuming overwide text must fail. No production behavior was changed.

Filesystem impact: created this directory with four PDF/TXT/TOML sets, combined
PDF, expected geometry and verification. Scratch tests use temporary directories
cleaned on completion. No album inputs moved/deleted, no database writes/schema
changes/ingest, no OKF regeneration, and unrelated manual autosaves untouched.

Logic risks / limits: compatibility-oriented wrapping is not certified
AlbumEasy behavior; adapted TXT is not parity evidence. Physical printing,
automatic pagination, production export findings, custom-font/Unicode shaping,
inside-frame text and images remain deferred.
