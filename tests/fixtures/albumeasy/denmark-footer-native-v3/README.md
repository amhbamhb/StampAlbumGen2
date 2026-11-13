# Denmark 1989: footer native proof v3

This derivative preserves [v2](../denmark-background-native-v2/README.md)
and restores only `COLOR_ALBUM_FOOTER` (original line 42) and
`ALBUM_PAGES_FOOTER_NUMBER` (line 52). Images are unchanged and reused via
relative paths into the [v1 bundle](../denmark-txt-native-v1/README.md).
There was no database lookup or copying/resizing of images.

| Input | Native output |
|---|---|
| [Supported TXT](Denmark-1989-1989-supported.txt) | [Native PDF](Denmark-1989-1989-native.pdf) |
| [Identical raw `.stampalbum`](Denmark-1989-1989-raw.stampalbum) | [Raw native PDF](Denmark-1989-1989-raw-native.pdf) |

Each output has 16 pages, 41 image placements and `Page N of 16` on every
page. Source dimensions remain 216.0 x 279.4 mm (612.283 x 792 points);
do not silently round the historical width to exact US Letter. Five v2
unsupported occurrences have become three:

| Original line | Still explicitly commented |
|---:|---|
| 45 | `COLOR_ALBUM_MARGIN_TXT` |
| 68 | `ALBUM_STAMP_SETTINGS_SAVE` |
| 83 | `ALBUM_STAMP_SETTINGS_RESTORE` |

Only these adapted copies contain `# NATIVE-TODO:` comments. The original
source still fails on unsupported commands; it is not certified unchanged.
Historical v1/v2 inputs, PDFs and assets remain untouched.

## Reproduction and execution report

From the repository root, on Windows or Linux:

```sh
cargo test --test native_pdf_footer_import regenerate_denmark_footer_proofs -- --ignored --nocapture
```

Exit 0: 1 passed. The generator checks 16 pages, 41 image draws, every
numbered label, exactly three remaining deferrals and unchanged v2 source
bytes. It creates this sibling directory, two inputs and two native PDFs,
with this README added separately. No files are moved, deleted or relocated.
No source images are changed. The TXT/raw routes share the native backend;
no AlbumEasy executable is invoked.

Page 6 was rasterized with:

```sh
pdftoppm -f 6 -l 6 -scale-to 1200 -singlefile -png tests/fixtures/albumeasy/denmark-footer-native-v3/Denmark-1989-1989-native.pdf <session-preview>
pdfinfo tests/fixtures/albumeasy/denmark-footer-native-v3/Denmark-1989-1989-native.pdf
pdftotext -f 6 -l 6 tests/fixtures/albumeasy/denmark-footer-native-v3/Denmark-1989-1989-native.pdf -
```

Screen inspection shows the blue footer below the border and the existing
black fills behind the mixed-height images. Dense caption overlap, historical
PDF/source differences, 72-dpi compatibility image scaling and full native
visual/print review remain open. Each PDF is approximately 44.7 MB; no image
optimization was attempted. No errors occurred in this regeneration.

DB impact: no DB opened/modified; target paths, schema compatibility,
inserted/updated/skipped rows and duplicate detection are not applicable.
No ingestion/inbox flow ran. Previews and logs are session-only.
Windows runtime and human Actual-size print validation remain unverified.

The [footer fixture report](../footer-numbering-import-v1/README.md)
contains the accepted AlbumEasy first pass, all three native input routes,
strict-error tests and focused regression results. Year/section-local
numbering is a separate future #88 item; production integration, OKF
regeneration and PR creation remain deferred.
