# Denmark 1989: stamp-background native proof v2

This derivative preserves the [v1 fixture](../denmark-txt-native-v1/README.md)
and restores only its three `COLOR_STAMP_BACKGROUND` lines (original lines
49, 72 and 79). It reuses the unchanged v1 image assets via relative paths,
without duplicating or editing images or querying SQLite.

| Input | Direct native output |
|---|---|
| [Supported TXT](Denmark-1989-1989-supported.txt) | [Native PDF](Denmark-1989-1989-native.pdf) |
| [Identical raw `.stampalbum`](Denmark-1989-1989-raw.stampalbum) | [Raw native PDF](Denmark-1989-1989-raw-native.pdf) |

Both outputs contain 16 pages and 41 image placements. The v2 page-6 preview
was inspected: black fills are restored behind the mixed-height sheet and
two smaller stamp images; captions remain below frames. Existing caption
column overlap, native/reference spacing differences, footer omissions and
human print-review gaps remain as documented in v1. This does not certify
untouched-source compatibility or exact visual parity.

Five original occurrences remain explicitly commented:

| Line | Command |
|---:|---|
| 42 | `COLOR_ALBUM_FOOTER` |
| 45 | `COLOR_ALBUM_MARGIN_TXT` |
| 52 | `ALBUM_PAGES_FOOTER_NUMBER` |
| 68 | `ALBUM_STAMP_SETTINGS_SAVE` |
| 83 | `ALBUM_STAMP_SETTINGS_RESTORE` |

The supported sources are not the immutable originals. See v1's source
hashes, image manifest and historical PDF for provenance. The original
source and all committed v1 assets/PDFs are unchanged.

## Reproduction and execution report

From the repository root:

```sh
cargo test --test native_pdf_stamp_background regenerate_denmark_background_proofs -- --ignored --nocapture
```

Exit 0: one passed; each input rendered 16 pages/41 image draws, five retained
deferrals, three background commands restored. The generator compares the v1
source bytes before/after and never writes into v1. This opt-in test requires
only committed fixtures, not the original Linux album directory.

Alternatively render either input with `cargo run --example render_stampalbum --`
followed by its input and output paths. Both file formats use the same native
compatibility backend without AlbumEasy. Windows can use the same Cargo
commands/portable relative paths; Windows execution remains unverified.

Created this directory with two inputs, two native PDFs and README; source
images are reused, not copied. No files were deleted, moved or relocated.
No SQLite database was opened or modified: target paths, schema checks,
insert/update/skip counts and duplicates are not applicable. No ingestion or
inbox flow ran. PDF previews stay in the session workspace.

The [background reference fixture](../stamp-background-colours-v1/README.md)
contains the new accepted AlbumEasy first pass, native-language third input
case, complete regression commands/results and failure inventory.
Native human visual/actual-size print acceptance remains pending.
