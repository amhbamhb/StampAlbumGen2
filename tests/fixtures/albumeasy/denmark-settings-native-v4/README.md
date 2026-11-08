# Denmark 1989: stamp-settings native proof v4

This derivative restores only `ALBUM_STAMP_SETTINGS_SAVE` (original line 68)
and `ALBUM_STAMP_SETTINGS_RESTORE` (83) from
[v3](../denmark-footer-native-v3/README.md). Single-snapshot stamp state now
restores after the temporary sheet settings rather than leaking into later
items. Existing v1-v3 sources/assets/PDFs remain untouched.

| Input | Direct native output |
|---|---|
| [Supported TXT](Denmark-1989-1989-supported.txt) | [Native PDF](Denmark-1989-1989-native.pdf) |
| [Identical raw `.stampalbum`](Denmark-1989-1989-raw.stampalbum) | [Raw-native PDF](Denmark-1989-1989-raw-native.pdf) |

Both outputs preserve 16 pages and 41 visible images, with every
`Page N of 16` footer. Only original line 45, `COLOR_ALBUM_MARGIN_TXT`,
remains explicitly commented. The untouched original TXT still fails on
that unsupported command; full source compatibility is not yet certified.
Dimensions remain 216.0 x 279.4 mm, not rounded to exact Letter.

## Reproduction / execution report

From repository root on Windows or Linux:

```sh
cargo test --test native_pdf_stamp_settings regenerate_denmark_settings_proofs -- --ignored --nocapture
```

Exit 0: 1 passed. Each input verifies 16 pages, 41 images, every numbered
footer and one remaining deferral; v3 source bytes are checked unchanged.
Source image assets are reused via relative paths into the original
[v1 bundle](../denmark-txt-native-v1/README.md), not duplicated or resized.

```sh
pdftoppm -f 6 -l 6 -scale-to 1200 -singlefile -png tests/fixtures/albumeasy/denmark-settings-native-v4/Denmark-1989-1989-native.pdf <session-preview>
```

Page 6 was inspected: the black sheet/background context, smaller real stamp
images and blue numbered footer remain visible. Existing dense caption
overlap, historical reference differences and native layout/72-dpi image
scaling limitations remain open; no pixel parity or human print acceptance
is claimed. No errors occurred in regeneration.

Filesystem impact: created this directory, two inputs, two native PDFs and
README. No earlier references, images or original inputs were modified,
moved, deleted or relocated. Preview PNGs/logs remain session-only. Database
impact: none, no DB opened/modified; schema/row counts/duplicate detection
not applicable. No ingestion/inbox flow ran. Windows runtime remains
unverified; portable Rust and relative paths are used.

The [save/restore report](../stamp-settings-save-restore-v1/README.md)
records accepted AlbumEasy first-pass evidence, strict negative probes,
all three native input routes and remaining native style/state coverage gaps.
No production integration, PR, OKF regeneration or commit/push is included
without subsequent maintainer instruction.
